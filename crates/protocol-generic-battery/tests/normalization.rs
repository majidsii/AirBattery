//! Integration tests for generic battery normalization.

use protocol_generic_battery::{normalize_platform_battery, normalize_standard_battery};
use shared_models::{ChargingState, ComponentType};
use time::OffsetDateTime;

#[test]
fn platform_and_standard_sources_keep_capabilities_accurate() {
    let Ok(platform) = normalize_platform_battery("dev", 44, OffsetDateTime::UNIX_EPOCH) else {
        panic!("valid platform percentage was rejected");
    };
    assert_eq!(
        platform.components[0].component_type,
        ComponentType::Aggregate
    );
    assert_eq!(
        platform.components[0].charging_state,
        ChargingState::Unknown
    );

    let Ok(standard) = normalize_standard_battery("dev", 55, OffsetDateTime::UNIX_EPOCH) else {
        panic!("valid standard percentage was rejected");
    };
    assert_eq!(
        standard.components[0].component_type,
        ComponentType::Headset
    );
    assert!(normalize_standard_battery("dev", 101, OffsetDateTime::UNIX_EPOCH).is_err());
}
