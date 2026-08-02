//! Provider contracts and raw backend observations.

use async_trait::async_trait;
use shared_models::{BatteryComponent, ConnectionState, DataSource};
use thiserror::Error;
use time::OffsetDateTime;

/// Documented vendor transport attached to a raw packet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum VendorProtocol {
    /// `Apple Accessory Communication Protocol` over classic `Bluetooth` `L2CAP`.
    AppleAccessory,
}

/// Raw information produced by a platform `Bluetooth` backend.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum RawObservation {
    /// Manufacturer-specific advertisement payload.
    ManufacturerData {
        /// Backend device identifier.
        device_id: String,
        /// `Bluetooth SIG` company identifier.
        company_id: u16,
        /// Bytes after the company identifier.
        payload: Vec<u8>,
        /// Observation timestamp.
        observed_at: OffsetDateTime,
    },
    /// Packet received from a documented active vendor transport.
    VendorPacket {
        /// Backend device identifier.
        device_id: String,
        /// Vendor protocol used by the transport.
        protocol: VendorProtocol,
        /// Complete vendor packet bytes.
        payload: Vec<u8>,
        /// Observation timestamp.
        observed_at: OffsetDateTime,
    },
    /// Service-specific advertising payload.
    ServiceData {
        /// Backend device identifier.
        device_id: String,
        /// Canonical service UUID.
        service_uuid: String,
        /// Raw advertised bytes.
        payload: Vec<u8>,
        /// Observation timestamp.
        observed_at: OffsetDateTime,
    },
    /// Aggregate battery supplied by a platform API.
    PlatformBattery {
        /// Backend device identifier.
        device_id: String,
        /// Raw percentage from the platform.
        percentage: u8,
        /// Exact platform source.
        source: DataSource,
        /// Observation timestamp.
        observed_at: OffsetDateTime,
    },
    /// Standard Battery Service value.
    StandardBattery {
        /// Backend device identifier.
        device_id: String,
        /// Raw percentage from characteristic 0x2A19.
        percentage: u8,
        /// Observation timestamp.
        observed_at: OffsetDateTime,
    },
    /// Connection state update.
    Connection {
        /// Backend device identifier.
        device_id: String,
        /// New connection state.
        state: ConnectionState,
        /// Observation timestamp.
        observed_at: OffsetDateTime,
    },
}

/// One provider's normalized component report.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProviderReport {
    /// Stable provider identifier used for deterministic tie-breaking.
    pub provider_id: String,
    /// Backend device identifier.
    pub device_id: String,
    /// Provider-specific priority.
    pub priority: u16,
    /// Report timestamp.
    pub observed_at: OffsetDateTime,
    /// Normalized components.
    pub components: Vec<BatteryComponent>,
}

impl ProviderReport {
    /// Creates a provider report.
    #[must_use]
    pub fn new(
        provider_id: impl Into<String>,
        device_id: impl Into<String>,
        priority: u16,
        observed_at: OffsetDateTime,
        components: Vec<BatteryComponent>,
    ) -> Self {
        Self {
            provider_id: provider_id.into(),
            device_id: device_id.into(),
            priority,
            observed_at,
            components,
        }
    }
}

/// Error returned by a device provider.
#[derive(Debug, Error)]
pub enum ProviderError {
    /// Observation is malformed for this provider.
    #[error("invalid observation: {0}")]
    InvalidObservation(String),
    /// Provider cannot process the observation.
    #[error("unsupported observation")]
    Unsupported,
}

/// Extensible protocol provider contract.
#[async_trait]
pub trait DeviceProvider: Send + Sync {
    /// Stable provider identifier.
    fn id(&self) -> &'static str;

    /// Whether this provider recognizes the observation shape.
    fn supports(&self, observation: &RawObservation) -> bool;

    /// Parses one observation into an optional normalized report.
    async fn parse(
        &self,
        observation: &RawObservation,
    ) -> Result<Option<ProviderReport>, ProviderError>;
}
