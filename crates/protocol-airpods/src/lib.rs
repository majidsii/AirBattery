//! Clean-room Rust implementation of `AirPods` proximity-pairing battery parsing.

mod accessory;
mod model;
mod parser;
mod provider;

pub use accessory::{AirPodsAccessoryBattery, AirPodsAccessoryParseError, parse_accessory_battery};
pub use model::{AirPodsAdvertisement, AirPodsComponent, AirPodsModel};
pub use parser::{
    APPLE_MANUFACTURER_ID, AirPodsParseError, PROXIMITY_PAIRING_TYPE, decode_battery_nibble,
    parse_proximity_pairing,
};
pub use provider::{AirPodsAccessoryProvider, AirPodsProvider};
