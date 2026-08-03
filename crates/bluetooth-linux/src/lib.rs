//! Native Linux Bluetooth integration built on `BlueZ`.

mod accessory;
mod backend;
mod mapping;

pub use accessory::{
    AppleAccessoryPacket, latest_apple_accessory_battery, stop_apple_accessory_monitors,
    sync_apple_accessory_monitors,
};
pub use backend::{BluetoothLinuxError, LinuxBackendStatus, LinuxBluetoothBackend};
pub use mapping::{BluezDeviceSnapshot, map_bluez_snapshot};
