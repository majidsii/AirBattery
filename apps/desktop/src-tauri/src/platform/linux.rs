//! `BlueZ` desktop collection with bounded discovery.

use std::{collections::BTreeMap, time::Duration};

use airbattery_service::DeviceDescriptor;
use bluetooth_linux::{
    BluezDeviceSnapshot, LinuxBluetoothBackend, latest_apple_accessory_battery, map_bluez_snapshot,
};
use device_protocols::{RawObservation, VendorProtocol};
use diagnostics::sanitize_identifier;
use protocol_airpods::{APPLE_MANUFACTURER_ID, parse_proximity_pairing};
use shared_models::{DeviceFamily, Transport};

use crate::{
    error::CommandError,
    model::{BackendStatus, PlatformCollection, PlatformKind, RefreshMode},
};

const DISCOVERY_WINDOW: Duration = Duration::from_secs(2);
const DIAGNOSTIC_DISCOVERY_WINDOW: Duration = Duration::from_secs(30);

#[allow(clippy::too_many_lines)]
pub async fn collect(mode: RefreshMode) -> Result<PlatformCollection, CommandError> {
    let backend = match LinuxBluetoothBackend::new().await {
        Ok(backend) => backend,
        Err(error) => {
            tracing::warn!(error = %error, "BlueZ session is unavailable");
            return Ok(unavailable(
                "BlueZ is unavailable or no Bluetooth adapter was found.",
            ));
        }
    };
    let native_status = match backend.status().await {
        Ok(status) => status,
        Err(error) => {
            tracing::warn!(error = %error, "BlueZ adapter status query failed");
            return Ok(unavailable(
                "The Bluetooth adapter status could not be read.",
            ));
        }
    };

    let mut snapshots = match backend.known_devices().await {
        Ok(values) => index_snapshots(values),
        Err(error) => {
            tracing::warn!(error = %error, "BlueZ known-device query failed");
            return Ok(PlatformCollection {
                status: BackendStatus {
                    platform: PlatformKind::Linux,
                    available: false,
                    adapter_name: Some(native_status.adapter_name),
                    powered: Some(native_status.powered),
                    discovering: Some(native_status.discovering),
                    detail: "Known Bluetooth devices could not be read.".to_owned(),
                },
                descriptors: Vec::new(),
                observations: Vec::new(),
            });
        }
    };

    let mut discovery_failed = false;
    let discovery_window = match mode {
        RefreshMode::KnownOnly => None,
        RefreshMode::BoundedDiscovery => Some(DISCOVERY_WINDOW),
        RefreshMode::DiagnosticDiscovery => Some(DIAGNOSTIC_DISCOVERY_WINDOW),
    };
    if let Some(discovery_window) = discovery_window
        && native_status.powered
        && !native_status.discovering
    {
        match backend.scan_window(discovery_window).await {
            Ok(discovered) => snapshots.extend(index_snapshots(discovered)),
            Err(error) => {
                discovery_failed = true;
                tracing::warn!(error = %error, "bounded BlueZ discovery failed");
            }
        }
    }

    correlate_airpods_advertisements(&mut snapshots);

    let mut exact_packets = BTreeMap::new();
    for snapshot in snapshots
        .values()
        .filter(|snapshot| snapshot.connected && is_paired_apple_audio_candidate(snapshot))
    {
        if let Some(packet) = latest_apple_accessory_battery(&snapshot.address).await {
            exact_packets.insert(snapshot.address.clone(), packet);
        }
    }

    let mut descriptors = Vec::with_capacity(snapshots.len());
    let mut observations = Vec::new();
    for snapshot in snapshots.into_values() {
        descriptors.push(descriptor(&snapshot));
        observations.extend(map_bluez_snapshot(&snapshot));
        if let Some(packet) = exact_packets.remove(&snapshot.address) {
            observations.push(RawObservation::VendorPacket {
                device_id: snapshot.address.clone(),
                protocol: VendorProtocol::AppleAccessory,
                payload: packet.payload,
                observed_at: packet.observed_at,
            });
        }
    }

    let detail = if !native_status.powered {
        "Bluetooth is available but powered off."
    } else if discovery_failed {
        "BlueZ is available, but the bounded advertisement refresh failed. Known-device data is retained."
    } else if matches!(mode, RefreshMode::DiagnosticDiscovery) {
        "BlueZ is available; a thirty-second diagnostic advertisement scan completed."
    } else {
        "BlueZ is available; automatic advertisement scans are bounded to two seconds."
    };

    Ok(PlatformCollection {
        status: BackendStatus {
            platform: PlatformKind::Linux,
            available: true,
            adapter_name: Some(native_status.adapter_name),
            powered: Some(native_status.powered),
            discovering: Some(native_status.discovering),
            detail: detail.to_owned(),
        },
        descriptors,
        observations,
    })
}

fn unavailable(detail: &str) -> PlatformCollection {
    PlatformCollection {
        status: BackendStatus::unavailable(PlatformKind::Linux, detail),
        descriptors: Vec::new(),
        observations: Vec::new(),
    }
}

fn index_snapshots(snapshots: Vec<BluezDeviceSnapshot>) -> BTreeMap<String, BluezDeviceSnapshot> {
    snapshots
        .into_iter()
        .map(|snapshot| (snapshot.address.clone(), snapshot))
        .collect()
}

fn apple_model_from_product(product_id: Option<u32>) -> Option<&'static str> {
    match product_id {
        // Captured Device ID for AirPods Pro (2020 / first generation).
        Some(0x200e) => Some("AirPods Pro (1st generation)"),
        _ => None,
    }
}

fn descriptor(snapshot: &BluezDeviceSnapshot) -> DeviceDescriptor {
    let airpods = snapshot
        .manufacturer_data
        .get(&APPLE_MANUFACTURER_ID)
        .and_then(|payload| parse_proximity_pairing(payload).ok().flatten());
    let is_airpods_identity = airpods.is_some() || is_paired_apple_audio_candidate(snapshot);
    let model = airpods
        .as_ref()
        .map(|value| value.model.display_name().to_owned())
        .or_else(|| apple_model_from_product(snapshot.product_id).map(ToOwned::to_owned));
    let alias = snapshot
        .alias
        .as_deref()
        .map(str::trim)
        .filter(|name| !name.is_empty() && *name != snapshot.address);
    let display_name = alias
        .map(ToOwned::to_owned)
        .or_else(|| model.clone())
        .unwrap_or_else(|| "Bluetooth audio device".to_owned());

    DeviceDescriptor {
        backend_id: snapshot.address.clone(),
        privacy_id: sanitize_identifier(&snapshot.address),
        display_name,
        system_name: alias.map(ToOwned::to_owned),
        manufacturer: is_airpods_identity.then(|| "Apple".to_owned()),
        model,
        icon: snapshot.icon.clone(),
        class: snapshot.class,
        appearance: snapshot.appearance,
        service_uuids: snapshot.service_uuids.iter().cloned().collect(),
        device_family: if is_airpods_identity {
            DeviceFamily::AirPods
        } else {
            generic_family(snapshot, alias)
        },
        transport: Transport::Unknown,
        last_seen_at: Some(snapshot.observed_at),
    }
}

fn generic_family(snapshot: &BluezDeviceSnapshot, alias: Option<&str>) -> DeviceFamily {
    let normalized = alias.unwrap_or_default().to_ascii_lowercase();
    let icon = snapshot
        .icon
        .as_deref()
        .unwrap_or_default()
        .to_ascii_lowercase();
    if normalized.contains("buds") || normalized.contains("earbud") || normalized.contains("tws") {
        DeviceFamily::Earbuds
    } else if normalized.contains("speaker")
        || normalized.contains("jbl charge")
        || icon.contains("speaker")
    {
        DeviceFamily::Speaker
    } else if normalized.contains("mouse")
        || normalized.contains("mx master")
        || icon.contains("mouse")
    {
        DeviceFamily::Mouse
    } else if normalized.contains("keyboard")
        || normalized.contains("keychron")
        || icon.contains("keyboard")
    {
        DeviceFamily::Keyboard
    } else if normalized.contains("controller")
        || normalized.contains("gamepad")
        || normalized.contains("joystick")
    {
        DeviceFamily::GameController
    } else if normalized.contains("stylus")
        || normalized.contains("pencil")
        || normalized.contains("digital pen")
    {
        DeviceFamily::Stylus
    } else if normalized.contains("headset")
        || normalized.contains("headphone")
        || icon.contains("headset")
        || icon.contains("headphones")
    {
        DeviceFamily::Headset
    } else if snapshot.battery_percentage.is_some() || snapshot.connected || snapshot.paired {
        DeviceFamily::GenericBle
    } else {
        DeviceFamily::Unknown
    }
}

fn has_airpods_payload(snapshot: &BluezDeviceSnapshot) -> bool {
    snapshot
        .manufacturer_data
        .get(&APPLE_MANUFACTURER_ID)
        .is_some_and(|payload| matches!(parse_proximity_pairing(payload), Ok(Some(_))))
}

fn is_connected_audio_candidate(snapshot: &BluezDeviceSnapshot) -> bool {
    snapshot.connected && !has_airpods_payload(snapshot) && is_audio_device(snapshot)
}

fn is_audio_device(snapshot: &BluezDeviceSnapshot) -> bool {
    if has_airpods_payload(snapshot) {
        return false;
    }

    let name = snapshot
        .alias
        .as_deref()
        .unwrap_or_default()
        .to_ascii_lowercase();
    let icon = snapshot
        .icon
        .as_deref()
        .unwrap_or_default()
        .to_ascii_lowercase();
    let audio_class = snapshot
        .class
        .is_some_and(|class| ((class >> 8) & 0x1f) == 0x04);
    let audio_service = snapshot.service_uuids.iter().any(|uuid| {
        ["0000110b", "0000110e", "0000111e", "0000111f"]
            .iter()
            .any(|prefix| uuid.starts_with(prefix))
    });

    name.contains("airpods")
        || name.contains("beats")
        || name.contains("earbud")
        || name.contains("buds")
        || name.contains("headset")
        || name.contains("headphone")
        || icon.contains("audio")
        || icon.contains("headset")
        || icon.contains("headphones")
        || audio_class
        || audio_service
}

fn is_paired_apple_audio_candidate(snapshot: &BluezDeviceSnapshot) -> bool {
    if !snapshot.paired {
        return false;
    }

    let name = snapshot
        .alias
        .as_deref()
        .unwrap_or_default()
        .to_ascii_lowercase();
    let apple_identity = name.contains("airpods")
        || name.contains("beats")
        || snapshot.vendor_id == Some(u32::from(APPLE_MANUFACTURER_ID));
    apple_identity && (is_audio_device(snapshot) || has_airpods_payload(snapshot))
}

fn unique_correlation_target(snapshots: &BTreeMap<String, BluezDeviceSnapshot>) -> Option<String> {
    let paired_apple_candidates = snapshots
        .iter()
        .filter(|(_address, snapshot)| is_paired_apple_audio_candidate(snapshot))
        .map(|(address, _snapshot)| address.clone())
        .collect::<Vec<_>>();
    if paired_apple_candidates.len() == 1 {
        return paired_apple_candidates.into_iter().next();
    }
    if !paired_apple_candidates.is_empty() {
        return None;
    }

    let connected_audio_candidates = snapshots
        .iter()
        .filter(|(_address, snapshot)| is_connected_audio_candidate(snapshot))
        .map(|(address, _snapshot)| address.clone())
        .collect::<Vec<_>>();
    if connected_audio_candidates.len() == 1 {
        connected_audio_candidates.into_iter().next()
    } else {
        None
    }
}

/// Correlates a rotating Apple continuity advertisement with one unambiguous
/// paired Apple audio identity, falling back to one connected audio device.
/// Ambiguous sessions stay separate rather than assigning a nearby person's
/// battery to the wrong device.
fn correlate_airpods_advertisements(snapshots: &mut BTreeMap<String, BluezDeviceSnapshot>) {
    let Some(target_key) = unique_correlation_target(snapshots) else {
        return;
    };

    let advertisement_keys = snapshots
        .iter()
        .filter(|(_address, snapshot)| {
            !snapshot.connected && !snapshot.paired && has_airpods_payload(snapshot)
        })
        .map(|(address, _snapshot)| address.clone())
        .collect::<Vec<_>>();
    let Some(selected_key) = advertisement_keys
        .iter()
        .max_by_key(|address| {
            snapshots
                .get(*address)
                .map(|snapshot| (snapshot.rssi.unwrap_or(i16::MIN), snapshot.observed_at))
        })
        .cloned()
    else {
        return;
    };
    let Some(selected) = snapshots.get(&selected_key).cloned() else {
        return;
    };

    for address in advertisement_keys {
        snapshots.remove(&address);
    }

    let Some(candidate) = snapshots.get_mut(&target_key) else {
        return;
    };
    candidate
        .manufacturer_data
        .extend(selected.manufacturer_data);
    candidate.service_data.extend(selected.service_data);
    candidate.service_uuids.extend(selected.service_uuids);
    candidate.rssi = selected.rssi.or(candidate.rssi);
    candidate.observed_at = candidate.observed_at.max(selected.observed_at);
}

#[cfg(test)]
mod tests {
    use std::collections::{BTreeMap, BTreeSet};

    use time::OffsetDateTime;

    use super::*;

    fn snapshot(address: &str, connected: bool, icon: Option<&str>) -> BluezDeviceSnapshot {
        BluezDeviceSnapshot {
            address: address.to_owned(),
            alias: Some(if connected { "My AirPods" } else { address }.to_owned()),
            connected,
            paired: connected,
            vendor_id: None,
            product_id: None,
            battery_percentage: connected.then_some(80),
            icon: icon.map(ToOwned::to_owned),
            class: None,
            appearance: None,
            service_uuids: BTreeSet::new(),
            rssi: connected.then_some(-45),
            manufacturer_data: BTreeMap::new(),
            service_data: BTreeMap::new(),
            observed_at: OffsetDateTime::UNIX_EPOCH,
        }
    }

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
    fn unique_connected_audio_device_receives_rotating_airpods_payload() {
        let mut connected = snapshot("known", true, Some("audio-headset"));
        connected.alias = Some("My AirPods".to_owned());
        let mut advertisement = snapshot("rotating", false, None);
        advertisement.alias = None;
        advertisement.rssi = Some(-38);
        advertisement
            .manufacturer_data
            .insert(APPLE_MANUFACTURER_ID, airpods_payload());
        let mut snapshots = BTreeMap::from([
            (connected.address.clone(), connected),
            (advertisement.address.clone(), advertisement),
        ]);

        correlate_airpods_advertisements(&mut snapshots);

        assert_eq!(snapshots.len(), 1);
        let merged = snapshots
            .get("known")
            .unwrap_or_else(|| panic!("connected device missing"));
        assert!(has_airpods_payload(merged));
        assert_eq!(merged.alias.as_deref(), Some("My AirPods"));
    }

    #[test]
    fn paired_apple_audio_identity_receives_payload_while_disconnected() {
        let mut known = snapshot("known", false, Some("audio-headphones"));
        known.alias = Some("My AirPods".to_owned());
        known.paired = true;
        known.vendor_id = Some(u32::from(APPLE_MANUFACTURER_ID));

        let mut advertisement = snapshot("rotating", false, None);
        advertisement.alias = None;
        advertisement.rssi = Some(-38);
        advertisement
            .manufacturer_data
            .insert(APPLE_MANUFACTURER_ID, airpods_payload());
        let mut snapshots = BTreeMap::from([
            (known.address.clone(), known),
            (advertisement.address.clone(), advertisement),
        ]);

        correlate_airpods_advertisements(&mut snapshots);

        assert_eq!(snapshots.len(), 1);
        assert!(has_airpods_payload(&snapshots["known"]));
    }

    #[test]
    fn paired_apple_identity_wins_when_multiple_audio_devices_are_connected() {
        let mut airpods = snapshot("airpods", true, Some("audio-headphones"));
        airpods.alias = Some("My AirPods".to_owned());
        airpods.vendor_id = Some(u32::from(APPLE_MANUFACTURER_ID));
        let mut speaker = snapshot("speaker", true, Some("audio-card"));
        speaker.alias = Some("Desk Speaker".to_owned());

        let mut advertisement = snapshot("rotating", false, None);
        advertisement.alias = None;
        advertisement.rssi = Some(-38);
        advertisement
            .manufacturer_data
            .insert(APPLE_MANUFACTURER_ID, airpods_payload());
        let mut snapshots = BTreeMap::from([
            (airpods.address.clone(), airpods),
            (speaker.address.clone(), speaker),
            (advertisement.address.clone(), advertisement),
        ]);

        correlate_airpods_advertisements(&mut snapshots);

        assert_eq!(snapshots.len(), 2);
        assert!(has_airpods_payload(&snapshots["airpods"]));
        assert!(!has_airpods_payload(&snapshots["speaker"]));
    }

    #[test]
    fn multiple_paired_apple_devices_are_never_guessed() {
        let mut first = snapshot("first", false, Some("audio-headphones"));
        first.alias = Some("AirPods One".to_owned());
        first.paired = true;
        first.vendor_id = Some(u32::from(APPLE_MANUFACTURER_ID));

        let mut second = snapshot("second", false, Some("audio-headphones"));
        second.alias = Some("AirPods Two".to_owned());
        second.paired = true;
        second.vendor_id = Some(u32::from(APPLE_MANUFACTURER_ID));

        let mut advertisement = snapshot("rotating", false, None);
        advertisement.alias = None;
        advertisement
            .manufacturer_data
            .insert(APPLE_MANUFACTURER_ID, airpods_payload());
        let mut snapshots = BTreeMap::from([
            (first.address.clone(), first),
            (second.address.clone(), second),
            (advertisement.address.clone(), advertisement),
        ]);

        correlate_airpods_advertisements(&mut snapshots);

        assert_eq!(snapshots.len(), 3);
        assert!(!has_airpods_payload(&snapshots["first"]));
        assert!(!has_airpods_payload(&snapshots["second"]));
    }

    #[test]
    fn ambiguous_connected_audio_devices_are_never_guessed() {
        let first = snapshot("one", true, Some("audio-headset"));
        let second = snapshot("two", true, Some("audio-headset"));
        let mut advertisement = snapshot("rotating", false, None);
        advertisement
            .manufacturer_data
            .insert(APPLE_MANUFACTURER_ID, airpods_payload());
        let mut snapshots = BTreeMap::from([
            (first.address.clone(), first),
            (second.address.clone(), second),
            (advertisement.address.clone(), advertisement),
        ]);

        correlate_airpods_advertisements(&mut snapshots);

        assert_eq!(snapshots.len(), 3);
        assert!(!has_airpods_payload(&snapshots["one"]));
        assert!(!has_airpods_payload(&snapshots["two"]));
    }
}
