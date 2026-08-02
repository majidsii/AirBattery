//! Target-independent Windows Bluetooth observation mapping.

use std::collections::{BTreeMap, BTreeSet};

use airbattery_service::DeviceDescriptor;
use device_protocols::RawObservation;
use diagnostics::sanitize_identifier;
use protocol_airpods::{APPLE_MANUFACTURER_ID, parse_proximity_pairing};
use shared_models::{ConnectionState, DeviceFamily, Transport};
use time::OffsetDateTime;

/// Paired or operating-system-known Windows BLE device.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowsKnownDevice {
    /// Windows Runtime device identifier.
    pub backend_id: String,
    /// Bluetooth address used to merge advertisements.
    pub address: Option<u64>,
    /// User-visible operating-system name.
    pub display_name: String,
    /// Current connection state.
    pub connection_state: ConnectionState,
    /// Standard Battery Service value when exposed.
    pub standard_battery: Option<u8>,
    /// Observation time.
    pub observed_at: OffsetDateTime,
}

/// One BLE advertisement received by the Windows watcher.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct WindowsAdvertisement {
    /// Bluetooth address reported by the watcher.
    pub address: u64,
    /// Advertisement local name when present.
    pub local_name: Option<String>,
    /// Company identifier and manufacturer payload pairs.
    pub manufacturer_data: Vec<(u16, Vec<u8>)>,
    /// Advertised service UUIDs.
    pub service_uuids: Vec<String>,
    /// Received signal strength.
    pub rssi: Option<i16>,
    /// Observation time.
    pub observed_at: OffsetDateTime,
}

/// Mapped descriptors and observations consumed by the application engine.
#[derive(Debug, Default)]
pub struct WindowsMapping {
    /// Device metadata merged by Bluetooth address.
    pub descriptors: Vec<DeviceDescriptor>,
    /// Connection, standard battery, and advertisement observations.
    pub observations: Vec<RawObservation>,
}

#[derive(Debug)]
struct MergedDevice {
    backend_id: String,
    display_name: String,
    connection_state: ConnectionState,
    standard_battery: Option<u8>,
    manufacturer_data: BTreeMap<u16, (Vec<u8>, OffsetDateTime)>,
    service_uuids: BTreeSet<String>,
    observed_at: OffsetDateTime,
}

/// Merges paired devices with the latest advertisement values per company id.
#[must_use]
#[allow(clippy::too_many_lines)]
pub fn map_windows_devices(
    known_devices: Vec<WindowsKnownDevice>,
    advertisements: Vec<WindowsAdvertisement>,
) -> WindowsMapping {
    let mut merged: BTreeMap<String, MergedDevice> = BTreeMap::new();
    let mut address_to_key = BTreeMap::new();
    let mut connected_keys = Vec::new();

    for known in known_devices {
        let key = known.backend_id.clone();
        if let Some(address) = known.address {
            address_to_key.insert(address, key.clone());
        }
        if known.connection_state == ConnectionState::Connected {
            connected_keys.push(key.clone());
        }
        merged.insert(
            key.clone(),
            MergedDevice {
                backend_id: key,
                display_name: nonempty_name(Some(known.display_name.as_str()))
                    .unwrap_or("Bluetooth LE device")
                    .to_owned(),
                connection_state: known.connection_state,
                standard_battery: known.standard_battery.filter(|value| *value <= 100),
                manufacturer_data: BTreeMap::new(),
                service_uuids: BTreeSet::new(),
                observed_at: known.observed_at,
            },
        );
    }

    let mut advertisements = advertisements;
    correlate_unique_airpods_advertisement(
        &mut advertisements,
        &mut address_to_key,
        &connected_keys,
    );

    for advertisement in advertisements {
        let key = address_to_key
            .get(&advertisement.address)
            .cloned()
            .unwrap_or_else(|| advertisement_backend_id(advertisement.address));
        let entry = merged.entry(key.clone()).or_insert_with(|| MergedDevice {
            backend_id: key,
            display_name: nonempty_name(advertisement.local_name.as_deref())
                .unwrap_or("Nearby Bluetooth device")
                .to_owned(),
            connection_state: ConnectionState::Unknown,
            standard_battery: None,
            manufacturer_data: BTreeMap::new(),
            service_uuids: BTreeSet::new(),
            observed_at: advertisement.observed_at,
        });

        if let Some(local_name) = nonempty_name(advertisement.local_name.as_deref())
            && (entry.display_name == "Bluetooth LE device"
                || entry.display_name == "Nearby Bluetooth device")
        {
            local_name.clone_into(&mut entry.display_name);
        }
        entry.observed_at = entry.observed_at.max(advertisement.observed_at);
        entry.service_uuids.extend(
            advertisement
                .service_uuids
                .into_iter()
                .map(|uuid| uuid.to_ascii_lowercase()),
        );
        for (company_id, payload) in advertisement.manufacturer_data {
            let replace = entry
                .manufacturer_data
                .get(&company_id)
                .is_none_or(|(_, observed_at)| advertisement.observed_at >= *observed_at);
            if replace {
                entry
                    .manufacturer_data
                    .insert(company_id, (payload, advertisement.observed_at));
            }
        }
    }

    let mut mapping = WindowsMapping::default();
    for device in merged.into_values() {
        let airpods = device
            .manufacturer_data
            .get(&APPLE_MANUFACTURER_ID)
            .and_then(|(payload, _)| parse_proximity_pairing(payload).ok().flatten());
        let model = airpods
            .as_ref()
            .map(|parsed| parsed.model.display_name().to_owned());
        let manufacturer = airpods.as_ref().map(|_| "Apple".to_owned());
        let display_name = if device.display_name == "Nearby Bluetooth device" {
            model.clone().unwrap_or(device.display_name)
        } else {
            device.display_name
        };
        let family = if airpods.is_some() {
            DeviceFamily::AirPods
        } else {
            family_hint(&display_name)
        };

        mapping.descriptors.push(DeviceDescriptor {
            privacy_id: sanitize_identifier(&device.backend_id),
            backend_id: device.backend_id.clone(),
            display_name: display_name.clone(),
            system_name: Some(display_name),
            manufacturer,
            model,
            icon: None,
            class: None,
            appearance: None,
            service_uuids: device.service_uuids.into_iter().collect(),
            device_family: family,
            transport: Transport::LowEnergy,
            last_seen_at: Some(device.observed_at),
        });
        mapping.observations.push(RawObservation::Connection {
            device_id: device.backend_id.clone(),
            state: device.connection_state,
            observed_at: device.observed_at,
        });
        if let Some(percentage) = device.standard_battery {
            mapping.observations.push(RawObservation::StandardBattery {
                device_id: device.backend_id.clone(),
                percentage,
                observed_at: device.observed_at,
            });
        }
        for (company_id, (payload, observed_at)) in device.manufacturer_data {
            mapping.observations.push(RawObservation::ManufacturerData {
                device_id: device.backend_id.clone(),
                company_id,
                payload,
                observed_at,
            });
        }
    }

    mapping
}

fn is_airpods_advertisement(advertisement: &WindowsAdvertisement) -> bool {
    advertisement
        .manufacturer_data
        .iter()
        .any(|(company_id, payload)| {
            *company_id == APPLE_MANUFACTURER_ID
                && matches!(parse_proximity_pairing(payload), Ok(Some(_)))
        })
}

/// Associates a rotating continuity address only when Windows reports exactly
/// one connected paired device and no exact address match exists.
fn correlate_unique_airpods_advertisement(
    advertisements: &mut Vec<WindowsAdvertisement>,
    address_to_key: &mut BTreeMap<u64, String>,
    connected_keys: &[String],
) {
    if connected_keys.len() != 1 {
        return;
    }

    let unmatched = advertisements
        .iter()
        .filter(|advertisement| {
            !address_to_key.contains_key(&advertisement.address)
                && is_airpods_advertisement(advertisement)
        })
        .collect::<Vec<_>>();
    let Some(selected_address) = unmatched
        .into_iter()
        .max_by_key(|advertisement| {
            (
                advertisement.rssi.unwrap_or(i16::MIN),
                advertisement.observed_at,
            )
        })
        .map(|advertisement| advertisement.address)
    else {
        return;
    };

    advertisements.retain(|advertisement| {
        !is_airpods_advertisement(advertisement)
            || address_to_key.contains_key(&advertisement.address)
            || advertisement.address == selected_address
    });
    address_to_key.insert(selected_address, connected_keys[0].clone());
}

fn nonempty_name(value: Option<&str>) -> Option<&str> {
    value.map(str::trim).filter(|value| !value.is_empty())
}

fn advertisement_backend_id(address: u64) -> String {
    format!("ble:{address:012x}")
}

fn family_hint(name: &str) -> DeviceFamily {
    let normalized = name.to_ascii_lowercase();
    if normalized.contains("buds")
        || normalized.contains("earbud")
        || normalized.contains("tws")
        || normalized.contains("liberty")
    {
        DeviceFamily::Earbuds
    } else if normalized.contains("speaker") || normalized.contains("jbl charge") {
        DeviceFamily::Speaker
    } else if normalized.contains("mouse") || normalized.contains("mx master") {
        DeviceFamily::Mouse
    } else if normalized.contains("keyboard") || normalized.contains("keychron") {
        DeviceFamily::Keyboard
    } else if normalized.contains("controller")
        || normalized.contains("gamepad")
        || normalized.contains("joystick")
    {
        DeviceFamily::GameController
    } else if normalized.contains("stylus") || normalized.contains("pencil") {
        DeviceFamily::Stylus
    } else if normalized.contains("headset")
        || normalized.contains("headphone")
        || normalized.starts_with("wh-")
    {
        DeviceFamily::Headset
    } else {
        DeviceFamily::GenericBle
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn address_backend_id_is_stable_and_privacy_scoped() {
        assert_eq!(
            advertisement_backend_id(0x0011_2233_4455),
            "ble:001122334455"
        );
    }

    #[test]
    fn airpods_model_names_come_from_protocol_evidence() {
        assert_eq!(
            protocol_airpods::AirPodsModel::AirPodsPro.display_name(),
            "AirPods Pro (1st generation)"
        );
    }
}
