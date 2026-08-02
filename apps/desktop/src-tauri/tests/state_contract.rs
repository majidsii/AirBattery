//! Integration tests for this crate.

use std::{
    fs,
    path::PathBuf,
    process,
    time::{SystemTime, UNIX_EPOCH},
};

use airbattery_desktop::{
    BackendStatus, PlatformCollection, PlatformKind, RuntimeState, tray_snapshot,
};
use airbattery_service::DeviceDescriptor;
use airbattery_settings::{AppSettings, SettingsRepository, Theme};
use device_protocols::RawObservation;
use shared_models::{ConnectionState, DeviceFamily, Transport};
use time::OffsetDateTime;

fn settings_path(label: &str) -> PathBuf {
    let nonce = SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|duration| duration.as_nanos())
        .unwrap_or_default();
    std::env::temp_dir().join(format!("airbattery-{label}-{}-{nonce}.json", process::id()))
}

fn connected_collection(observed_at: OffsetDateTime) -> PlatformCollection {
    PlatformCollection {
        status: BackendStatus {
            platform: PlatformKind::Linux,
            available: true,
            adapter_name: Some("hci-test".to_owned()),
            powered: Some(true),
            discovering: Some(false),
            detail: "test backend".to_owned(),
        },
        descriptors: vec![DeviceDescriptor {
            backend_id: "backend-device".to_owned(),
            privacy_id: "bt-safe".to_owned(),
            display_name: "Headphones".to_owned(),
            system_name: None,
            manufacturer: None,
            model: None,
            icon: None,
            class: None,
            appearance: None,
            service_uuids: Vec::new(),
            device_family: DeviceFamily::Headset,
            transport: Transport::Dual,
            last_seen_at: Some(observed_at),
        }],
        observations: vec![RawObservation::Connection {
            device_id: "backend-device".to_owned(),
            state: ConnectionState::Connected,
            observed_at,
        }],
    }
}

#[tokio::test]
async fn duplicate_collection_does_not_emit_a_changed_snapshot() {
    let path = settings_path("dedupe");
    let state = RuntimeState::new(SettingsRepository::new(&path));
    let collection = connected_collection(OffsetDateTime::UNIX_EPOCH);

    let first = state.apply_collection(collection.clone()).await;
    let second = state.apply_collection(collection).await;

    assert!(first.devices_changed);
    assert!(!second.devices_changed);
    assert_eq!(second.devices.len(), 1);
    let _cleanup = fs::remove_file(path);
}

#[tokio::test]
async fn returned_device_snapshots_are_isolated_clones() {
    let path = settings_path("clone");
    let state = RuntimeState::new(SettingsRepository::new(&path));
    state
        .apply_collection(connected_collection(OffsetDateTime::UNIX_EPOCH))
        .await;

    let mut first = state.devices().await;
    first.clear();

    assert_eq!(state.devices().await.len(), 1);
    let _cleanup = fs::remove_file(path);
}

#[test]
fn runtime_state_uses_the_atomic_settings_repository() {
    let path = settings_path("settings");
    let state = RuntimeState::new(SettingsRepository::new(&path));
    let mut settings = AppSettings::default();
    settings.appearance.theme = Theme::Dark;

    state
        .save_settings(&settings)
        .unwrap_or_else(|error| panic!("save failed: {error}"));
    let loaded = state
        .load_settings()
        .unwrap_or_else(|error| panic!("load failed: {error}"));

    assert_eq!(loaded.appearance.theme, Theme::Dark);
    let _cleanup = fs::remove_file(path);
}

#[tokio::test]
async fn backend_status_changes_are_deduplicated_independently() {
    let path = settings_path("status-dedupe");
    let state = RuntimeState::new(SettingsRepository::new(&path));
    let collection = connected_collection(OffsetDateTime::UNIX_EPOCH);

    let first = state.apply_collection(collection.clone()).await;
    let second = state.apply_collection(collection.clone()).await;
    let mut changed_status = collection;
    changed_status.status.detail = "adapter resumed".to_owned();
    let third = state.apply_collection(changed_status).await;

    assert!(first.status_changed);
    assert!(!second.status_changed);
    assert!(!third.devices_changed);
    assert!(third.status_changed);
    let _cleanup = fs::remove_file(path);
}

#[test]
fn refresh_schedule_scans_for_short_lived_advertisements_every_tick() {
    use airbattery_desktop::scheduled_refresh_mode;

    assert_eq!(
        scheduled_refresh_mode(1),
        airbattery_desktop::RefreshMode::BoundedDiscovery
    );
    assert_eq!(
        scheduled_refresh_mode(3),
        airbattery_desktop::RefreshMode::BoundedDiscovery
    );
    assert_eq!(
        scheduled_refresh_mode(4),
        airbattery_desktop::RefreshMode::BoundedDiscovery
    );
    assert_eq!(
        scheduled_refresh_mode(8),
        airbattery_desktop::RefreshMode::BoundedDiscovery
    );
}

#[test]
fn tray_summary_prefers_split_earbuds_and_ignores_case_for_icon_percentage() {
    use shared_models::{
        BatteryComponent, BatteryPercentage, BluetoothAudioDevice, ChargingState, ComponentType,
        DataConfidence, DataSource, DeviceVisual, VisualConfidence,
    };
    use std::collections::BTreeSet;

    let value = |component_type, percentage| BatteryComponent {
        component_type,
        percentage: Some(
            BatteryPercentage::new(percentage)
                .unwrap_or_else(|error| panic!("fixture failed: {error}")),
        ),
        charging_state: ChargingState::Unknown,
        updated_at: OffsetDateTime::UNIX_EPOCH,
        source: DataSource::AirPodsAdvertisement,
        confidence: DataConfidence::High,
        stale: false,
    };
    let device = BluetoothAudioDevice {
        id: "airpods".to_owned(),
        display_name: "SHABIN".to_owned(),
        system_name: Some("SHABIN".to_owned()),
        manufacturer: Some("Apple".to_owned()),
        model: Some("AirPods Pro (1st generation)".to_owned()),
        device_family: DeviceFamily::AirPods,
        visual: DeviceVisual {
            key: "airpods-pro-1".to_owned(),
            confidence: VisualConfidence::Exact,
        },
        transport: Transport::Dual,
        connection_state: ConnectionState::Connected,
        last_seen_at: Some(OffsetDateTime::UNIX_EPOCH),
        last_updated_at: Some(OffsetDateTime::UNIX_EPOCH),
        capabilities: BTreeSet::new(),
        components: vec![
            value(ComponentType::Left, 82),
            value(ComponentType::Right, 79),
            value(ComponentType::Case, 20),
        ],
    };

    let summary = airbattery_desktop::tray_snapshot(&[device], None)
        .unwrap_or_else(|| panic!("tray snapshot missing"));
    assert_eq!(summary.label, "L 82%  R 79%  C 20%");
    assert_eq!(summary.icon_percentage, Some(79));
}

#[test]
fn tray_summary_uses_one_real_aggregate_for_airpods_when_split_data_is_unavailable() {
    use shared_models::{
        BatteryComponent, BatteryPercentage, BluetoothAudioDevice, ChargingState, ComponentType,
        DataConfidence, DataSource, DeviceVisual, VisualConfidence,
    };
    use std::collections::BTreeSet;

    let device = BluetoothAudioDevice {
        id: "airpods".to_owned(),
        display_name: "SHABIN".to_owned(),
        system_name: Some("SHABIN".to_owned()),
        manufacturer: Some("Apple".to_owned()),
        model: Some("AirPods Pro (1st generation)".to_owned()),
        device_family: DeviceFamily::AirPods,
        visual: DeviceVisual {
            key: "airpods-pro-1".to_owned(),
            confidence: VisualConfidence::Exact,
        },
        transport: Transport::Dual,
        connection_state: ConnectionState::Connected,
        last_seen_at: Some(OffsetDateTime::UNIX_EPOCH),
        last_updated_at: Some(OffsetDateTime::UNIX_EPOCH),
        capabilities: BTreeSet::new(),
        components: vec![BatteryComponent {
            component_type: ComponentType::Aggregate,
            percentage: Some(
                BatteryPercentage::new(88)
                    .unwrap_or_else(|error| panic!("fixture failed: {error}")),
            ),
            charging_state: ChargingState::Unknown,
            updated_at: OffsetDateTime::UNIX_EPOCH,
            source: DataSource::BluezBattery,
            confidence: DataConfidence::Medium,
            stale: false,
        }],
    };

    let summary = airbattery_desktop::tray_snapshot(&[device], None)
        .unwrap_or_else(|| panic!("tray snapshot missing"));
    assert_eq!(summary.label, "88%");
    assert_eq!(summary.icon_percentage, Some(88));
}

#[test]
fn tray_prefers_the_freshest_connected_battery_sender_over_an_old_preference() {
    use shared_models::{
        BatteryComponent, BatteryPercentage, BluetoothAudioDevice, ChargingState, ComponentType,
        DataConfidence, DataSource, DeviceVisual, VisualConfidence,
    };
    use std::collections::BTreeSet;
    use time::macros::datetime;

    let make_device = |id: &str, updated_at, stale: bool| BluetoothAudioDevice {
        id: id.to_owned(),
        display_name: id.to_owned(),
        system_name: Some(id.to_owned()),
        manufacturer: Some("Apple".to_owned()),
        model: Some("AirPods Pro".to_owned()),
        device_family: DeviceFamily::AirPods,
        visual: DeviceVisual {
            key: "airpods-pro-1".to_owned(),
            confidence: VisualConfidence::Exact,
        },
        transport: Transport::Dual,
        connection_state: ConnectionState::Connected,
        last_seen_at: Some(updated_at),
        last_updated_at: Some(updated_at),
        capabilities: BTreeSet::new(),
        components: vec![BatteryComponent {
            component_type: ComponentType::Left,
            percentage: Some(
                BatteryPercentage::new(68)
                    .unwrap_or_else(|error| panic!("fixture failed: {error}")),
            ),
            charging_state: ChargingState::NotCharging,
            updated_at,
            source: DataSource::VendorProtocol,
            confidence: DataConfidence::Verified,
            stale,
        }],
    };

    let old = make_device("old", datetime!(2026-08-01 17:00 UTC), true);
    let current = make_device("current", datetime!(2026-08-01 17:45 UTC), false);
    let snapshot = tray_snapshot(&[old, current], Some("old"))
        .unwrap_or_else(|| panic!("tray snapshot missing"));

    assert_eq!(snapshot.device_name, "current");
}

#[test]
fn tray_uses_fresh_exact_battery_evidence_when_bluez_connection_state_lags() {
    use shared_models::{
        BatteryComponent, BatteryPercentage, BluetoothAudioDevice, ChargingState, ComponentType,
        DataConfidence, DataSource, DeviceVisual, VisualConfidence,
    };
    use std::collections::BTreeSet;
    use time::macros::datetime;

    let old = BluetoothAudioDevice {
        id: "old".to_owned(),
        display_name: "ACEFAST N2".to_owned(),
        system_name: Some("ACEFAST N2".to_owned()),
        manufacturer: None,
        model: None,
        device_family: DeviceFamily::Headset,
        visual: DeviceVisual {
            key: "headset-generic".to_owned(),
            confidence: VisualConfidence::Category,
        },
        transport: Transport::Dual,
        connection_state: ConnectionState::Disconnected,
        last_seen_at: None,
        last_updated_at: None,
        capabilities: BTreeSet::new(),
        components: Vec::new(),
    };
    let updated_at = datetime!(2026-08-02 06:50 UTC);
    let current = BluetoothAudioDevice {
        id: "current".to_owned(),
        display_name: "SHABIN".to_owned(),
        system_name: Some("SHABIN".to_owned()),
        manufacturer: Some("Apple".to_owned()),
        model: Some("AirPods Pro".to_owned()),
        device_family: DeviceFamily::AirPods,
        visual: DeviceVisual {
            key: "airpods-pro-1".to_owned(),
            confidence: VisualConfidence::Exact,
        },
        transport: Transport::Dual,
        connection_state: ConnectionState::Disconnected,
        last_seen_at: Some(updated_at),
        last_updated_at: Some(updated_at),
        capabilities: BTreeSet::new(),
        components: vec![BatteryComponent {
            component_type: ComponentType::Left,
            percentage: Some(
                BatteryPercentage::new(71)
                    .unwrap_or_else(|error| panic!("fixture failed: {error}")),
            ),
            charging_state: ChargingState::NotCharging,
            updated_at,
            source: DataSource::VendorProtocol,
            confidence: DataConfidence::Verified,
            stale: false,
        }],
    };

    let snapshot = tray_snapshot(&[old, current], Some("old"))
        .unwrap_or_else(|| panic!("tray snapshot missing"));
    assert_eq!(snapshot.device_name, "SHABIN");
    assert_eq!(snapshot.icon_percentage, Some(71));
}
