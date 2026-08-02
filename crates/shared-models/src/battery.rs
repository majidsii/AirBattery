//! Battery-related domain values.

use serde::{Deserialize, Serialize};
use thiserror::Error;
use time::OffsetDateTime;

/// Error returned when a battery percentage is outside 0..=100.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
#[error("battery percentage {value} is outside the valid 0..=100 range")]
pub struct InvalidBatteryPercentage {
    value: u8,
}

/// A validated battery percentage in the inclusive range 0..=100.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(transparent)]
pub struct BatteryPercentage(u8);

impl BatteryPercentage {
    /// Creates a validated percentage.
    ///
    /// # Errors
    ///
    /// Returns [`InvalidBatteryPercentage`] when `value` is greater than 100.
    pub fn new(value: u8) -> Result<Self, InvalidBatteryPercentage> {
        if value <= 100 {
            Ok(Self(value))
        } else {
            Err(InvalidBatteryPercentage { value })
        }
    }

    /// Returns the percentage as an integer.
    #[must_use]
    pub const fn get(self) -> u8 {
        self.0
    }
}

/// Logical battery component exposed by a device.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ComponentType {
    /// Left earbud.
    Left,
    /// Right earbud.
    Right,
    /// Charging case.
    Case,
    /// Whole headset value from a standard Battery Service.
    Headset,
    /// Aggregate platform-provided value.
    Aggregate,
    /// A component the provider cannot classify further.
    Unknown,
}

/// Charging state supported by reliable source evidence.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum ChargingState {
    /// The component is charging.
    Charging,
    /// The component is known not to be charging.
    NotCharging,
    /// The source explicitly reports a full state.
    Full,
    /// The source does not reliably expose charging state.
    Unknown,
}

/// Normalized origin of battery information.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DataSource {
    /// Apple Continuity proximity-pairing advertisement.
    AirPodsAdvertisement,
    /// `BlueZ` `org.bluez.Battery1` value.
    BluezBattery,
    /// Bluetooth SIG standard Battery Service.
    StandardBleBattery,
    /// Windows platform battery value.
    WindowsBattery,
    /// Battery value decoded from advertised service data.
    ServiceData,
    /// Battery value reported through a Human Interface Device channel.
    HidBattery,
    /// Battery value decoded from a documented vendor protocol.
    VendorProtocol,
    /// Generic aggregate value supplied by an operating system.
    AggregatePlatform,
}

impl DataSource {
    /// Stable resolver priority for equally fresh and confident values.
    #[must_use]
    pub const fn priority(self) -> u8 {
        match self {
            Self::VendorProtocol => 55,
            Self::AirPodsAdvertisement => 50,
            Self::StandardBleBattery | Self::HidBattery => 40,
            Self::BluezBattery | Self::WindowsBattery | Self::ServiceData => 30,
            Self::AggregatePlatform => 20,
        }
    }
}

/// Provider confidence in a normalized value.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum DataConfidence {
    /// No useful confidence signal.
    Unknown,
    /// Weak heuristic evidence.
    Low,
    /// Platform-reported or partially verified evidence.
    Medium,
    /// Protocol-specific validated evidence.
    High,
    /// Direct standard or platform evidence with verified semantics.
    Verified,
}

/// One user-visible battery component.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct BatteryComponent {
    /// Component kind.
    pub component_type: ComponentType,
    /// Percentage when currently known. `None` is never equivalent to zero.
    pub percentage: Option<BatteryPercentage>,
    /// Charging state when reliably known.
    pub charging_state: ChargingState,
    /// Timestamp of the source observation, encoded as RFC 3339 for every IPC surface.
    #[serde(with = "time::serde::rfc3339")]
    pub updated_at: OffsetDateTime,
    /// Source selected for this component.
    pub source: DataSource,
    /// Confidence assigned by the provider.
    pub confidence: DataConfidence,
    /// Whether this value is older than the applicable freshness policy.
    pub stale: bool,
}
