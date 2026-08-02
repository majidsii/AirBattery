//! Integration tests for mapping `BlueZ` device data into observations.

use bluetooth_linux::{BluezDeviceSnapshot, map_bluez_snapshot};
use device_protocols::RawObservation;
use std::collections::{BTreeMap, BTreeSet};
use time::OffsetDateTime;

#[test]
fn maps_advertisements_platform_battery_and_service_data_without_invention() {
    let mut manufacturer_data = BTreeMap::new();
    manufacturer_data.insert(76, vec![0x07; 27]);
    let mut service_data = BTreeMap::new();
    service_data.insert("0000180f-0000-1000-8000-00805f9b34fb".to_owned(), vec![70]);
    let snapshot = BluezDeviceSnapshot {
        address: "AA:BB:CC:DD:EE:FF".into(),
        alias: Some("AirPods".into()),
        connected: true,
        paired: true,
        vendor_id: Some(0x004c),
        product_id: Some(0x200e),
        battery_percentage: Some(70),
        icon: Some("audio-headset".to_owned()),
        class: Some(0x0024_0418),
        appearance: Some(0x0941),
        service_uuids: BTreeSet::from(["0000180f-0000-1000-8000-00805f9b34fb".to_owned()]),
        rssi: Some(-42),
        manufacturer_data,
        service_data,
        observed_at: OffsetDateTime::UNIX_EPOCH,
    };
    let observations = map_bluez_snapshot(&snapshot);
    assert!(observations.iter().any(|observation| matches!(
        observation,
        RawObservation::ManufacturerData { company_id: 76, .. }
    )));
    assert!(observations.iter().any(|observation| matches!(
        observation,
        RawObservation::PlatformBattery { percentage: 70, .. }
    )));
    assert!(observations.iter().any(|observation| matches!(
        observation,
        RawObservation::ServiceData { service_uuid, payload, .. }
            if service_uuid == "0000180f-0000-1000-8000-00805f9b34fb" && payload == &[70]
    )));
}
