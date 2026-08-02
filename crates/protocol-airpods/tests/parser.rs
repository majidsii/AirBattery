//! Integration tests for this crate.

use protocol_airpods::{AirPodsModel, decode_battery_nibble, parse_proximity_pairing};
use shared_models::{BatteryPercentage, ChargingState};

fn payload() -> [u8; 27] {
    let mut data = [0_u8; 27];
    data[0] = 0x07;
    data[3] = 0x0e;
    data[5] = 0x20;
    data[6] = 0x87;
    data[7] = 0x45;
    data
}

#[test]
fn decodes_deciles_and_unknown_nibbles() {
    assert_eq!(
        decode_battery_nibble(10).map(BatteryPercentage::get),
        Some(100)
    );
    assert_eq!(decode_battery_nibble(11), None);
    assert_eq!(decode_battery_nibble(15), None);
}

#[test]
fn rejects_non_proximity_and_truncated_payloads() {
    assert_eq!(parse_proximity_pairing(&[0x01; 27]), Ok(None));
    assert!(parse_proximity_pairing(&[0x07; 5]).is_err());
}

#[test]
fn parses_airpods_pro_battery_and_charging() {
    let Ok(Some(parsed)) = parse_proximity_pairing(&payload()) else {
        panic!("AirPods fixture did not parse");
    };
    assert_eq!(parsed.model, AirPodsModel::AirPodsPro);
    assert_eq!(parsed.left.percentage.map(BatteryPercentage::get), Some(70));
    assert_eq!(
        parsed.right.percentage.map(BatteryPercentage::get),
        Some(80)
    );
    assert_eq!(parsed.case.percentage.map(BatteryPercentage::get), Some(50));
    assert_eq!(parsed.case.charging_state, ChargingState::Charging);
}

#[test]
fn orientation_bit_swaps_left_and_right() {
    let mut data = payload();
    data[5] = 0x00;
    let Ok(Some(parsed)) = parse_proximity_pairing(&data) else {
        panic!("flipped fixture did not parse");
    };
    assert_eq!(parsed.left.percentage.map(BatteryPercentage::get), Some(80));
    assert_eq!(
        parsed.right.percentage.map(BatteryPercentage::get),
        Some(70)
    );
}

#[test]
fn absent_component_never_gets_a_charging_state() {
    let mut data = payload();
    data[7] = 0x4f;
    let Ok(Some(parsed)) = parse_proximity_pairing(&data) else {
        panic!("absent-case fixture did not parse");
    };
    assert_eq!(parsed.case.percentage, None);
    assert_eq!(parsed.case.charging_state, ChargingState::Unknown);
}

#[test]
fn parses_captured_airpods_pro_2020_state_transitions() {
    let fixtures = [
        // Lid open, both earbuds seated in the case.
        (
            "0719010e2055aab231000030017c73b0c995e497d4e135bad1e8f6",
            Some(100),
            Some(100),
            Some(20),
            ChargingState::Charging,
            ChargingState::Charging,
            ChargingState::NotCharging,
        ),
        // Left earbud removed, right earbud still seated and charging.
        (
            "0719010e2031aaa201000446fb26b8d77ae04e8c3ea8a33a8aab9e",
            Some(100),
            Some(100),
            Some(20),
            ChargingState::NotCharging,
            ChargingState::Charging,
            ChargingState::NotCharging,
        ),
        // Both earbuds removed; the case component is no longer advertised.
        (
            "0719010e2021aa8f010004cccd96ffb1bcf1c80194df1ecccbacc2",
            Some(100),
            Some(100),
            None,
            ChargingState::NotCharging,
            ChargingState::NotCharging,
            ChargingState::Unknown,
        ),
    ];

    for (hex, left, right, case, left_state, right_state, case_state) in fixtures {
        let bytes = hex::decode(hex).unwrap_or_else(|error| panic!("fixture hex failed: {error}"));
        let Ok(Some(parsed)) = parse_proximity_pairing(&bytes) else {
            panic!("captured AirPods Pro payload did not parse");
        };
        assert_eq!(parsed.model, AirPodsModel::AirPodsPro);
        assert_eq!(parsed.left.percentage.map(BatteryPercentage::get), left);
        assert_eq!(parsed.right.percentage.map(BatteryPercentage::get), right);
        assert_eq!(parsed.case.percentage.map(BatteryPercentage::get), case);
        assert_eq!(parsed.left.charging_state, left_state);
        assert_eq!(parsed.right.charging_state, right_state);
        assert_eq!(parsed.case.charging_state, case_state);
    }
}

#[test]
fn extended_payloads_keep_the_required_battery_fields() {
    let mut data = payload().to_vec();
    data.extend_from_slice(&[0xaa, 0xbb, 0xcc]);
    let Ok(Some(parsed)) = parse_proximity_pairing(&data) else {
        panic!("extended fixture did not parse");
    };
    assert_eq!(parsed.model, AirPodsModel::AirPodsPro);
    assert_eq!(parsed.left.percentage.map(BatteryPercentage::get), Some(70));
    assert_eq!(
        parsed.right.percentage.map(BatteryPercentage::get),
        Some(80)
    );
    assert_eq!(parsed.case.percentage.map(BatteryPercentage::get), Some(50));
}

#[test]
fn charging_bits_are_applied_to_each_present_component() {
    let mut data = payload();
    data[7] = 0x75;
    let Ok(Some(parsed)) = parse_proximity_pairing(&data) else {
        panic!("charging fixture did not parse");
    };
    assert_eq!(parsed.left.charging_state, ChargingState::Charging);
    assert_eq!(parsed.right.charging_state, ChargingState::Charging);
    assert_eq!(parsed.case.charging_state, ChargingState::Charging);
}

#[test]
fn model_labels_are_stable_for_exact_and_unknown_models() {
    assert_eq!(
        AirPodsModel::AirPodsPro.display_name(),
        "AirPods Pro (1st generation)"
    );
    assert_eq!(
        AirPodsModel::AirPodsPro2.display_name(),
        "AirPods Pro (2nd generation)"
    );
    assert_eq!(AirPodsModel::Unknown(0).display_name(), "AirPods");
}

#[test]
fn parses_exact_accessory_battery_percentages_without_decile_rounding() {
    let bytes = hex::decode("040004000400030201440201040147020108010c0201")
        .unwrap_or_else(|error| panic!("fixture hex failed: {error}"));
    let Ok(Some(parsed)) = protocol_airpods::parse_accessory_battery(&bytes) else {
        panic!("accessory battery packet did not parse");
    };

    assert_eq!(
        parsed
            .right
            .and_then(|component| component.percentage)
            .map(BatteryPercentage::get),
        Some(68)
    );
    assert_eq!(
        parsed
            .left
            .and_then(|component| component.percentage)
            .map(BatteryPercentage::get),
        Some(71)
    );
    assert_eq!(
        parsed
            .case
            .and_then(|component| component.percentage)
            .map(BatteryPercentage::get),
        Some(12)
    );
}

#[test]
fn ignores_non_battery_accessory_packets_and_rejects_truncated_records() {
    assert_eq!(
        protocol_airpods::parse_accessory_battery(&[0x04, 0x00, 0x04, 0x00, 0x09, 0x00]),
        Ok(None)
    );
    assert!(
        protocol_airpods::parse_accessory_battery(&[
            0x04, 0x00, 0x04, 0x00, 0x04, 0x00, 0x01, 0x02
        ])
        .is_err()
    );
}

#[test]
fn exact_accessory_unavailable_sentinel_does_not_reject_other_components() {
    let bytes = hex::decode("04000400040003020144020104014702010801ff0201")
        .unwrap_or_else(|error| panic!("fixture hex failed: {error}"));
    let Ok(Some(parsed)) = protocol_airpods::parse_accessory_battery(&bytes) else {
        panic!("accessory battery packet with unavailable case did not parse");
    };

    assert_eq!(
        parsed
            .right
            .and_then(|component| component.percentage)
            .map(BatteryPercentage::get),
        Some(68)
    );
    assert_eq!(
        parsed
            .left
            .and_then(|component| component.percentage)
            .map(BatteryPercentage::get),
        Some(71)
    );
    assert_eq!(parsed.case.and_then(|component| component.percentage), None);
}

#[test]
fn exact_accessory_zero_case_with_unknown_status_is_temporarily_unavailable() {
    let bytes = hex::decode("04000400040003020164020104016402010801000001")
        .unwrap_or_else(|error| panic!("fixture hex failed: {error}"));
    let Ok(Some(parsed)) = protocol_airpods::parse_accessory_battery(&bytes) else {
        panic!("accessory battery packet with an out-of-case marker did not parse");
    };

    assert_eq!(
        parsed
            .right
            .and_then(|component| component.percentage)
            .map(BatteryPercentage::get),
        Some(100)
    );
    assert_eq!(
        parsed
            .left
            .and_then(|component| component.percentage)
            .map(BatteryPercentage::get),
        Some(100)
    );
    assert_eq!(parsed.case.and_then(|component| component.percentage), None);
}
