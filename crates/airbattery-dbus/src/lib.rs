#![allow(missing_docs)]
//! Versioned Linux session D-Bus contract for the GNOME Shell companion.

use std::{sync::Arc, time::Duration};

use serde::{Deserialize, Serialize};
use shared_models::BluetoothAudioDevice;
use thiserror::Error;
use time::OffsetDateTime;
use tokio::{
    sync::{RwLock, mpsc, oneshot},
    time::timeout,
};
use zbus::{Connection, connection::Builder, object_server::SignalEmitter};

/// Well-known session-bus name owned by the `AirBattery` desktop process.
pub const BUS_NAME: &str = "io.github.airbattery.Service";
/// Object path exported by the `AirBattery` desktop process.
pub const OBJECT_PATH: &str = "/io/github/airbattery/Service";
/// Versioned D-Bus interface consumed by the GNOME extension.
pub const INTERFACE_NAME: &str = "io.github.airbattery.Service1";
/// Current JSON envelope schema version.
pub const IPC_SCHEMA_VERSION: u16 = 1;
const REQUEST_TIMEOUT: Duration = Duration::from_secs(10);

/// Bluetooth backend health included in shell snapshots.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackendSnapshot {
    /// Stable platform name such as `linux` or `windows`.
    pub platform: String,
    /// Whether the native Bluetooth backend is currently available.
    pub available: bool,
    /// User-safe adapter display name when available.
    pub adapter_name: Option<String>,
    /// Adapter power state when exposed by the platform.
    pub powered: Option<bool>,
    /// Whether bounded discovery is active.
    pub discovering: Option<bool>,
    /// Sanitized human-readable backend status.
    pub detail: String,
}

/// Complete versioned JSON snapshot transported over D-Bus.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ServiceSnapshot {
    /// JSON contract version.
    pub schema_version: u16,
    /// Generation time, serialized as RFC 3339, or `None` before the first refresh.
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub generated_at: Option<OffsetDateTime>,
    /// Current native backend health.
    pub backend: Option<BackendSnapshot>,
    /// Privacy-safe normalized devices.
    pub devices: Vec<BluetoothAudioDevice>,
    /// Preferred privacy-safe device identifier from application settings.
    pub preferred_device_id: Option<String>,
}

impl ServiceSnapshot {
    /// Creates a snapshot using the current schema version.
    #[must_use]
    pub fn new(
        generated_at: OffsetDateTime,
        backend: Option<BackendSnapshot>,
        devices: Vec<BluetoothAudioDevice>,
        preferred_device_id: Option<String>,
    ) -> Self {
        Self {
            schema_version: IPC_SCHEMA_VERSION,
            generated_at: Some(generated_at),
            backend,
            devices,
            preferred_device_id,
        }
    }

    /// Creates an unavailable snapshot before the native backend starts.
    #[must_use]
    pub const fn unavailable() -> Self {
        Self {
            schema_version: IPC_SCHEMA_VERSION,
            generated_at: None,
            backend: None,
            devices: Vec::new(),
            preferred_device_id: None,
        }
    }

    /// Serializes this snapshot for D-Bus transport.
    ///
    /// # Errors
    ///
    /// Returns [`SnapshotError`] when the snapshot cannot be serialized.
    pub fn to_json(&self) -> Result<String, SnapshotError> {
        Ok(serde_json::to_string(self)?)
    }

    /// Parses a versioned snapshot and rejects unknown schemas.
    ///
    /// # Errors
    ///
    /// Returns [`SnapshotError`] when `input` is invalid JSON or contains an
    /// unsupported snapshot schema version.
    pub fn from_json(input: &str) -> Result<Self, SnapshotError> {
        let snapshot: Self = serde_json::from_str(input)?;
        if snapshot.schema_version != IPC_SCHEMA_VERSION {
            return Err(SnapshotError::UnsupportedSchema(snapshot.schema_version));
        }
        Ok(snapshot)
    }
}

/// Errors while validating the JSON snapshot contract.
#[derive(Debug, Error)]
pub enum SnapshotError {
    /// JSON could not be encoded or decoded.
    #[error("snapshot JSON is invalid: {0}")]
    Json(#[from] serde_json::Error),
    /// A peer supplied a newer or otherwise unsupported schema.
    #[error("snapshot schema version {0} is unsupported")]
    UnsupportedSchema(u16),
}

/// Errors while creating or publishing the session D-Bus service.
#[derive(Debug, Error)]
pub enum ServiceError {
    /// D-Bus connection, registration, or signal delivery failed.
    #[error("D-Bus operation failed: {0}")]
    Dbus(#[from] zbus::Error),
    /// Snapshot serialization failed.
    #[error(transparent)]
    Snapshot(#[from] SnapshotError),
}

/// Request emitted by a D-Bus method and fulfilled by the desktop application.
#[derive(Debug)]
pub enum ServiceRequest {
    /// Run one bounded native refresh and return the resulting snapshot JSON.
    Refresh {
        /// Completion channel for the method response.
        response: oneshot::Sender<Result<String, String>>,
    },
    /// Open the desktop application on its settings surface.
    OpenSettings {
        /// Completion channel for the method response.
        response: oneshot::Sender<Result<(), String>>,
    },
    /// Show and focus the main desktop application window.
    ShowMainWindow {
        /// Completion channel for the method response.
        response: oneshot::Sender<Result<(), String>>,
    },
}

#[derive(Debug)]
struct AirBatteryInterface {
    snapshot_json: Arc<RwLock<String>>,
    requests: mpsc::Sender<ServiceRequest>,
}

#[zbus::interface(name = "io.github.airbattery.Service1")]
impl AirBatteryInterface {
    /// Returns the latest serialized service snapshot.
    async fn get_snapshot(&self) -> String {
        self.snapshot_json.read().await.clone()
    }

    /// Requests a bounded Bluetooth refresh.
    ///
    /// # Errors
    ///
    /// Returns a D-Bus error when the desktop request cannot be queued or
    /// its response channel closes before completion.
    async fn refresh(&self) -> zbus::fdo::Result<String> {
        let (response, receiver) = oneshot::channel();
        self.send_request(ServiceRequest::Refresh { response })?;
        receive_response(receiver).await
    }

    /// Opens the `AirBattery` settings window.
    ///
    /// # Errors
    ///
    /// Returns a D-Bus error when the desktop request cannot be queued or
    /// its response channel closes before completion.
    async fn open_settings(&self) -> zbus::fdo::Result<()> {
        let (response, receiver) = oneshot::channel();
        self.send_request(ServiceRequest::OpenSettings { response })?;
        receive_response(receiver).await
    }

    /// Shows and focuses the main `AirBattery` window.
    ///
    /// # Errors
    ///
    /// Returns a D-Bus error when the desktop request cannot be queued or
    /// its response channel closes before completion.
    async fn show_main_window(&self) -> zbus::fdo::Result<()> {
        let (response, receiver) = oneshot::channel();
        self.send_request(ServiceRequest::ShowMainWindow { response })?;
        receive_response(receiver).await
    }

    /// Signals that the serialized service snapshot has changed.
    ///
    /// # Errors
    ///
    /// Returns a D-Bus error when the signal cannot be emitted.
    #[zbus(signal)]
    async fn snapshot_changed(
        signal_emitter: &SignalEmitter<'_>,
        snapshot_json: &str,
    ) -> zbus::Result<()>;
}

impl AirBatteryInterface {
    fn send_request(&self, request: ServiceRequest) -> zbus::fdo::Result<()> {
        self.requests.try_send(request).map_err(|error| {
            zbus::fdo::Error::Failed(format!(
                "AirBattery desktop request queue is unavailable: {error}"
            ))
        })
    }
}

async fn receive_response<T>(
    receiver: oneshot::Receiver<Result<T, String>>,
) -> zbus::fdo::Result<T> {
    receive_response_with_timeout(receiver, REQUEST_TIMEOUT).await
}

async fn receive_response_with_timeout<T>(
    receiver: oneshot::Receiver<Result<T, String>>,
    wait: Duration,
) -> zbus::fdo::Result<T> {
    match timeout(wait, receiver).await {
        Ok(Ok(result)) => result.map_err(zbus::fdo::Error::Failed),
        Ok(Err(_)) => Err(zbus::fdo::Error::Failed(
            "AirBattery request was canceled".to_owned(),
        )),
        Err(_) => Err(zbus::fdo::Error::Failed(
            "AirBattery request timed out".to_owned(),
        )),
    }
}

/// Handle retained by the desktop application to publish changed snapshots.
#[derive(Debug, Clone)]
pub struct ServiceHandle {
    connection: Connection,
    snapshot_json: Arc<RwLock<String>>,
}

impl ServiceHandle {
    /// Replaces the current snapshot and emits `SnapshotChanged`.
    ///
    /// # Errors
    ///
    /// Returns [`ServiceError`] when the snapshot cannot be delivered to the
    /// running D-Bus service.
    pub async fn publish(&self, snapshot: ServiceSnapshot) -> Result<(), ServiceError> {
        let encoded = snapshot.to_json()?;
        *self.snapshot_json.write().await = encoded.clone();
        let emitter = SignalEmitter::new(&self.connection, OBJECT_PATH)?;
        AirBatteryInterface::snapshot_changed(&emitter, &encoded).await?;
        Ok(())
    }
}

/// Starts the session D-Bus service and returns its command receiver.
///
/// # Errors
///
/// Returns [`ServiceError`] when the session bus cannot be opened or the
/// service name and object cannot be registered.
pub async fn start_service(
    initial_snapshot: ServiceSnapshot,
) -> Result<(ServiceHandle, mpsc::Receiver<ServiceRequest>), ServiceError> {
    let snapshot_json = Arc::new(RwLock::new(initial_snapshot.to_json()?));
    let (request_tx, request_rx) = mpsc::channel(16);
    let interface = AirBatteryInterface {
        snapshot_json: Arc::clone(&snapshot_json),
        requests: request_tx,
    };
    let connection = Builder::session()?
        .name(BUS_NAME)?
        .serve_at(OBJECT_PATH, interface)?
        .build()
        .await?;

    Ok((
        ServiceHandle {
            connection,
            snapshot_json,
        },
        request_rx,
    ))
}

#[cfg(test)]
mod request_tests {
    use super::*;

    #[tokio::test]
    async fn request_response_times_out() {
        let (_sender, receiver) = oneshot::channel::<Result<(), String>>();
        let result = receive_response_with_timeout(receiver, Duration::ZERO).await;
        assert!(matches!(
            result,
            Err(zbus::fdo::Error::Failed(message)) if message.contains("timed out")
        ));
    }

    #[tokio::test]
    async fn completed_response_is_forwarded() {
        let (sender, receiver) = oneshot::channel();
        assert!(sender.send(Ok::<_, String>("snapshot".to_owned())).is_ok());
        let result = receive_response_with_timeout(receiver, Duration::from_millis(10)).await;
        assert!(matches!(result, Ok(value) if value == "snapshot"));
    }
}
