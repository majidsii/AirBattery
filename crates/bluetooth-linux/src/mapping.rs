//! Conversion from `BlueZ` properties into platform-neutral observations.

use std::collections::{BTreeMap, BTreeSet};

use device_protocols::RawObservation;
use time::OffsetDateTime;

/// Snapshot of `BlueZ` properties required by `AirBattery`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct BluezDeviceSnapshot {
    /// Bluetooth address used only inside the backend boundary.
    pub address: String,
    /// Device alias or name.
    pub alias: Option<String>,
    /// Current `BlueZ` connection state.
    pub connected: bool,
    /// Whether the device is paired with the host.
    pub paired: bool,
    /// Bluetooth SIG vendor id from the Device ID modalias, when exposed.
    ///
    /// This is stable identity evidence for paired devices and is intentionally
    /// kept separate from unauthenticated advertisement manufacturer data.
    pub vendor_id: Option<u32>,
    /// Bluetooth SIG product id from the Device ID modalias, when exposed.
    pub product_id: Option<u32>,
    /// `org.bluez.Battery1.Percentage` when exposed.
    pub battery_percentage: Option<u8>,
    /// `BlueZ` icon/category hint.
    pub icon: Option<String>,
    /// Bluetooth Class of Device.
    pub class: Option<u32>,
    /// Bluetooth SIG appearance value.
    pub appearance: Option<u16>,
    /// Advertised or discovered service UUIDs.
    pub service_uuids: BTreeSet<String>,
    /// Most recent RSSI value.
    pub rssi: Option<i16>,
    /// Manufacturer payloads keyed by Bluetooth SIG company id.
    pub manufacturer_data: BTreeMap<u16, Vec<u8>>,
    /// Service payloads keyed by canonical service UUID.
    pub service_data: BTreeMap<String, Vec<u8>>,
    /// Snapshot timestamp.
    pub observed_at: OffsetDateTime,
}

/// Maps one `BlueZ` snapshot to raw observations without protocol inference.
#[must_use]
pub fn map_bluez_snapshot(snapshot: &BluezDeviceSnapshot) -> Vec<RawObservation> {
    let mut observations =
        Vec::with_capacity(snapshot.manufacturer_data.len() + snapshot.service_data.len() + 2);
    observations.push(RawObservation::Connection {
        device_id: snapshot.address.clone(),
        state: if snapshot.connected {
            shared_models::ConnectionState::Connected
        } else {
            shared_models::ConnectionState::Disconnected
        },
        observed_at: snapshot.observed_at,
    });

    if let Some(percentage) = snapshot.battery_percentage {
        observations.push(RawObservation::PlatformBattery {
            device_id: snapshot.address.clone(),
            percentage,
            source: shared_models::DataSource::BluezBattery,
            observed_at: snapshot.observed_at,
        });
    }

    observations.extend(
        snapshot
            .manufacturer_data
            .iter()
            .map(|(company_id, payload)| RawObservation::ManufacturerData {
                device_id: snapshot.address.clone(),
                company_id: *company_id,
                payload: payload.clone(),
                observed_at: snapshot.observed_at,
            }),
    );
    observations.extend(snapshot.service_data.iter().map(|(service_uuid, payload)| {
        RawObservation::ServiceData {
            device_id: snapshot.address.clone(),
            service_uuid: service_uuid.clone(),
            payload: payload.clone(),
            observed_at: snapshot.observed_at,
        }
    }));

    observations
}
