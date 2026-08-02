//! Bounded `BlueZ` discovery backend.

use std::{collections::BTreeMap, time::Duration as StdDuration};

use bluer::{Adapter, AdapterEvent, DiscoveryFilter, DiscoveryTransport};
use futures::StreamExt;
use serde::{Deserialize, Serialize};
use thiserror::Error;
use time::OffsetDateTime;

use crate::BluezDeviceSnapshot;

/// Linux Bluetooth backend error.
#[derive(Debug, Error)]
pub enum BluetoothLinuxError {
    /// `BlueZ` or the system D-Bus is unavailable.
    #[error("BlueZ backend unavailable: {0}")]
    BackendUnavailable(String),
    /// The selected adapter exists but is powered off.
    #[error("Bluetooth adapter is powered off")]
    AdapterPoweredOff,
}

/// Read-only adapter status.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct LinuxBackendStatus {
    /// `BlueZ` adapter name, such as `hci0`.
    pub adapter_name: String,
    /// Whether the adapter is powered.
    pub powered: bool,
    /// Whether the adapter is currently discovering.
    pub discovering: bool,
}

/// `BlueZ` backend that never powers on or configures the adapter implicitly.
#[derive(Clone)]
pub struct LinuxBluetoothBackend {
    adapter: Adapter,
}

impl LinuxBluetoothBackend {
    /// Connects to `BlueZ` and selects its default adapter.
    ///
    /// # Errors
    ///
    /// Returns [`BluetoothLinuxError`] when the system D-Bus connection or
    /// default `BlueZ` adapter cannot be initialized.
    pub async fn new() -> Result<Self, BluetoothLinuxError> {
        let session = bluer::Session::new()
            .await
            .map_err(|error| BluetoothLinuxError::BackendUnavailable(error.to_string()))?;
        let adapter = session
            .default_adapter()
            .await
            .map_err(|error| BluetoothLinuxError::BackendUnavailable(error.to_string()))?;
        Ok(Self { adapter })
    }

    /// Returns current adapter state without changing it.
    ///
    /// # Errors
    ///
    /// Returns [`BluetoothLinuxError`] when adapter state or adapter metadata
    /// cannot be read from `BlueZ`.
    pub async fn status(&self) -> Result<LinuxBackendStatus, BluetoothLinuxError> {
        let powered = self
            .adapter
            .is_powered()
            .await
            .map_err(|error| BluetoothLinuxError::BackendUnavailable(error.to_string()))?;
        let discovering = self
            .adapter
            .is_discovering()
            .await
            .map_err(|error| BluetoothLinuxError::BackendUnavailable(error.to_string()))?;
        Ok(LinuxBackendStatus {
            adapter_name: self.adapter.name().to_owned(),
            powered,
            discovering,
        })
    }

    /// Reads all devices currently known to `BlueZ`.
    ///
    /// # Errors
    ///
    /// Returns [`BluetoothLinuxError`] when known devices cannot be enumerated
    /// or their required `BlueZ` properties cannot be read.
    pub async fn known_devices(&self) -> Result<Vec<BluezDeviceSnapshot>, BluetoothLinuxError> {
        let addresses = self
            .adapter
            .device_addresses()
            .await
            .map_err(|error| BluetoothLinuxError::BackendUnavailable(error.to_string()))?;
        let mut snapshots = Vec::with_capacity(addresses.len());
        for address in addresses {
            if let Ok(snapshot) = self.snapshot(address).await {
                snapshots.push(snapshot);
            }
        }
        Ok(snapshots)
    }

    /// Runs discovery for at most `duration` and returns the latest snapshot per address.
    ///
    /// # Errors
    ///
    /// Returns [`BluetoothLinuxError`] when bounded discovery cannot start,
    /// discovery events cannot be consumed, or device properties cannot be read.
    pub async fn scan_window(
        &self,
        duration: StdDuration,
    ) -> Result<Vec<BluezDeviceSnapshot>, BluetoothLinuxError> {
        if !self
            .adapter
            .is_powered()
            .await
            .map_err(|error| BluetoothLinuxError::BackendUnavailable(error.to_string()))?
        {
            return Err(BluetoothLinuxError::AdapterPoweredOff);
        }

        self.adapter
            .set_discovery_filter(DiscoveryFilter {
                transport: DiscoveryTransport::Auto,
                duplicate_data: true,
                ..DiscoveryFilter::default()
            })
            .await
            .map_err(|error| BluetoothLinuxError::BackendUnavailable(error.to_string()))?;

        let events = self
            .adapter
            .discover_devices_with_changes()
            .await
            .map_err(|error| BluetoothLinuxError::BackendUnavailable(error.to_string()))?;
        tokio::pin!(events);

        let deadline = tokio::time::Instant::now() + duration;
        let mut snapshots = BTreeMap::new();
        loop {
            let now = tokio::time::Instant::now();
            if now >= deadline {
                break;
            }
            let remaining = deadline.saturating_duration_since(now);
            let Ok(Some(event)) = tokio::time::timeout(remaining, events.next()).await else {
                break;
            };

            if let AdapterEvent::DeviceAdded(address) = event
                && let Ok(snapshot) = self.snapshot(address).await
            {
                snapshots.insert(snapshot.address.clone(), snapshot);
            }
        }

        Ok(snapshots.into_values().collect())
    }

    async fn snapshot(
        &self,
        address: bluer::Address,
    ) -> Result<BluezDeviceSnapshot, BluetoothLinuxError> {
        let device = self
            .adapter
            .device(address)
            .map_err(|error| BluetoothLinuxError::BackendUnavailable(error.to_string()))?;
        let alias = device
            .name()
            .await
            .map_err(|error| BluetoothLinuxError::BackendUnavailable(error.to_string()))?;
        let connected = device
            .is_connected()
            .await
            .map_err(|error| BluetoothLinuxError::BackendUnavailable(error.to_string()))?;
        let battery_percentage = device
            .battery_percentage()
            .await
            .map_err(|error| BluetoothLinuxError::BackendUnavailable(error.to_string()))?;
        let paired = device
            .is_paired()
            .await
            .map_err(|error| BluetoothLinuxError::BackendUnavailable(error.to_string()))?;
        let (vendor_id, product_id) = device
            .modalias()
            .await
            .ok()
            .flatten()
            .map_or((None, None), |modalias| {
                (Some(modalias.vendor), Some(modalias.product))
            });
        let icon = device
            .icon()
            .await
            .map_err(|error| BluetoothLinuxError::BackendUnavailable(error.to_string()))?;
        let class = device
            .class()
            .await
            .map_err(|error| BluetoothLinuxError::BackendUnavailable(error.to_string()))?;
        let appearance = device
            .appearance()
            .await
            .map_err(|error| BluetoothLinuxError::BackendUnavailable(error.to_string()))?;
        let service_uuids = device
            .uuids()
            .await
            .map_err(|error| BluetoothLinuxError::BackendUnavailable(error.to_string()))?
            .unwrap_or_default()
            .into_iter()
            .map(|uuid| uuid.to_string().to_ascii_lowercase())
            .collect();
        let rssi = device
            .rssi()
            .await
            .map_err(|error| BluetoothLinuxError::BackendUnavailable(error.to_string()))?;
        let manufacturer_data = device
            .manufacturer_data()
            .await
            .map_err(|error| BluetoothLinuxError::BackendUnavailable(error.to_string()))?
            .unwrap_or_default()
            .into_iter()
            .collect();
        let service_data = device
            .service_data()
            .await
            .map_err(|error| BluetoothLinuxError::BackendUnavailable(error.to_string()))?
            .unwrap_or_default()
            .into_iter()
            .map(|(uuid, payload)| (uuid.to_string().to_ascii_lowercase(), payload))
            .collect();

        Ok(BluezDeviceSnapshot {
            address: address.to_string(),
            alias,
            connected,
            paired,
            vendor_id,
            product_id,
            battery_percentage,
            icon,
            class,
            appearance,
            service_uuids,
            rssi,
            manufacturer_data,
            service_data,
            observed_at: OffsetDateTime::now_utc(),
        })
    }
}
