//! Integration tests for this crate.

use device_protocols::{ProviderReport, resolve_reports};
use shared_models::{
    BatteryComponent, BatteryPercentage, ChargingState, ComponentType, DataConfidence, DataSource,
};
use time::{Duration, OffsetDateTime};

fn component(
    kind: ComponentType,
    value: u8,
    confidence: DataConfidence,
    source: DataSource,
    updated_at: OffsetDateTime,
    stale: bool,
) -> BatteryComponent {
    let Ok(percentage) = BatteryPercentage::new(value) else {
        panic!("fixture percentage is invalid");
    };
    BatteryComponent {
        component_type: kind,
        percentage: Some(percentage),
        charging_state: ChargingState::Unknown,
        updated_at,
        source,
        confidence,
        stale,
    }
}

#[test]
fn resolver_merges_components_and_rejects_stale_overwrite() {
    let now = OffsetDateTime::UNIX_EPOCH + Duration::minutes(10);
    let reports = vec![
        ProviderReport::new(
            "airpods",
            "device",
            100,
            now - Duration::seconds(2),
            vec![
                component(
                    ComponentType::Left,
                    80,
                    DataConfidence::High,
                    DataSource::AirPodsAdvertisement,
                    now - Duration::seconds(2),
                    false,
                ),
                component(
                    ComponentType::Case,
                    50,
                    DataConfidence::High,
                    DataSource::AirPodsAdvertisement,
                    now - Duration::seconds(2),
                    false,
                ),
            ],
        ),
        ProviderReport::new(
            "bluez",
            "device",
            80,
            now,
            vec![
                component(
                    ComponentType::Left,
                    20,
                    DataConfidence::Verified,
                    DataSource::BluezBattery,
                    now,
                    true,
                ),
                component(
                    ComponentType::Aggregate,
                    70,
                    DataConfidence::Medium,
                    DataSource::BluezBattery,
                    now,
                    false,
                ),
            ],
        ),
    ];
    let resolved = resolve_reports(&reports);
    assert_eq!(
        resolved
            .iter()
            .find(|component| component.component_type == ComponentType::Left)
            .and_then(|component| component.percentage)
            .map(BatteryPercentage::get),
        Some(80)
    );
    assert!(
        resolved
            .iter()
            .any(|component| component.component_type == ComponentType::Case)
    );
    assert!(
        resolved
            .iter()
            .any(|component| component.component_type == ComponentType::Aggregate)
    );
}

#[test]
fn fresher_value_wins_when_quality_is_equal() {
    let now = OffsetDateTime::UNIX_EPOCH + Duration::minutes(10);
    let reports = vec![
        ProviderReport::new(
            "older",
            "device",
            10,
            now - Duration::minutes(1),
            vec![component(
                ComponentType::Headset,
                20,
                DataConfidence::Medium,
                DataSource::AggregatePlatform,
                now - Duration::minutes(1),
                false,
            )],
        ),
        ProviderReport::new(
            "newer",
            "device",
            10,
            now,
            vec![component(
                ComponentType::Headset,
                90,
                DataConfidence::Medium,
                DataSource::AggregatePlatform,
                now,
                false,
            )],
        ),
    ];
    let resolved = resolve_reports(&reports);
    assert_eq!(resolved[0].percentage.map(BatteryPercentage::get), Some(90));
}

#[test]
fn fresher_value_beats_provider_priority_when_quality_is_equal() {
    let now = OffsetDateTime::UNIX_EPOCH + Duration::minutes(10);
    let reports = vec![
        ProviderReport::new(
            "preferred-but-old",
            "device",
            100,
            now - Duration::minutes(1),
            vec![component(
                ComponentType::Headset,
                20,
                DataConfidence::Medium,
                DataSource::AggregatePlatform,
                now - Duration::minutes(1),
                false,
            )],
        ),
        ProviderReport::new(
            "fresh",
            "device",
            1,
            now,
            vec![component(
                ComponentType::Headset,
                90,
                DataConfidence::Medium,
                DataSource::AggregatePlatform,
                now,
                false,
            )],
        ),
    ];

    let resolved = resolve_reports(&reports);
    assert_eq!(resolved[0].percentage.map(BatteryPercentage::get), Some(90));
}

#[test]
fn available_case_value_beats_higher_priority_temporarily_unavailable_value() {
    let now = OffsetDateTime::UNIX_EPOCH + Duration::minutes(10);
    let valid_case = component(
        ComponentType::Case,
        12,
        DataConfidence::Medium,
        DataSource::AirPodsAdvertisement,
        now - Duration::seconds(2),
        false,
    );
    let unavailable_case = BatteryComponent {
        component_type: ComponentType::Case,
        percentage: None,
        charging_state: ChargingState::Unknown,
        updated_at: now,
        source: DataSource::VendorProtocol,
        confidence: DataConfidence::Verified,
        stale: false,
    };
    let reports = vec![
        ProviderReport::new("passive", "device", 100, now, vec![valid_case]),
        ProviderReport::new("exact", "device", 300, now, vec![unavailable_case]),
    ];

    let resolved = resolve_reports(&reports);
    let case = resolved
        .iter()
        .find(|component| component.component_type == ComponentType::Case)
        .unwrap_or_else(|| panic!("case component missing"));
    assert_eq!(case.percentage.map(BatteryPercentage::get), Some(12));
}
