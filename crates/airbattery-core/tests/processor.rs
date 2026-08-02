//! Integration tests for observation processing.

use std::sync::Arc;

use airbattery_core::{FreshnessPolicy, ObservationProcessor};
use async_trait::async_trait;
use device_protocols::{DeviceProvider, ProviderError, ProviderReport, RawObservation};
use shared_models::{
    BatteryComponent, BatteryPercentage, ChargingState, ComponentType, ConnectionState,
    DataConfidence, DataSource,
};
use time::OffsetDateTime;

#[derive(Debug)]
struct FixtureProvider {
    id: &'static str,
    priority: u16,
    component_type: ComponentType,
    percentage: u8,
}

#[async_trait]
impl DeviceProvider for FixtureProvider {
    fn id(&self) -> &'static str {
        self.id
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
            observed_at,
            ..
        } = observation
        else {
            return Err(ProviderError::Unsupported);
        };
        let percentage = BatteryPercentage::new(self.percentage)
            .map_err(|error| ProviderError::InvalidObservation(error.to_string()))?;
        Ok(Some(ProviderReport::new(
            self.id,
            device_id,
            self.priority,
            *observed_at,
            vec![BatteryComponent {
                component_type: self.component_type,
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

#[tokio::test]
async fn processor_routes_observations_and_merges_provider_reports() {
    let now = OffsetDateTime::UNIX_EPOCH;
    let providers: Vec<Arc<dyn DeviceProvider>> = vec![
        Arc::new(FixtureProvider {
            id: "left",
            priority: 10,
            component_type: ComponentType::Left,
            percentage: 60,
        }),
        Arc::new(FixtureProvider {
            id: "case",
            priority: 20,
            component_type: ComponentType::Case,
            percentage: 80,
        }),
    ];
    let mut processor = ObservationProcessor::new(FreshnessPolicy::default(), providers);

    let snapshot = processor
        .process(&RawObservation::PlatformBattery {
            device_id: "dev".to_owned(),
            percentage: 1,
            source: DataSource::AggregatePlatform,
            observed_at: now,
        })
        .await
        .unwrap_or_else(|error| panic!("processing failed: {error}"));

    assert_eq!(snapshot.device_id, "dev");
    assert_eq!(snapshot.components.len(), 2);
    assert!(
        snapshot
            .components
            .iter()
            .any(|component| component.component_type == ComponentType::Left)
    );
    assert!(
        snapshot
            .components
            .iter()
            .any(|component| component.component_type == ComponentType::Case)
    );
}

#[tokio::test]
async fn processor_applies_connection_observations_without_protocol_parsing() {
    let now = OffsetDateTime::UNIX_EPOCH;
    let mut processor = ObservationProcessor::new(FreshnessPolicy::default(), Vec::new());

    let snapshot = processor
        .process(&RawObservation::Connection {
            device_id: "dev".to_owned(),
            state: ConnectionState::Connected,
            observed_at: now,
        })
        .await
        .unwrap_or_else(|error| panic!("processing failed: {error}"));

    assert_eq!(snapshot.connection_state, ConnectionState::Connected);
    assert!(snapshot.components.is_empty());
}

#[tokio::test]
async fn processor_prunes_devices_absent_from_authoritative_catalog() {
    use std::collections::BTreeSet;

    let now = OffsetDateTime::UNIX_EPOCH;
    let mut processor = ObservationProcessor::new(FreshnessPolicy::default(), Vec::new());
    for device_id in ["keep", "remove"] {
        processor
            .process(&RawObservation::Connection {
                device_id: device_id.to_owned(),
                state: ConnectionState::Connected,
                observed_at: now,
            })
            .await
            .unwrap_or_else(|error| panic!("processing failed: {error}"));
    }

    processor.retain_devices(&BTreeSet::from(["keep".to_owned()]));

    let snapshots = processor.snapshots(now);
    assert_eq!(snapshots.len(), 1);
    assert_eq!(snapshots[0].device_id, "keep");
}
