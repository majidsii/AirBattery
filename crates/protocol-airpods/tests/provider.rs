//! Integration tests for `AirPods` advertisement and accessory providers.

use device_protocols::{DeviceProvider, RawObservation, VendorProtocol};
use protocol_airpods::{APPLE_MANUFACTURER_ID, AirPodsAccessoryProvider, AirPodsProvider};
use shared_models::{ComponentType, DataConfidence, DataSource};
use time::OffsetDateTime;

#[test]
fn short_apple_continuity_packets_are_not_claimed_by_the_airpods_provider() {
    let provider = AirPodsProvider;
    let observation = RawObservation::ManufacturerData {
        device_id: "airpods".to_owned(),
        company_id: APPLE_MANUFACTURER_ID,
        payload: vec![0x07; 19],
        observed_at: OffsetDateTime::UNIX_EPOCH,
    };

    assert!(!provider.supports(&observation));
}

#[tokio::test]
async fn active_accessory_report_preserves_exact_component_values() {
    let provider = AirPodsAccessoryProvider;
    let observation = RawObservation::VendorPacket {
        device_id: "airpods".to_owned(),
        protocol: VendorProtocol::AppleAccessory,
        payload: hex::decode("040004000400030201440201040147020108010c0201")
            .unwrap_or_else(|error| panic!("fixture hex failed: {error}")),
        observed_at: OffsetDateTime::UNIX_EPOCH,
    };

    let report = provider
        .parse(&observation)
        .await
        .unwrap_or_else(|error| panic!("provider failed: {error}"))
        .unwrap_or_else(|| panic!("provider returned no report"));
    let values = report
        .components
        .iter()
        .map(|component| {
            (
                component.component_type,
                component
                    .percentage
                    .map(shared_models::BatteryPercentage::get),
                component.source,
                component.confidence,
            )
        })
        .collect::<Vec<_>>();

    assert!(values.contains(&(
        ComponentType::Left,
        Some(71),
        DataSource::VendorProtocol,
        DataConfidence::Verified,
    )));
    assert!(values.contains(&(
        ComponentType::Right,
        Some(68),
        DataSource::VendorProtocol,
        DataConfidence::Verified,
    )));
    assert!(values.contains(&(
        ComponentType::Case,
        Some(12),
        DataSource::VendorProtocol,
        DataConfidence::Verified,
    )));
}

#[tokio::test]
async fn active_accessory_report_keeps_valid_earbuds_when_case_is_temporarily_unavailable() {
    let provider = AirPodsAccessoryProvider;
    let observation = RawObservation::VendorPacket {
        device_id: "airpods".to_owned(),
        protocol: VendorProtocol::AppleAccessory,
        payload: hex::decode("04000400040003020144020104014702010801ff0201")
            .unwrap_or_else(|error| panic!("fixture hex failed: {error}")),
        observed_at: OffsetDateTime::UNIX_EPOCH,
    };

    let report = provider
        .parse(&observation)
        .await
        .unwrap_or_else(|error| panic!("provider failed: {error}"))
        .unwrap_or_else(|| panic!("provider returned no report"));

    assert_eq!(report.components.len(), 3);
    let case = report
        .components
        .iter()
        .find(|component| component.component_type == ComponentType::Case)
        .unwrap_or_else(|| panic!("case component missing"));
    assert_eq!(case.percentage, None);
}

#[tokio::test]
async fn active_accessory_zero_case_marker_preserves_last_known_case_semantics() {
    let provider = AirPodsAccessoryProvider;
    let observation = RawObservation::VendorPacket {
        device_id: "airpods".to_owned(),
        protocol: VendorProtocol::AppleAccessory,
        payload: hex::decode("04000400040003020164020104016402010801000001")
            .unwrap_or_else(|error| panic!("fixture hex failed: {error}")),
        observed_at: OffsetDateTime::UNIX_EPOCH,
    };

    let report = provider
        .parse(&observation)
        .await
        .unwrap_or_else(|error| panic!("provider failed: {error}"))
        .unwrap_or_else(|| panic!("provider returned no report"));

    let case = report
        .components
        .iter()
        .find(|component| component.component_type == ComponentType::Case)
        .unwrap_or_else(|| panic!("case component missing"));
    assert_eq!(case.percentage, None);
    assert_eq!(case.charging_state, shared_models::ChargingState::Unknown);
}
