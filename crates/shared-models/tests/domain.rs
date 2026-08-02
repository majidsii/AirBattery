//! Integration tests for this crate.

use shared_models::{
    BatteryComponent, BatteryPercentage, ChargingState, ComponentType, DataConfidence, DataSource,
};
use time::OffsetDateTime;

#[test]
fn percentage_accepts_100_and_rejects_101() {
    assert_eq!(
        BatteryPercentage::new(100).map(BatteryPercentage::get),
        Ok(100)
    );
    assert!(BatteryPercentage::new(101).is_err());
}

#[test]
fn unavailable_percentage_serializes_as_null_not_zero() {
    let component = BatteryComponent {
        component_type: ComponentType::Case,
        percentage: None,
        charging_state: ChargingState::Unknown,
        updated_at: OffsetDateTime::UNIX_EPOCH,
        source: DataSource::AirPodsAdvertisement,
        confidence: DataConfidence::High,
        stale: false,
    };
    let Ok(value) = serde_json::to_value(component) else {
        panic!("test serialization failed");
    };
    assert!(value["percentage"].is_null());
}
