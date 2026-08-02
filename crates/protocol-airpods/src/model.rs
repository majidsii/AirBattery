//! Normalized values parsed from an `AirPods` advertisement.

use serde::{Deserialize, Serialize};
use shared_models::{BatteryPercentage, ChargingState};

/// `AirPods` model identified by the proximity-pairing advertisement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum AirPodsModel {
    /// `AirPods` first generation.
    AirPods1,
    /// `AirPods` second generation.
    AirPods2,
    /// `AirPods` third generation.
    AirPods3,
    /// `AirPods` Pro first generation.
    AirPodsPro,
    /// `AirPods` Pro second generation.
    AirPodsPro2,
    /// `AirPods` Max.
    AirPodsMax,
    /// Unknown model nibble.
    Unknown(u8),
}

impl AirPodsModel {
    /// Returns the stable user-visible model label.
    #[must_use]
    pub const fn display_name(self) -> &'static str {
        match self {
            Self::AirPods1 => "AirPods (1st generation)",
            Self::AirPods2 => "AirPods (2nd generation)",
            Self::AirPods3 => "AirPods (3rd generation)",
            Self::AirPodsPro => "AirPods Pro (1st generation)",
            Self::AirPodsPro2 => "AirPods Pro (2nd generation)",
            Self::AirPodsMax => "AirPods Max",
            Self::Unknown(_) => "AirPods",
        }
    }
}

/// One component decoded from a complete advertisement.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AirPodsComponent {
    /// Percentage in deciles, or unavailable when the nibble is 11..=15.
    pub percentage: Option<BatteryPercentage>,
    /// Charging state, unknown when the component is absent.
    pub charging_state: ChargingState,
}

/// Complete `AirPods` proximity-pairing battery snapshot.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AirPodsAdvertisement {
    /// Detected model.
    pub model: AirPodsModel,
    /// Left earbud component.
    pub left: AirPodsComponent,
    /// Right earbud component.
    pub right: AirPodsComponent,
    /// Charging case component.
    pub case: AirPodsComponent,
}
