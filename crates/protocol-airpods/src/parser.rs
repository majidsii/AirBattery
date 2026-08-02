//! Apple Continuity proximity-pairing advertisement parser.

use shared_models::{BatteryPercentage, ChargingState};
use thiserror::Error;

use crate::{AirPodsAdvertisement, AirPodsComponent, AirPodsModel};

/// Apple Bluetooth SIG company identifier.
pub const APPLE_MANUFACTURER_ID: u16 = 0x004c;
/// Continuity proximity-pairing message type.
pub const PROXIMITY_PAIRING_TYPE: u8 = 0x07;
const PAYLOAD_LENGTH: usize = 27;

/// Strict parser error for a recognized but incomplete payload.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Error)]
pub enum AirPodsParseError {
    /// A proximity-pairing payload must contain at least 27 bytes.
    #[error(
        "AirPods proximity-pairing payload is truncated: expected at least 27 bytes, got {actual}"
    )]
    Truncated {
        /// Actual payload length.
        actual: usize,
    },
}

/// Converts an Apple decile nibble to a validated percentage.
#[must_use]
pub fn decode_battery_nibble(nibble: u8) -> Option<BatteryPercentage> {
    (nibble <= 10)
        .then(|| BatteryPercentage::new(nibble.saturating_mul(10)).ok())
        .flatten()
}

fn nibble_at(data: &[u8], index: usize) -> u8 {
    let byte = data[index / 2];
    if index.is_multiple_of(2) {
        byte >> 4
    } else {
        byte & 0x0f
    }
}

fn model_from_nibble(nibble: u8) -> AirPodsModel {
    match nibble {
        0x2 => AirPodsModel::AirPods1,
        0xf => AirPodsModel::AirPods2,
        0x3 => AirPodsModel::AirPods3,
        0xe => AirPodsModel::AirPodsPro,
        0x4 => AirPodsModel::AirPodsPro2,
        0xa => AirPodsModel::AirPodsMax,
        other => AirPodsModel::Unknown(other),
    }
}

fn component(percentage_nibble: u8, charge_nibble: u8, charge_bit: u8) -> AirPodsComponent {
    let percentage = decode_battery_nibble(percentage_nibble);
    let charging_state = match percentage {
        None => ChargingState::Unknown,
        Some(_) if charge_nibble & charge_bit != 0 => ChargingState::Charging,
        Some(_) => ChargingState::NotCharging,
    };
    AirPodsComponent {
        percentage,
        charging_state,
    }
}

/// Parses the bytes after Apple company id `0x004C`.
///
/// Returns `Ok(None)` for non-proximity Apple advertisements and a typed error
/// only when message type `0x07` is recognized but incomplete.
///
/// # Errors
///
/// Returns [`AirPodsParseError`] when the proximity-pairing payload is
/// malformed or shorter than the required packet length.
pub fn parse_proximity_pairing(
    data: &[u8],
) -> Result<Option<AirPodsAdvertisement>, AirPodsParseError> {
    if data.first().copied() != Some(PROXIMITY_PAIRING_TYPE) {
        return Ok(None);
    }
    if data.len() < PAYLOAD_LENGTH {
        return Err(AirPodsParseError::Truncated { actual: data.len() });
    }

    let model = model_from_nibble(nibble_at(data, 7));
    let flipped = nibble_at(data, 10) & 0x02 == 0;
    let left_index = if flipped { 12 } else { 13 };
    let right_index = if flipped { 13 } else { 12 };
    let charge = nibble_at(data, 14);
    let left_bit = if flipped { 0b0010 } else { 0b0001 };
    let right_bit = if flipped { 0b0001 } else { 0b0010 };

    Ok(Some(AirPodsAdvertisement {
        model,
        left: component(nibble_at(data, left_index), charge, left_bit),
        right: component(nibble_at(data, right_index), charge, right_bit),
        case: component(nibble_at(data, 15), charge, 0b0100),
    }))
}
