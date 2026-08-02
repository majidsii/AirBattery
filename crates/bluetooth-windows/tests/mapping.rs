//! Target-independent Windows advertisement mapping tests.

use bluetooth_windows::{WindowsAdvertisement, WindowsKnownDevice, map_windows_devices};
use device_protocols::RawObservation;
use shared_models::{ConnectionState, DeviceFamily};
use time::{Duration, OffsetDateTime};

fn airpods_payload() -> Vec<u8> {
    let mut data = vec![0_u8; 27];
    data[0] = 0x07;
    data[3] = 0x0e;
    data[5] = 0x20;
    data[6] = 0x87;
    data[7] = 0x45;
    data
}

#[test]
fn paired_device_and_advertisement_merge_by_address() {
    let now = OffsetDateTime::UNIX_EPOCH;
    let mapping = map_windows_devices(
        vec![WindowsKnownDevice {
            backend_id: "windows-device-id".to_owned(),
            address: Some(0x1122_3344_5566),
            display_name: "SHABIN".to_owned(),
            connection_state: ConnectionState::Connected,
            standard_battery: Some(90),
            observed_at: now,
        }],
        vec![WindowsAdvertisement {
            address: 0x1122_3344_5566,
            local_name: None,
            manufacturer_data: vec![(0x004c, airpods_payload())],
            service_uuids: vec!["0000180f-0000-1000-8000-00805f9b34fb".to_owned()],
            rssi: Some(-40),
            observed_at: now + Duration::seconds(1),
        }],
    );

    assert_eq!(mapping.descriptors.len(), 1);
    assert_eq!(mapping.descriptors[0].backend_id, "windows-device-id");
    assert_eq!(mapping.descriptors[0].device_family, DeviceFamily::AirPods);
    assert_eq!(
        mapping.descriptors[0].model.as_deref(),
        Some("AirPods Pro (1st generation)")
    );
    assert!(mapping.observations.iter().any(|observation| matches!(
        observation,
        RawObservation::StandardBattery { percentage: 90, .. }
    )));
    assert!(mapping.observations.iter().any(|observation| matches!(
        observation,
        RawObservation::ManufacturerData {
            company_id: 0x004c,
            ..
        }
    )));
}

#[test]
fn latest_company_payload_wins_without_inventing_connection_state() {
    let now = OffsetDateTime::UNIX_EPOCH;
    let mut older = airpods_payload();
    older[6] = 0x22;
    let newer = airpods_payload();
    let mapping = map_windows_devices(
        Vec::new(),
        vec![
            WindowsAdvertisement {
                address: 0x1122_3344_5566,
                local_name: Some("AirPods".to_owned()),
                manufacturer_data: vec![(0x004c, older)],
                service_uuids: Vec::new(),
                rssi: None,
                observed_at: now,
            },
            WindowsAdvertisement {
                address: 0x1122_3344_5566,
                local_name: Some("AirPods".to_owned()),
                manufacturer_data: vec![(0x004c, newer.clone())],
                service_uuids: Vec::new(),
                rssi: None,
                observed_at: now + Duration::seconds(1),
            },
        ],
    );

    assert_eq!(mapping.descriptors.len(), 1);
    assert!(mapping.observations.iter().any(|observation| matches!(
        observation,
        RawObservation::Connection {
            state: ConnectionState::Unknown,
            ..
        }
    )));
    let payload = mapping
        .observations
        .iter()
        .find_map(|observation| match observation {
            RawObservation::ManufacturerData { payload, .. } => Some(payload),
            _ => None,
        })
        .unwrap_or_else(|| panic!("manufacturer payload missing"));
    assert_eq!(payload, &newer);
}

#[test]
fn rotating_airpods_address_merges_into_one_connected_windows_device() {
    let now = OffsetDateTime::UNIX_EPOCH;
    let mapping = map_windows_devices(
        vec![WindowsKnownDevice {
            backend_id: "windows-device-id".to_owned(),
            address: Some(0x1111_1111_1111),
            display_name: "SHABIN".to_owned(),
            connection_state: ConnectionState::Connected,
            standard_battery: Some(90),
            observed_at: now,
        }],
        vec![WindowsAdvertisement {
            address: 0x2222_2222_2222,
            local_name: None,
            manufacturer_data: vec![(0x004c, airpods_payload())],
            service_uuids: Vec::new(),
            rssi: Some(-35),
            observed_at: now + Duration::seconds(1),
        }],
    );

    assert_eq!(mapping.descriptors.len(), 1);
    assert_eq!(mapping.descriptors[0].backend_id, "windows-device-id");
    assert_eq!(mapping.descriptors[0].display_name, "SHABIN");
    assert_eq!(mapping.descriptors[0].device_family, DeviceFamily::AirPods);
}

#[test]
fn rotating_airpods_address_is_not_guessed_with_multiple_connected_devices() {
    let now = OffsetDateTime::UNIX_EPOCH;
    let known = ["one", "two"].map(|id| WindowsKnownDevice {
        backend_id: id.to_owned(),
        address: None,
        display_name: id.to_owned(),
        connection_state: ConnectionState::Connected,
        standard_battery: Some(70),
        observed_at: now,
    });
    let mapping = map_windows_devices(
        known.into_iter().collect(),
        vec![WindowsAdvertisement {
            address: 0x2222_2222_2222,
            local_name: None,
            manufacturer_data: vec![(0x004c, airpods_payload())],
            service_uuids: Vec::new(),
            rssi: Some(-35),
            observed_at: now + Duration::seconds(1),
        }],
    );

    assert_eq!(mapping.descriptors.len(), 3);
    assert_eq!(
        mapping
            .descriptors
            .iter()
            .filter(|descriptor| descriptor.device_family == DeviceFamily::AirPods)
            .count(),
        1
    );
}
