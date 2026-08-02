//! Device protocol provider contracts and report resolution.

mod provider;
mod resolver;

pub use provider::{DeviceProvider, ProviderError, ProviderReport, RawObservation, VendorProtocol};
pub use resolver::resolve_reports;
