//! Integration tests for D-Bus snapshot serialization and validation.

use std::collections::BTreeSet;

use airbattery_dbus::{BackendSnapshot, IPC_SCHEMA_VERSION, ServiceSnapshot, SnapshotError};
use shared_models::{
    BatteryComponent, BatteryPercentage, BluetoothAudioDevice, Capability, ChargingState,
    ComponentType, ConnectionState, DataConfidence, DataSource, DeviceFamily, Transport,
};
use time::{OffsetDateTime, macros::datetime};

fn fixture_device() -> BluetoothAudioDevice {
    let percentage = BatteryPercentage::new(42)
        .unwrap_or_else(|error| panic!("fixture percentage failed: {error}"));
    BluetoothAudioDevice {
        id: "bt-safe-device".to_owned(),
        display_name: "AirPods Pro".to_owned(),
        system_name: Some("AirPods Pro".to_owned()),
        manufacturer: Some("Apple".to_owned()),
        model: Some("AirPods Pro (1st generation)".to_owned()),
        device_family: DeviceFamily::AirPods,
        visual: shared_models::DeviceVisual {
            key: "airpods-pro-1".to_owned(),
            confidence: shared_models::VisualConfidence::Exact,
        },
        transport: Transport::Dual,
        connection_state: ConnectionState::Connected,
        last_seen_at: Some(OffsetDateTime::UNIX_EPOCH),
        last_updated_at: Some(OffsetDateTime::UNIX_EPOCH),
        capabilities: BTreeSet::from([
            Capability::EarbudBattery,
            Capability::CaseBattery,
            Capability::ChargingState,
        ]),
        components: vec![BatteryComponent {
            component_type: ComponentType::Left,
            percentage: Some(percentage),
            charging_state: ChargingState::NotCharging,
            updated_at: OffsetDateTime::UNIX_EPOCH,
            source: DataSource::AirPodsAdvertisement,
            confidence: DataConfidence::High,
            stale: false,
        }],
    }
}

fn fixture() -> ServiceSnapshot {
    ServiceSnapshot::new(
        datetime!(2026-07-29 12:00 UTC),
        Some(BackendSnapshot {
            platform: "linux".to_owned(),
            available: true,
            adapter_name: Some("Primary Bluetooth adapter".to_owned()),
            powered: Some(true),
            discovering: Some(false),
            detail: "BlueZ backend ready".to_owned(),
        }),
        vec![fixture_device()],
        Some("bt-safe-device".to_owned()),
    )
}

#[test]
fn round_trips_versioned_snapshot_with_rfc3339_timestamps() {
    let snapshot = fixture();
    let encoded = snapshot
        .to_json()
        .unwrap_or_else(|error| panic!("snapshot encoding failed: {error}"));
    assert!(encoded.contains(&format!("\"schemaVersion\":{IPC_SCHEMA_VERSION}")));
    assert!(encoded.contains("\"generatedAt\":\"2026-07-29T12:00:00Z\""));
    assert!(encoded.contains("\"updatedAt\":\"1970-01-01T00:00:00Z\""));
    assert!(encoded.contains("\"lastSeenAt\":\"1970-01-01T00:00:00Z\""));
    assert!(encoded.contains("\"preferredDeviceId\":\"bt-safe-device\""));

    let decoded = ServiceSnapshot::from_json(&encoded)
        .unwrap_or_else(|error| panic!("snapshot decoding failed: {error}"));
    assert_eq!(decoded, snapshot);
}

#[test]
fn rejects_unsupported_snapshot_schema() {
    let encoded = r#"{"schemaVersion":99,"generatedAt":null,"backend":null,"devices":[],"preferredDeviceId":null}"#;
    let error = ServiceSnapshot::from_json(encoded)
        .err()
        .unwrap_or_else(|| panic!("unsupported schema was accepted"));
    assert!(matches!(error, SnapshotError::UnsupportedSchema(99)));
}
