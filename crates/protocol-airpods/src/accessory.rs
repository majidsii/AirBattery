//! `Apple Accessory Communication Protocol` battery packet parser.

use shared_models::{BatteryPercentage, ChargingState};
use thiserror::Error;

use crate::AirPodsComponent;

const HEADER: [u8; 4] = [0x04, 0x00, 0x04, 0x00];
const BATTERY_OPCODE: [u8; 2] = [0x04, 0x00];
const RECORD_LENGTH: usize = 5;
const RIGHT_COMPONENT: u8 = 0x02;
const LEFT_COMPONENT: u8 = 0x04;
const CASE_COMPONENT: u8 = 0x08;

/// Exact component values carried by an active `AirPods` accessory connection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct AirPodsAccessoryBattery {
    /// Exact left earbud value when present in the packet.
    pub left: Option<AirPodsComponent>,
    /// Exact right earbud value when present in the packet.
    pub right: Option<AirPodsComponent>,
    /// Exact charging case value when present in the packet.
    pub case: Option<AirPodsComponent>,
}

/// Strict active accessory packet error.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum AirPodsAccessoryParseError {
    /// The packet declares more component records than it contains.
    #[error(
        "AirPods accessory battery packet is truncated: expected {expected} bytes, got {actual}"
    )]
    Truncated {
        /// Required packet length derived from the record count.
        expected: usize,
        /// Actual received packet length.
        actual: usize,
    },
    /// A component record does not contain the required marker bytes.
    #[error("AirPods accessory battery record {index} has invalid marker bytes")]
    InvalidRecord {
        /// Zero-based component record index.
        index: usize,
    },
    /// An exact battery percentage is outside the supported range.
    #[error("AirPods accessory battery percentage {percentage} is outside 0..=100")]
    InvalidPercentage {
        /// Invalid packet value.
        percentage: u8,
    },
}

fn charging_state(status: u8) -> ChargingState {
    match status {
        0x01 => ChargingState::Charging,
        0x02 => ChargingState::NotCharging,
        _ => ChargingState::Unknown,
    }
}

/// Parses one active Apple accessory battery notification.
///
/// Returns `Ok(None)` for accessory packets carrying another opcode. Unlike the
/// passive proximity advertisement, component percentages are preserved at
/// one-percent resolution.
///
/// # Errors
///
/// Returns [`AirPodsAccessoryParseError`] when a recognized battery packet is
/// truncated or contains invalid record framing or percentages.
pub fn parse_accessory_battery(
    data: &[u8],
) -> Result<Option<AirPodsAccessoryBattery>, AirPodsAccessoryParseError> {
    if data.len() < 6 || data[..4] != HEADER || data[4..6] != BATTERY_OPCODE {
        return Ok(None);
    }
    let Some(&count) = data.get(6) else {
        return Err(AirPodsAccessoryParseError::Truncated {
            expected: 7,
            actual: data.len(),
        });
    };
    let expected = 7 + usize::from(count) * RECORD_LENGTH;
    if data.len() < expected {
        return Err(AirPodsAccessoryParseError::Truncated {
            expected,
            actual: data.len(),
        });
    }

    let mut battery = AirPodsAccessoryBattery {
        left: None,
        right: None,
        case: None,
    };
    for index in 0..usize::from(count) {
        let offset = 7 + index * RECORD_LENGTH;
        let record = &data[offset..offset + RECORD_LENGTH];
        if record[1] != 0x01 || record[4] != 0x01 {
            return Err(AirPodsAccessoryParseError::InvalidRecord { index });
        }
        let state = charging_state(record[3]);
        let temporarily_unavailable = record[2] == u8::MAX
            || (record[0] == CASE_COMPONENT && record[2] == 0 && state == ChargingState::Unknown);
        let percentage = if temporarily_unavailable {
            None
        } else {
            Some(BatteryPercentage::new(record[2]).map_err(|_| {
                AirPodsAccessoryParseError::InvalidPercentage {
                    percentage: record[2],
                }
            })?)
        };
        let component = AirPodsComponent {
            percentage,
            charging_state: percentage.map_or(ChargingState::Unknown, |_| state),
        };
        match record[0] {
            LEFT_COMPONENT => battery.left = Some(component),
            RIGHT_COMPONENT => battery.right = Some(component),
            CASE_COMPONENT => battery.case = Some(component),
            _ => {}
        }
    }

    Ok(Some(battery))
}
