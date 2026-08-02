//! Desktop-host transport models.

use airbattery_service::DeviceDescriptor;
use device_protocols::RawObservation;
use serde::{Deserialize, Serialize};
use shared_models::BluetoothAudioDevice;

/// Runtime platform name exposed to the frontend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum PlatformKind {
    /// `BlueZ`-based Linux host.
    Linux,
    /// Windows Runtime host.
    Windows,
    /// Build target without a Bluetooth adapter implementation.
    Unsupported,
}

impl PlatformKind {
    /// Returns the platform selected at compile time.
    #[must_use]
    pub const fn current() -> Self {
        #[cfg(target_os = "linux")]
        {
            return Self::Linux;
        }
        #[cfg(target_os = "windows")]
        {
            return Self::Windows;
        }
        #[allow(unreachable_code)]
        Self::Unsupported
    }
}

/// Current platform Bluetooth health.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BackendStatus {
    /// Active platform backend.
    pub platform: PlatformKind,
    /// Whether the native backend can currently be queried.
    pub available: bool,
    /// Adapter name supplied by the operating system.
    pub adapter_name: Option<String>,
    /// Adapter power state when exposed.
    pub powered: Option<bool>,
    /// Active bounded discovery state when exposed.
    pub discovering: Option<bool>,
    /// User-safe status detail.
    pub detail: String,
}

impl BackendStatus {
    /// Creates an unavailable backend status without failing application startup.
    #[must_use]
    pub fn unavailable(platform: PlatformKind, detail: impl Into<String>) -> Self {
        Self {
            platform,
            available: false,
            adapter_name: None,
            powered: None,
            discovering: None,
            detail: detail.into(),
        }
    }
}

/// Platform observations collected as one deterministic refresh transaction.
#[derive(Debug, Clone)]
pub struct PlatformCollection {
    /// Current native backend health.
    pub status: BackendStatus,
    /// Metadata keyed by backend-local identifiers.
    pub descriptors: Vec<DeviceDescriptor>,
    /// Raw values routed through protocol providers.
    pub observations: Vec<RawObservation>,
}

/// Snapshot returned after state application.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct RefreshResult {
    /// Current normalized public devices.
    pub devices: Vec<BluetoothAudioDevice>,
    /// Current backend status.
    pub status: BackendStatus,
    /// Whether the public device snapshot changed.
    pub devices_changed: bool,
    /// Whether native backend health changed independently of device data.
    pub status_changed: bool,
}

/// Whether a platform collection may initiate bounded discovery.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum RefreshMode {
    /// Read only devices already known to the operating system.
    KnownOnly,
    /// Add a short discovery window to obtain intermittent advertisements.
    BoundedDiscovery,
    /// Run a longer user-initiated discovery window for hardware diagnostics.
    DiagnosticDiscovery,
}
