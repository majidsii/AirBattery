//! Native Windows Bluetooth Low Energy integration.

mod mapping;

use std::time::Duration;

use airbattery_service::DeviceDescriptor;
use device_protocols::RawObservation;
use thiserror::Error;

pub use mapping::{WindowsAdvertisement, WindowsKnownDevice, WindowsMapping, map_windows_devices};

/// One Windows refresh transaction consumed by the desktop host.
#[derive(Debug)]
pub struct WindowsCollection {
    /// Whether the native adapter could be queried.
    pub available: bool,
    /// Adapter display name when Windows exposes one.
    pub adapter_name: Option<String>,
    /// Radio power state when exposed.
    pub powered: Option<bool>,
    /// User-safe status detail.
    pub detail: String,
    /// Privacy-safe device metadata with backend ids retained internally.
    pub descriptors: Vec<DeviceDescriptor>,
    /// Connection, Battery Service, and advertisement observations.
    pub observations: Vec<RawObservation>,
}

/// Native Windows Bluetooth failure.
#[derive(Debug, Error)]
pub enum WindowsBluetoothError {
    /// Windows Runtime API failed.
    #[error("Windows Bluetooth API failed: {0}")]
    Native(String),
    /// The crate was invoked on a non-Windows target.
    #[error("Windows Bluetooth backend is unavailable on this operating system")]
    UnsupportedTarget,
}

/// Enumerates paired BLE devices and optionally receives BLE advertisements.
///
/// # Errors
///
/// Returns [`WindowsBluetoothError::Native`] when the native Windows API
/// cannot enumerate devices. On non-Windows systems, returns
/// [`WindowsBluetoothError::UnsupportedTarget`].
#[cfg_attr(not(target_os = "windows"), allow(clippy::unused_async))]
pub async fn collect_devices(
    discovery_window: Option<Duration>,
) -> Result<WindowsCollection, WindowsBluetoothError> {
    #[cfg(target_os = "windows")]
    {
        return native::collect_devices(discovery_window).await;
    }
    #[cfg(not(target_os = "windows"))]
    {
        let _ = discovery_window;
        Err(WindowsBluetoothError::UnsupportedTarget)
    }
}

#[cfg(target_os = "windows")]
mod native {
    use std::{
        sync::{Arc, Mutex},
        time::Duration,
    };

    use shared_models::ConnectionState;
    use time::OffsetDateTime;
    use windows::{
        Devices::{
            Bluetooth::{
                Advertisement::{
                    BluetoothLEAdvertisementReceivedEventArgs, BluetoothLEAdvertisementWatcher,
                    BluetoothLEScanningMode,
                },
                BluetoothAdapter, BluetoothConnectionStatus, BluetoothLEDevice,
                GenericAttributeProfile::GattCommunicationStatus,
            },
            Enumeration::DeviceInformation,
            Radios::RadioState,
        },
        Foundation::TypedEventHandler,
        Storage::Streams::{DataReader, IBuffer},
        core::GUID,
    };

    use super::{
        WindowsAdvertisement, WindowsBluetoothError, WindowsCollection, WindowsKnownDevice,
        map_windows_devices,
    };

    const BATTERY_SERVICE: GUID = GUID::from_u128(0x0000_180f_0000_1000_8000_0080_5f9b_34fb);
    const BATTERY_LEVEL: GUID = GUID::from_u128(0x0000_2a19_0000_1000_8000_0080_5f9b_34fb);

    pub async fn collect_devices(
        discovery_window: Option<Duration>,
    ) -> Result<WindowsCollection, WindowsBluetoothError> {
        let adapter = BluetoothAdapter::GetDefaultAsync()
            .map_err(native_error)?
            .await
            .map_err(native_error)?;
        let radio = adapter
            .GetRadioAsync()
            .map_err(native_error)?
            .await
            .map_err(native_error)?;
        let powered = Some(radio.State().map_err(native_error)? == RadioState::On);
        let adapter_name = Some(radio.Name().map_err(native_error)?.to_string());

        let known_devices = collect_known_devices().await?;
        let advertisements = if powered == Some(true) {
            match discovery_window {
                Some(window) => tokio::task::block_in_place(|| collect_advertisements(window))?,
                None => Vec::new(),
            }
        } else {
            Vec::new()
        };
        let mapping = map_windows_devices(known_devices, advertisements);
        let detail = match discovery_window {
            Some(window) => format!(
                "Windows Bluetooth is available; paired GATT data and a {}-second BLE advertisement window were combined.",
                window.as_secs()
            ),
            None => {
                "Windows Bluetooth is available; paired GATT battery data was refreshed.".to_owned()
            }
        };

        Ok(WindowsCollection {
            available: true,
            adapter_name,
            powered,
            detail,
            descriptors: mapping.descriptors,
            observations: mapping.observations,
        })
    }

    async fn collect_known_devices() -> Result<Vec<WindowsKnownDevice>, WindowsBluetoothError> {
        let selector =
            BluetoothLEDevice::GetDeviceSelectorFromPairingState(true).map_err(native_error)?;
        let information = DeviceInformation::FindAllAsyncAqsFilter(&selector)
            .map_err(native_error)?
            .await
            .map_err(native_error)?;
        let observed_at = OffsetDateTime::now_utc();
        let mut known_devices = Vec::new();

        for index in 0..information.Size().map_err(native_error)? {
            let info = information.GetAt(index).map_err(native_error)?;
            let id_value = info.Id().map_err(native_error)?;
            let id = id_value.to_string();
            let device = match BluetoothLEDevice::FromIdAsync(&id_value) {
                Ok(operation) => match operation.await {
                    Ok(value) => value,
                    Err(_) => continue,
                },
                Err(_) => continue,
            };
            let name = device
                .Name()
                .map(|value| value.to_string())
                .ok()
                .filter(|value| !value.trim().is_empty())
                .or_else(|| info.Name().ok().map(|value| value.to_string()))
                .filter(|value| !value.trim().is_empty())
                .unwrap_or_else(|| "Bluetooth LE device".to_owned());
            let connection_state = device
                .ConnectionStatus()
                .map(|status| {
                    if status == BluetoothConnectionStatus::Connected {
                        ConnectionState::Connected
                    } else {
                        ConnectionState::Disconnected
                    }
                })
                .unwrap_or(ConnectionState::Unknown);
            let address = device.BluetoothAddress().ok();
            let standard_battery = read_standard_battery(&device).await;

            known_devices.push(WindowsKnownDevice {
                backend_id: id,
                address,
                display_name: name,
                connection_state,
                standard_battery,
                observed_at,
            });
        }

        Ok(known_devices)
    }

    fn collect_advertisements(
        window: Duration,
    ) -> Result<Vec<WindowsAdvertisement>, WindowsBluetoothError> {
        let watcher = BluetoothLEAdvertisementWatcher::new().map_err(native_error)?;
        watcher
            .SetScanningMode(BluetoothLEScanningMode::Active)
            .map_err(native_error)?;
        let captured = Arc::new(Mutex::new(Vec::new()));
        let callback_values = Arc::clone(&captured);
        let handler = TypedEventHandler::<
            BluetoothLEAdvertisementWatcher,
            BluetoothLEAdvertisementReceivedEventArgs,
        >::new(move |_sender, args| {
            if let Some(args) = args.as_ref()
                && let Some(value) = advertisement_from_event(args)
                && let Ok(mut values) = callback_values.lock()
            {
                values.push(value);
            }
            Ok(())
        });
        let token = watcher.Received(&handler).map_err(native_error)?;
        watcher.Start().map_err(native_error)?;
        std::thread::sleep(window);
        let stop_result = watcher.Stop().map_err(native_error);
        let remove_result = watcher.RemoveReceived(token).map_err(native_error);
        stop_result?;
        remove_result?;

        let values = captured
            .lock()
            .map_err(|_| {
                WindowsBluetoothError::Native("advertisement buffer was poisoned".to_owned())
            })?
            .clone();
        Ok(values)
    }

    fn advertisement_from_event(
        args: &BluetoothLEAdvertisementReceivedEventArgs,
    ) -> Option<WindowsAdvertisement> {
        let advertisement = args.Advertisement().ok()?;
        let local_name = advertisement
            .LocalName()
            .ok()
            .map(|value| value.to_string())
            .filter(|value| !value.trim().is_empty());
        let service_uuids = advertisement
            .ServiceUuids()
            .ok()
            .map(|values| {
                let mut output = Vec::new();
                if let Ok(size) = values.Size() {
                    for index in 0..size {
                        if let Ok(value) = values.GetAt(index) {
                            output.push(format_guid(value));
                        }
                    }
                }
                output
            })
            .unwrap_or_default();
        let manufacturer_data = advertisement
            .ManufacturerData()
            .ok()
            .map(|values| {
                let mut output = Vec::new();
                if let Ok(size) = values.Size() {
                    for index in 0..size {
                        let Ok(value) = values.GetAt(index) else {
                            continue;
                        };
                        let Ok(company_id) = value.CompanyId() else {
                            continue;
                        };
                        let Ok(buffer) = value.Data() else {
                            continue;
                        };
                        if let Some(payload) = read_buffer(&buffer) {
                            output.push((company_id, payload));
                        }
                    }
                }
                output
            })
            .unwrap_or_default();

        Some(WindowsAdvertisement {
            address: args.BluetoothAddress().ok()?,
            local_name,
            manufacturer_data,
            service_uuids,
            rssi: args.RawSignalStrengthInDBm().ok(),
            observed_at: OffsetDateTime::now_utc(),
        })
    }

    fn format_guid(value: GUID) -> String {
        format!(
            "{:08x}-{:04x}-{:04x}-{:02x}{:02x}-{:02x}{:02x}{:02x}{:02x}{:02x}{:02x}",
            value.data1,
            value.data2,
            value.data3,
            value.data4[0],
            value.data4[1],
            value.data4[2],
            value.data4[3],
            value.data4[4],
            value.data4[5],
            value.data4[6],
            value.data4[7],
        )
    }

    fn read_buffer(buffer: &IBuffer) -> Option<Vec<u8>> {
        let reader = DataReader::FromBuffer(buffer).ok()?;
        let length = usize::try_from(reader.UnconsumedBufferLength().ok()?).ok()?;
        let mut bytes = vec![0_u8; length];
        reader.ReadBytes(&mut bytes).ok()?;
        Some(bytes)
    }

    async fn read_standard_battery(device: &BluetoothLEDevice) -> Option<u8> {
        let services = device
            .GetGattServicesForUuidAsync(BATTERY_SERVICE)
            .ok()?
            .await
            .ok()?;
        if services.Status().ok()? != GattCommunicationStatus::Success {
            return None;
        }
        let service_list = services.Services().ok()?;
        for service_index in 0..service_list.Size().ok()? {
            let service = service_list.GetAt(service_index).ok()?;
            let characteristics = service
                .GetCharacteristicsForUuidAsync(BATTERY_LEVEL)
                .ok()?
                .await
                .ok()?;
            if characteristics.Status().ok()? != GattCommunicationStatus::Success {
                continue;
            }
            let characteristic_list = characteristics.Characteristics().ok()?;
            for characteristic_index in 0..characteristic_list.Size().ok()? {
                let characteristic = characteristic_list.GetAt(characteristic_index).ok()?;
                let result = characteristic.ReadValueAsync().ok()?.await.ok()?;
                if result.Status().ok()? != GattCommunicationStatus::Success {
                    continue;
                }
                let buffer = result.Value().ok()?;
                if buffer.Length().ok()? < 1 {
                    continue;
                }
                let reader = DataReader::FromBuffer(&buffer).ok()?;
                let value = reader.ReadByte().ok()?;
                if value <= 100 {
                    return Some(value);
                }
            }
        }
        None
    }

    fn native_error(error: windows::core::Error) -> WindowsBluetoothError {
        WindowsBluetoothError::Native(error.to_string())
    }
}
