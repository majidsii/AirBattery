//! Native tray, lifecycle behavior, and snapshot summaries.

use std::fmt::Write as _;

use shared_models::{BluetoothAudioDevice, ComponentType, ConnectionState};
use tauri::{
    App, AppHandle, Emitter,
    menu::{Menu, MenuItem},
    tray::{MouseButton, MouseButtonState, TrayIconBuilder, TrayIconEvent},
};

#[cfg(target_os = "windows")]
use crate::tray_icon;
use crate::windowing;
#[cfg(target_os = "windows")]
use tauri::Manager;

/// Display data shared by Linux tray tooltips and the Windows percentage icon.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TraySnapshot {
    /// Device name.
    pub device_name: String,
    /// Human-readable component summary.
    pub label: String,
    /// Percentage rendered into the Windows tray icon.
    pub icon_percentage: Option<u8>,
}

fn known_percentage(device: &BluetoothAudioDevice, component_type: ComponentType) -> Option<u8> {
    device
        .components
        .iter()
        .find(|component| {
            component.component_type == component_type
                && !component.stale
                && component.percentage.is_some()
        })
        .and_then(|component| component.percentage)
        .map(shared_models::BatteryPercentage::get)
}

fn percentage_text(value: Option<u8>) -> String {
    value.map_or_else(|| "—".to_owned(), |percentage| format!("{percentage}%"))
}

fn has_fresh_battery(device: &BluetoothAudioDevice) -> bool {
    device
        .components
        .iter()
        .any(|component| component.percentage.is_some() && !component.stale)
}

fn has_live_accessory_evidence(device: &BluetoothAudioDevice) -> bool {
    device.components.iter().any(|component| {
        component.percentage.is_some()
            && !component.stale
            && component.source == shared_models::DataSource::VendorProtocol
            && component.confidence == shared_models::DataConfidence::Verified
    })
}

fn activity_timestamp(device: &BluetoothAudioDevice) -> i128 {
    device
        .last_updated_at
        .map_or(i128::MIN, time::OffsetDateTime::unix_timestamp_nanos)
}

fn active_device_score(
    device: &BluetoothAudioDevice,
    preferred_device_id: Option<&str>,
) -> (bool, i128, bool) {
    (
        has_fresh_battery(device),
        activity_timestamp(device),
        preferred_device_id.is_some_and(|id| device.id == id),
    )
}

fn select_active_device<'a>(
    devices: &'a [BluetoothAudioDevice],
    preferred_device_id: Option<&str>,
) -> Option<&'a BluetoothAudioDevice> {
    devices
        .iter()
        .filter(|device| {
            matches!(
                device.connection_state,
                ConnectionState::Connected | ConnectionState::Connecting
            ) || has_live_accessory_evidence(device)
        })
        .max_by_key(|device| active_device_score(device, preferred_device_id))
        .or_else(|| {
            preferred_device_id.and_then(|id| devices.iter().find(|device| device.id == id))
        })
        .or_else(|| devices.first())
}

/// Selects the preferred or connected device and builds a truthful tray summary.
#[must_use]
pub fn tray_snapshot(
    devices: &[BluetoothAudioDevice],
    preferred_device_id: Option<&str>,
) -> Option<TraySnapshot> {
    let device = select_active_device(devices, preferred_device_id)?;

    let left = known_percentage(device, ComponentType::Left);
    let right = known_percentage(device, ComponentType::Right);
    let case = known_percentage(device, ComponentType::Case);
    let aggregate = known_percentage(device, ComponentType::Aggregate)
        .or_else(|| known_percentage(device, ComponentType::Headset))
        .or_else(|| known_percentage(device, ComponentType::Unknown));

    let has_split_component = left.is_some() || right.is_some() || case.is_some();
    let split = (matches!(device.device_family, shared_models::DeviceFamily::AirPods)
        && has_split_component)
        || (left.is_some() && right.is_some());
    let (label, icon_percentage) = if split {
        let mut label = format!("L {}  R {}", percentage_text(left), percentage_text(right));
        if matches!(device.device_family, shared_models::DeviceFamily::AirPods) || case.is_some() {
            let _ = write!(label, "  C {}", percentage_text(case));
        }
        (label, left.into_iter().chain(right).min())
    } else {
        (percentage_text(aggregate), aggregate)
    };

    Some(TraySnapshot {
        device_name: device.display_name.clone(),
        label,
        icon_percentage,
    })
}

/// Creates the tray menu and native event handlers.
pub fn setup(app: &App, visible: bool) -> tauri::Result<()> {
    let show = MenuItem::with_id(app, "show", "Open AirBattery", true, None::<&str>)?;
    let settings = MenuItem::with_id(app, "settings", "Settings", true, None::<&str>)?;
    let restart = MenuItem::with_id(app, "restart", "Restart AirBattery", true, None::<&str>)?;
    let quit = MenuItem::with_id(app, "quit", "Quit", true, None::<&str>)?;
    let menu = Menu::with_items(app, &[&show, &settings, &restart, &quit])?;

    let mut builder = TrayIconBuilder::with_id("airbattery-tray")
        .tooltip("AirBattery · waiting for Bluetooth battery data")
        .menu(&menu)
        .show_menu_on_left_click(false)
        .on_menu_event(|app, event| match event.id.as_ref() {
            "show" => show_main_safely(app),
            "settings" => {
                if let Err(error) = windowing::show_main(app) {
                    tracing::warn!(code = error.code, message = %error.message, "settings window could not be shown");
                }
                if let Err(error) = app.emit(crate::commands::NAVIGATE_EVENT, "settings") {
                    tracing::warn!(error = %error, "settings navigation event failed");
                }
            }
            "restart" => app.request_restart(),
            "quit" => app.exit(0),
            _ => {}
        })
        .on_tray_icon_event(|tray, event| {
            if matches!(
                event,
                TrayIconEvent::Click {
                    button: MouseButton::Left,
                    button_state: MouseButtonState::Up,
                    ..
                }
            ) {
                show_main_safely(tray.app_handle());
            }
        });

    if let Some(icon) = app.default_window_icon() {
        builder = builder.icon(icon.clone());
    }
    let tray = builder.build(app)?;
    tray.set_visible(visible)?;
    Ok(())
}

/// Synchronizes tray tooltip and, on Windows, its generated percentage icon.
pub fn update_snapshot(
    app: &AppHandle,
    devices: &[BluetoothAudioDevice],
    preferred_device_id: Option<&str>,
) -> tauri::Result<()> {
    let Some(tray) = app.tray_by_id("airbattery-tray") else {
        return Ok(());
    };
    let snapshot = tray_snapshot(devices, preferred_device_id);
    let tooltip = snapshot.as_ref().map_or_else(
        || "AirBattery · battery data unavailable".to_owned(),
        |snapshot| format!("AirBattery · {} · {}", snapshot.device_name, snapshot.label),
    );
    tray.set_tooltip(Some(tooltip))?;

    #[cfg(target_os = "windows")]
    {
        let pixels = snapshot
            .and_then(|snapshot| snapshot.icon_percentage)
            .map_or_else(
                tray_icon::render_unavailable_icon,
                tray_icon::render_percentage_icon,
            );
        let image = tauri::image::Image::new_owned(pixels, 32, 32);
        tray.set_icon(Some(image))?;
    }

    Ok(())
}

fn show_main_safely(app: &AppHandle) {
    if let Err(error) = windowing::show_main(app) {
        tracing::warn!(code = error.code, message = %error.message, "main window could not be shown");
    }
}
