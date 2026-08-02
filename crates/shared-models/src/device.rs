//! Platform-independent Bluetooth device model.

use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};
use time::OffsetDateTime;

use crate::BatteryComponent;

/// Device transport reported by the operating system.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Transport {
    /// Bluetooth Low Energy.
    LowEnergy,
    /// Bluetooth Classic / BR-EDR.
    Classic,
    /// Device supports both transports or the platform does not disambiguate.
    Dual,
    /// Transport is unavailable.
    Unknown,
}

/// Connection state reported by the backend.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ConnectionState {
    /// Device is connected.
    Connected,
    /// Device is paired or known but not connected.
    Disconnected,
    /// Backend is attempting to reconnect.
    Connecting,
    /// State is not currently available.
    #[default]
    Unknown,
}

/// Broad device family used for capability-aware presentation.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DeviceFamily {
    /// Apple `AirPods` family.
    AirPods,
    /// Generic earbuds with independent components.
    Earbuds,
    /// Headphones or headset.
    Headset,
    /// Portable or stationary speaker.
    Speaker,
    /// Pointing device.
    Mouse,
    /// Keyboard.
    Keyboard,
    /// Game controller or joystick.
    GameController,
    /// Stylus or digital pen.
    Stylus,
    /// Generic BLE device.
    GenericBle,
    /// Unknown family.
    Unknown,
}

/// Confidence assigned to model-aware artwork selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum VisualConfidence {
    /// Exact model evidence.
    Exact,
    /// Device-category evidence.
    Category,
    /// Generic Bluetooth fallback.
    Fallback,
}

/// Presentation artwork selected without affecting battery semantics.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DeviceVisual {
    /// Stable artwork key interpreted by each UI surface.
    pub key: String,
    /// Strength of evidence behind the selected key.
    pub confidence: VisualConfidence,
}

/// Capability a device or provider can expose.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Capability {
    /// Per-earbud battery values.
    EarbudBattery,
    /// Charging-case battery.
    CaseBattery,
    /// Aggregate or headset battery.
    AggregateBattery,
    /// Per-component charging state.
    ChargingState,
}

/// Complete normalized device snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BluetoothAudioDevice {
    /// Privacy-safe stable identifier assigned by `AirBattery`.
    pub id: String,
    /// User-visible name.
    pub display_name: String,
    /// Operating-system name, when different.
    pub system_name: Option<String>,
    /// Manufacturer when reliably detected.
    pub manufacturer: Option<String>,
    /// Model when reliably detected.
    pub model: Option<String>,
    /// Device family.
    pub device_family: DeviceFamily,
    /// Model-aware or category-aware presentation artwork.
    pub visual: DeviceVisual,
    /// Bluetooth transport.
    pub transport: Transport,
    /// Current connection state.
    pub connection_state: ConnectionState,
    /// Last time the platform observed the device, encoded as RFC 3339 when present.
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub last_seen_at: Option<OffsetDateTime>,
    /// Last successful normalized battery update, encoded as RFC 3339 when present.
    #[serde(default, with = "time::serde::rfc3339::option")]
    pub last_updated_at: Option<OffsetDateTime>,
    /// Supported capabilities.
    pub capabilities: BTreeSet<Capability>,
    /// Current normalized components.
    pub components: Vec<BatteryComponent>,
}
