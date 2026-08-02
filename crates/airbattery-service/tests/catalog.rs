//! Integration tests for this crate.

use std::sync::Arc;

use airbattery_service::{ApplicationEngine, DeviceDescriptor};
use async_trait::async_trait;
use device_protocols::{DeviceProvider, ProviderError, ProviderReport, RawObservation};
use shared_models::{
    BatteryComponent, BatteryPercentage, Capability, ChargingState, ComponentType, ConnectionState,
    DataConfidence, DataSource, DeviceFamily, Transport,
};
use time::OffsetDateTime;

#[derive(Debug)]
struct FixtureProvider;

#[async_trait]
impl DeviceProvider for FixtureProvider {
    fn id(&self) -> &'static str {
        "fixture"
    }
    fn supports(&self, observation: &RawObservation) -> bool {
        matches!(observation, RawObservation::PlatformBattery { .. })
    }
    async fn parse(
        &self,
        observation: &RawObservation,
    ) -> Result<Option<ProviderReport>, ProviderError> {
        let RawObservation::PlatformBattery {
            device_id,
            percentage,
            observed_at,
            ..
        } = observation
        else {
            return Ok(None);
        };
        let percentage = BatteryPercentage::new(*percentage)
            .map_err(|error| ProviderError::InvalidObservation(error.to_string()))?;
        Ok(Some(ProviderReport::new(
            self.id(),
            device_id,
            10,
            *observed_at,
            vec![BatteryComponent {
                component_type: ComponentType::Aggregate,
                percentage: Some(percentage),
                charging_state: ChargingState::Unknown,
                updated_at: *observed_at,
                source: DataSource::AggregatePlatform,
                confidence: DataConfidence::Medium,
                stale: false,
            }],
        )))
    }
}

fn descriptor() -> DeviceDescriptor {
    DeviceDescriptor {
        backend_id: "raw-address".to_owned(),
        privacy_id: "bt-safe".to_owned(),
        display_name: "Headphones".to_owned(),
        system_name: Some("System Headphones".to_owned()),
        manufacturer: None,
        model: None,
        icon: None,
        class: None,
        appearance: None,
        service_uuids: Vec::new(),
        device_family: DeviceFamily::Headset,
        transport: Transport::Dual,
        last_seen_at: Some(OffsetDateTime::UNIX_EPOCH),
    }
}

#[tokio::test]
async fn engine_exposes_privacy_id_and_derived_capabilities() {
    let mut engine = ApplicationEngine::new(vec![Arc::new(FixtureProvider)]);
    engine.upsert_descriptor(descriptor());
    engine
        .process(&RawObservation::Connection {
            device_id: "raw-address".to_owned(),
            state: ConnectionState::Connected,
            observed_at: OffsetDateTime::UNIX_EPOCH,
        })
        .await
        .unwrap_or_else(|error| panic!("connection processing failed: {error}"));
    let device = engine
        .process(&RawObservation::PlatformBattery {
            device_id: "raw-address".to_owned(),
            percentage: 44,
            source: DataSource::AggregatePlatform,
            observed_at: OffsetDateTime::UNIX_EPOCH,
        })
        .await
        .unwrap_or_else(|error| panic!("battery processing failed: {error}"))
        .unwrap_or_else(|| panic!("known descriptor did not produce a device"));

    assert_eq!(device.id, "bt-safe");
    assert_eq!(device.display_name, "Headphones");
    assert_eq!(device.connection_state, ConnectionState::Connected);
    assert!(device.capabilities.contains(&Capability::AggregateBattery));
    assert_eq!(device.last_updated_at, Some(OffsetDateTime::UNIX_EPOCH));
}

#[tokio::test]
async fn unknown_backend_device_is_not_exposed_without_metadata() {
    let mut engine = ApplicationEngine::new(vec![Arc::new(FixtureProvider)]);
    let result = engine
        .process(&RawObservation::PlatformBattery {
            device_id: "unknown".to_owned(),
            percentage: 50,
            source: DataSource::AggregatePlatform,
            observed_at: OffsetDateTime::UNIX_EPOCH,
        })
        .await
        .unwrap_or_else(|error| panic!("processing failed: {error}"));
    assert!(result.is_none());
}

#[tokio::test]
async fn descriptor_reconciliation_removes_expired_platform_devices() {
    use std::collections::BTreeSet;

    let mut engine = ApplicationEngine::new(vec![Arc::new(FixtureProvider)]);
    engine.upsert_descriptor(descriptor());
    engine
        .process(&RawObservation::Connection {
            device_id: "raw-address".to_owned(),
            state: ConnectionState::Connected,
            observed_at: OffsetDateTime::UNIX_EPOCH,
        })
        .await
        .unwrap_or_else(|error| panic!("connection processing failed: {error}"));

    engine.retain_descriptors(&BTreeSet::new());

    assert!(engine.devices(OffsetDateTime::UNIX_EPOCH).is_empty());
}
