//! Serialized refresh state and validated settings persistence.

use std::{collections::BTreeSet, sync::Arc};

use airbattery_core::ProcessingError;
use airbattery_service::ApplicationEngine;
use airbattery_settings::{AppSettings, SettingsError, SettingsRepository};
use device_protocols::DeviceProvider;
use protocol_airpods::{AirPodsAccessoryProvider, AirPodsProvider};
use protocol_generic_battery::GenericBatteryProvider;
use shared_models::BluetoothAudioDevice;
use time::OffsetDateTime;
use tokio::sync::{Mutex, MutexGuard, RwLock};

use crate::model::{BackendStatus, PlatformCollection, PlatformKind, RefreshResult};

/// Tauri-managed application state.
#[derive(Debug)]
pub struct RuntimeState {
    refresh_serial: Mutex<()>,
    engine: RwLock<ApplicationEngine>,
    devices: RwLock<Vec<BluetoothAudioDevice>>,
    backend_status: RwLock<BackendStatus>,
    settings: SettingsRepository,
}

impl RuntimeState {
    /// Creates empty runtime state bound to an application-controlled settings path.
    #[must_use]
    pub fn new(settings: SettingsRepository) -> Self {
        let providers: Vec<Arc<dyn DeviceProvider>> = vec![
            Arc::new(AirPodsAccessoryProvider),
            Arc::new(AirPodsProvider),
            Arc::new(GenericBatteryProvider),
        ];
        Self {
            refresh_serial: Mutex::new(()),
            engine: RwLock::new(ApplicationEngine::new(providers)),
            devices: RwLock::new(Vec::new()),
            backend_status: RwLock::new(BackendStatus::unavailable(
                PlatformKind::current(),
                "Bluetooth backend has not started yet.",
            )),
            settings,
        }
    }

    /// Serializes bounded scans and state application.
    pub async fn lock_refresh(&self) -> MutexGuard<'_, ()> {
        self.refresh_serial.lock().await
    }

    /// Returns an isolated device snapshot.
    pub async fn devices(&self) -> Vec<BluetoothAudioDevice> {
        self.devices.read().await.clone()
    }

    /// Returns a cloned backend status.
    pub async fn backend_status(&self) -> BackendStatus {
        self.backend_status.read().await.clone()
    }

    /// Updates backend status while retaining prior device values for stale evaluation.
    pub async fn set_backend_status(&self, status: BackendStatus) {
        *self.backend_status.write().await = status;
    }

    /// Applies one native collection and deduplicates the public snapshot.
    pub async fn apply_collection(&self, collection: PlatformCollection) -> RefreshResult {
        let PlatformCollection {
            status,
            descriptors,
            observations,
        } = collection;

        let mut engine = self.engine.write().await;
        if status.available {
            let backend_ids = descriptors
                .iter()
                .map(|descriptor| descriptor.backend_id.clone())
                .collect::<BTreeSet<_>>();
            engine.retain_descriptors(&backend_ids);
        }
        for descriptor in descriptors {
            engine.upsert_descriptor(descriptor);
        }
        for observation in &observations {
            if let Err(error) = engine.process(observation).await {
                log_processing_error(&error);
            }
        }
        let devices = engine.devices(OffsetDateTime::now_utc());
        drop(engine);

        let mut current_devices = self.devices.write().await;
        let devices_changed = *current_devices != devices;
        if devices_changed {
            (*current_devices).clone_from(&devices);
        }
        drop(current_devices);

        let mut current_status = self.backend_status.write().await;
        let status_changed = *current_status != status;
        if status_changed {
            *current_status = status.clone();
        }
        drop(current_status);

        RefreshResult {
            devices,
            status,
            devices_changed,
            status_changed,
        }
    }

    /// Loads normalized settings, including corruption recovery.
    ///
    /// # Errors
    ///
    /// Returns [`SettingsError`] when stored settings cannot be read, parsed,
    /// validated, or recovered from a corrupted settings file.
    pub fn load_settings(&self) -> Result<AppSettings, SettingsError> {
        self.settings.load().map(|result| result.settings)
    }

    /// Persists normalized settings atomically.
    ///
    /// # Errors
    ///
    /// Returns [`SettingsError`] when normalized settings cannot be serialized
    /// or atomically written to the application settings file.
    pub fn save_settings(&self, settings: &AppSettings) -> Result<AppSettings, SettingsError> {
        let normalized = settings.clone().normalized();
        self.settings.save(&normalized)?;
        Ok(normalized)
    }
}

fn log_processing_error(error: &ProcessingError) {
    match error {
        ProcessingError::Provider {
            provider_id,
            source,
        } => {
            tracing::warn!(provider_id, error = %source, "Bluetooth provider rejected an observation");
        }
        ProcessingError::MissingSnapshot { .. } => {
            tracing::debug!("Bluetooth observation produced no battery report");
        }
    }
}
