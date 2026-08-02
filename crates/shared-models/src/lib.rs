//! Platform-independent `AirBattery` domain types.

mod battery;
mod device;

pub use battery::{
    BatteryComponent, BatteryPercentage, ChargingState, ComponentType, DataConfidence, DataSource,
    InvalidBatteryPercentage,
};
pub use device::{
    BluetoothAudioDevice, Capability, ConnectionState, DeviceFamily, DeviceVisual, Transport,
    VisualConfidence,
};
