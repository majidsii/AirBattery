//! Validated Tauri command boundary.

use std::{collections::BTreeSet, fs};

use airbattery_settings::AppSettings;
use serde::Serialize;
use shared_models::{
    BatteryComponent, BluetoothAudioDevice, ConnectionState, DataConfidence, DataSource,
    DeviceFamily, DeviceVisual,
};
use tauri::{AppHandle, Emitter, Manager, State};
use tauri_plugin_autostart::ManagerExt as _;
use time::{OffsetDateTime, format_description::well_known::Rfc3339};

use crate::{
    error::CommandError,
    gnome,
    model::{BackendStatus, PlatformKind, RefreshMode},
    native_surface, platform,
    state::RuntimeState,
    tray, windowing,
};

/// Frontend event carrying complete device snapshots.
pub const DEVICE_EVENT: &str = "airbattery://devices-changed";
/// Frontend event carrying native Bluetooth backend health.
pub const BACKEND_STATUS_EVENT: &str = "airbattery://backend-status-changed";
/// Frontend event used by native integrations to switch views.
pub const NAVIGATE_EVENT: &str = "airbattery://navigate";

/// Diagnostic export metadata returned to the frontend.
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticBundle {
    path: String,
    created_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DiagnosticExport {
    application_version: String,
    operating_system: String,
    architecture: String,
    backend: BackendStatus,
    device_count: usize,
    connected_device_count: usize,
    sanitized_device_ids: Vec<String>,
    data_sources: BTreeSet<DataSource>,
    devices: Vec<DiagnosticDevice>,
    generated_at: String,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DiagnosticDevice {
    id: String,
    display_name: String,
    manufacturer: Option<String>,
    model: Option<String>,
    family: DeviceFamily,
    visual: DeviceVisual,
    connection_state: ConnectionState,
    components: Vec<DiagnosticComponent>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
struct DiagnosticComponent {
    component_type: shared_models::ComponentType,
    percentage: Option<u8>,
    charging_state: shared_models::ChargingState,
    source: DataSource,
    confidence: DataConfidence,
    stale: bool,
    updated_at: String,
}

impl From<&BatteryComponent> for DiagnosticComponent {
    fn from(component: &BatteryComponent) -> Self {
        Self {
            component_type: component.component_type,
            percentage: component
                .percentage
                .map(shared_models::BatteryPercentage::get),
            charging_state: component.charging_state,
            source: component.source,
            confidence: component.confidence,
            stale: component.stale,
            updated_at: component
                .updated_at
                .format(&Rfc3339)
                .unwrap_or_else(|_| component.updated_at.unix_timestamp().to_string()),
        }
    }
}

impl From<&BluetoothAudioDevice> for DiagnosticDevice {
    fn from(device: &BluetoothAudioDevice) -> Self {
        Self {
            id: device.id.clone(),
            display_name: device.display_name.clone(),
            manufacturer: device.manufacturer.clone(),
            model: device.model.clone(),
            family: device.device_family,
            visual: device.visual.clone(),
            connection_state: device.connection_state,
            components: device
                .components
                .iter()
                .map(DiagnosticComponent::from)
                .collect(),
        }
    }
}

/// Returns the current in-memory devices without triggering a scan.
///
/// # Errors
///
/// This command currently has no application-level failure path. The
/// [`Result`] return is required by `Tauri` because this asynchronous command
/// borrows managed application state.
#[tauri::command]
pub async fn get_devices(
    state: State<'_, RuntimeState>,
) -> Result<Vec<BluetoothAudioDevice>, CommandError> {
    Ok(state.devices().await)
}

/// Returns the latest Bluetooth backend status.
///
/// # Errors
///
/// This command currently has no application-level failure path. The
/// [`Result`] return is required by `Tauri` because this asynchronous command
/// borrows managed application state.
#[tauri::command]
pub async fn get_backend_status(
    state: State<'_, RuntimeState>,
) -> Result<BackendStatus, CommandError> {
    Ok(state.backend_status().await)
}

/// Runs one bounded native refresh.
#[tauri::command]
pub async fn refresh_devices(app: AppHandle) -> Result<Vec<BluetoothAudioDevice>, CommandError> {
    refresh_and_emit(&app, RefreshMode::BoundedDiscovery).await
}

/// Runs a longer user-initiated scan and returns the normalized snapshot.
#[tauri::command]
pub async fn run_diagnostic_scan(
    app: AppHandle,
) -> Result<Vec<BluetoothAudioDevice>, CommandError> {
    refresh_and_emit(&app, RefreshMode::DiagnosticDiscovery).await
}

/// Serializes a native collection, applies protocol providers, and emits only changed snapshots.
pub async fn refresh_and_emit(
    app: &AppHandle,
    mode: RefreshMode,
) -> Result<Vec<BluetoothAudioDevice>, CommandError> {
    let state = app.state::<RuntimeState>();
    let _refresh_guard = state.lock_refresh().await;
    let collection = platform::collect(mode).await?;
    let result = state.apply_collection(collection).await;
    let preferred_device_id = state
        .load_settings()
        .ok()
        .and_then(|settings| settings.preferred_device_id);
    if let Err(error) = tray::update_snapshot(app, &result.devices, preferred_device_id.as_deref())
    {
        tracing::warn!(error = %error, "tray snapshot update failed");
    }
    if result.devices_changed {
        app.emit(DEVICE_EVENT, &result.devices)
            .map_err(|error| CommandError::new("event_delivery", error.to_string()))?;
    }
    if result.status_changed {
        app.emit(BACKEND_STATUS_EVENT, &result.status)
            .map_err(|error| CommandError::new("event_delivery", error.to_string()))?;
    }
    if (result.devices_changed || result.status_changed)
        && let Err(error) = gnome::publish(app).await
    {
        tracing::warn!(code = error.code, message = %error.message, "GNOME snapshot signal failed");
    }
    Ok(result.devices)
}

/// Loads validated versioned settings.
#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub fn get_settings(state: State<'_, RuntimeState>) -> Result<AppSettings, CommandError> {
    state.load_settings().map_err(CommandError::storage)
}

/// Applies native lifecycle settings and atomically stores normalized preferences.
#[tauri::command]
pub async fn save_settings(
    app: AppHandle,
    state: State<'_, RuntimeState>,
    settings: AppSettings,
) -> Result<AppSettings, CommandError> {
    let normalized = settings.normalized();
    let autostart = app.autolaunch();
    if normalized.start_with_system {
        autostart.enable().map_err(CommandError::backend)?;
    } else {
        autostart.disable().map_err(CommandError::backend)?;
    }

    if let Some(tray) = app.tray_by_id("airbattery-tray") {
        let tray_visible = native_surface::should_show_native_tray(
            PlatformKind::current(),
            native_surface::detect_desktop_environment(),
            &normalized,
        );
        tray.set_visible(tray_visible)
            .map_err(CommandError::backend)?;
    }
    if !normalized.enable_desktop_widget
        && let Some(widget) = app.get_webview_window("widget")
    {
        widget.hide().map_err(CommandError::backend)?;
    }

    let saved = state
        .save_settings(&normalized)
        .map_err(CommandError::storage)?;
    if let Err(error) = gnome::reconcile(&app, saved.enable_gnome_integration).await {
        tracing::warn!(
            code = error.code,
            message = %error.message,
            "GNOME integration reconfiguration failed"
        );
    }
    Ok(saved)
}

/// Opens or focuses the floating widget.
#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub fn open_widget(app: AppHandle) -> Result<(), CommandError> {
    windowing::show_widget(&app)
}

/// Opens the main settings surface for GNOME and tray integrations.
#[allow(clippy::needless_pass_by_value)]
#[tauri::command]
pub fn open_settings(app: AppHandle) -> Result<(), CommandError> {
    windowing::show_main(&app)?;
    app.emit(NAVIGATE_EVENT, "settings")
        .map_err(|error| CommandError::new("event_delivery", error.to_string()))
}

/// Writes a sanitized diagnostic report to an application-controlled directory.
#[tauri::command]
pub async fn export_diagnostics(
    app: AppHandle,
    state: State<'_, RuntimeState>,
) -> Result<DiagnosticBundle, CommandError> {
    let now = OffsetDateTime::now_utc();
    let created_at = now.format(&Rfc3339).map_err(CommandError::backend)?;
    let devices = state.devices().await;
    let backend = state.backend_status().await;
    let data_sources = devices
        .iter()
        .flat_map(|device| device.components.iter().map(|component| component.source))
        .collect();
    let report = DiagnosticExport {
        application_version: env!("CARGO_PKG_VERSION").to_owned(),
        operating_system: std::env::consts::OS.to_owned(),
        architecture: std::env::consts::ARCH.to_owned(),
        backend,
        device_count: devices.len(),
        connected_device_count: devices
            .iter()
            .filter(|device| {
                matches!(
                    device.connection_state,
                    shared_models::ConnectionState::Connected
                )
            })
            .count(),
        sanitized_device_ids: devices.iter().map(|device| device.id.clone()).collect(),
        data_sources,
        devices: devices.iter().map(DiagnosticDevice::from).collect(),
        generated_at: created_at.clone(),
    };

    let directory = app
        .path()
        .app_data_dir()
        .map_err(CommandError::backend)?
        .join("diagnostics");
    fs::create_dir_all(&directory).map_err(CommandError::storage)?;
    let path = directory.join(format!(
        "airbattery-diagnostics-{}.json",
        now.unix_timestamp()
    ));
    let serialized = serde_json::to_vec_pretty(&report).map_err(CommandError::storage)?;
    fs::write(&path, serialized).map_err(CommandError::storage)?;

    Ok(DiagnosticBundle {
        path: path.to_string_lossy().into_owned(),
        created_at,
    })
}
