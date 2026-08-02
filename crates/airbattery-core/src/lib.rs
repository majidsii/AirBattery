//! `AirBattery` orchestration and normalized device registry.

mod freshness;
mod processor;
mod registry;

pub use freshness::{FreshnessPolicy, expire_components};
pub use processor::{ObservationProcessor, ProcessingError};
pub use registry::{DeviceRegistry, DeviceSnapshot};
