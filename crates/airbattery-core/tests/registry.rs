//! Integration tests for the normalized device registry.

use airbattery_core::{DeviceRegistry, FreshnessPolicy};
use device_protocols::ProviderReport;
use shared_models::{
    BatteryComponent, BatteryPercentage, ChargingState, ComponentType, ConnectionState,
    DataConfidence, DataSource,
};
use time::{Duration, OffsetDateTime};

fn report(kind: ComponentType, at: OffsetDateTime) -> ProviderReport {
    let Ok(percentage) = BatteryPercentage::new(60) else {
        panic!("fixture percentage is invalid");
    };
    ProviderReport::new(
        "fixture",
        "dev",
        50,
        at,
        vec![BatteryComponent {
            component_type: kind,
            percentage: Some(percentage),
            charging_state: ChargingState::Unknown,
            updated_at: at,
            source: DataSource::AirPodsAdvertisement,
            confidence: DataConfidence::High,
            stale: false,
        }],
    )
}

#[test]
fn case_expires_before_earbuds_and_disconnect_marks_stale() {
    let start = OffsetDateTime::UNIX_EPOCH;
    let mut registry = DeviceRegistry::new(FreshnessPolicy::default());
    registry.apply_report(&report(ComponentType::Case, start));
    registry.apply_report(&report(ComponentType::Left, start));

    let later = start + Duration::minutes(2);
    let Some(snapshot) = registry.snapshot("dev", later) else {
        panic!("known device was not returned");
    };
    let Some(case) = snapshot
        .components
        .iter()
        .find(|component| component.component_type == ComponentType::Case)
    else {
        panic!("case component missing");
    };
    let Some(left) = snapshot
        .components
        .iter()
        .find(|component| component.component_type == ComponentType::Left)
    else {
        panic!("left component missing");
    };
    assert!(case.stale);
    assert!(!left.stale);

    registry.set_connection_state("dev", ConnectionState::Disconnected);
    let Some(disconnected) = registry.snapshot("dev", start) else {
        panic!("known device was not returned after disconnect");
    };
    assert!(
        disconnected
            .components
            .iter()
            .all(|component| component.stale)
    );
}

#[test]
fn unavailable_case_update_keeps_last_known_value_until_it_expires() {
    let start = OffsetDateTime::UNIX_EPOCH;
    let mut registry = DeviceRegistry::new(FreshnessPolicy::default());
    registry.apply_report(&report(ComponentType::Case, start));

    registry.apply_report(&ProviderReport::new(
        "fixture",
        "dev",
        50,
        start + Duration::seconds(30),
        vec![BatteryComponent {
            component_type: ComponentType::Case,
            percentage: None,
            charging_state: ChargingState::Unknown,
            updated_at: start + Duration::seconds(30),
            source: DataSource::AirPodsAdvertisement,
            confidence: DataConfidence::High,
            stale: false,
        }],
    ));

    let Some(fresh) = registry.snapshot("dev", start + Duration::seconds(45)) else {
        panic!("known device was not returned");
    };
    let Some(case) = fresh
        .components
        .iter()
        .find(|component| component.component_type == ComponentType::Case)
    else {
        panic!("case component missing");
    };
    assert_eq!(case.percentage.map(BatteryPercentage::get), Some(60));
    assert!(!case.stale);

    let Some(expired) = registry.snapshot("dev", start + Duration::minutes(2)) else {
        panic!("known device was not returned");
    };
    let Some(case) = expired
        .components
        .iter()
        .find(|component| component.component_type == ComponentType::Case)
    else {
        panic!("case component missing after expiry");
    };
    assert!(case.stale);
}

#[test]
fn duplicate_report_is_not_visible_as_a_change() {
    let now = OffsetDateTime::UNIX_EPOCH;
    let mut registry = DeviceRegistry::new(FreshnessPolicy::default());
    let fixture = report(ComponentType::Left, now);

    assert!(registry.apply_report(&fixture));
    assert!(!registry.apply_report(&fixture));
}

#[test]
fn out_of_order_report_cannot_replace_newer_component() {
    let start = OffsetDateTime::UNIX_EPOCH;
    let mut registry = DeviceRegistry::new(FreshnessPolicy::default());
    let newer = start + Duration::minutes(1);
    assert!(registry.apply_report(&report(ComponentType::Left, newer)));

    let Ok(old_percentage) = BatteryPercentage::new(10) else {
        panic!("fixture percentage is invalid");
    };
    assert!(!registry.apply_report(&ProviderReport::new(
        "fixture",
        "dev",
        50,
        start,
        vec![BatteryComponent {
            component_type: ComponentType::Left,
            percentage: Some(old_percentage),
            charging_state: ChargingState::Unknown,
            updated_at: start,
            source: DataSource::AirPodsAdvertisement,
            confidence: DataConfidence::High,
            stale: false,
        }],
    )));

    let Some(snapshot) = registry.snapshot("dev", newer) else {
        panic!("known device missing");
    };
    assert_eq!(
        snapshot.components[0]
            .percentage
            .map(BatteryPercentage::get),
        Some(60)
    );
}

#[test]
fn registry_tracks_devices_independently() {
    let now = OffsetDateTime::UNIX_EPOCH;
    let mut registry = DeviceRegistry::new(FreshnessPolicy::default());
    registry.apply_report(&report(ComponentType::Left, now));

    let mut second = report(ComponentType::Case, now);
    second.device_id = "dev-2".to_owned();
    registry.apply_report(&second);

    assert_eq!(registry.snapshots(now).len(), 2);
    assert!(registry.snapshot("dev", now).is_some());
    assert!(registry.snapshot("dev-2", now).is_some());
}
