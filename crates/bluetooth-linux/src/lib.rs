//! Native Linux Bluetooth integration built on `BlueZ`.

mod accessory;
mod backend;
mod mapping;

pub use accessory::{AppleAccessoryPacket, latest_apple_accessory_battery};
pub use backend::{BluetoothLinuxError, LinuxBackendStatus, LinuxBluetoothBackend};
pub use mapping::{BluezDeviceSnapshot, map_bluez_snapshot};
