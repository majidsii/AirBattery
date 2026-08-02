//! `AirBattery` Tauri desktop shell.

mod commands;
mod error;
mod gnome;
mod model;
mod native_surface;
mod platform;
mod refresh;
mod state;
mod tray;
#[cfg(target_os = "windows")]
mod tray_icon;
mod windowing;

use airbattery_settings::{AppSettings, SettingsRepository};
use tauri::{Manager, WindowEvent};
use tauri_plugin_autostart::MacosLauncher;

pub use model::{BackendStatus, PlatformCollection, PlatformKind, RefreshMode, RefreshResult};
pub use refresh::scheduled_refresh_mode;
pub use state::RuntimeState;
pub use tray::{TraySnapshot, tray_snapshot};

/// Starts the native desktop application.
pub fn run() {
    initialize_logging();
    let result = tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _arguments, _cwd| {
            if let Err(error) = windowing::show_main(app) {
                tracing::warn!(code = error.code, message = %error.message, "single-instance focus failed");
            }
        }))
        .plugin(tauri_plugin_autostart::init(
            MacosLauncher::LaunchAgent,
            Some(vec!["--background"]),
        ))
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_notification::init())
        .setup(|app| {
            let config_dir = app.path().app_config_dir()?;
            let state = RuntimeState::new(SettingsRepository::new(config_dir.join("settings.json")));
            let settings = match state.load_settings() {
                Ok(settings) => settings,
                Err(error) => {
                    tracing::warn!(error = %error, "settings could not be loaded; defaults are active");
                    AppSettings::default()
                }
            };
            app.manage(state);
            app.manage(gnome::GnomeIntegration::default());
            let tray_visible = native_surface::should_show_native_tray(
                PlatformKind::current(),
                native_surface::detect_desktop_environment(),
                &settings,
            );
            tray::setup(app, tray_visible)?;

            if std::env::args().any(|argument| argument == "--background")
                && let Some(window) = app.get_webview_window("main")
            {
                window.hide()?;
            }

            let handle = app.handle().clone();
            let gnome_enabled = settings.enable_gnome_integration;
            tauri::async_runtime::spawn(async move {
                if let Err(error) = gnome::reconcile(&handle, gnome_enabled).await {
                    tracing::warn!(code = error.code, message = %error.message, "GNOME session service failed to start");
                }
                if let Err(error) = commands::refresh_and_emit(&handle, RefreshMode::BoundedDiscovery).await {
                    tracing::warn!(code = error.code, message = %error.message, "initial refresh failed");
                }
                refresh::run(handle).await;
            });
            Ok(())
        })
        .on_window_event(|window, event| {
            if window.label() != "main" {
                return;
            }
            if matches!(event, WindowEvent::Focused(true)) {
                let handle = window.app_handle().clone();
                tauri::async_runtime::spawn(async move {
                    if let Err(error) = commands::refresh_and_emit(&handle, RefreshMode::KnownOnly).await {
                        tracing::debug!(code = error.code, message = %error.message, "focus refresh failed");
                    }
                });
            }
            if let WindowEvent::CloseRequested { api, .. } = event {
                let state = window.app_handle().state::<RuntimeState>();
                let run_in_background = state
                    .load_settings()
                    .map_or(true, |settings| settings.run_in_background);
                if run_in_background {
                    api.prevent_close();
                    if let Err(error) = window.hide() {
                        tracing::warn!(error = %error, "main window could not be hidden");
                    }
                }
            }
        })
        .invoke_handler(tauri::generate_handler![
            commands::get_devices,
            commands::get_backend_status,
            commands::refresh_devices,
            commands::run_diagnostic_scan,
            commands::get_settings,
            commands::save_settings,
            commands::export_diagnostics,
            commands::open_widget,
            commands::open_settings,
        ])
        .run(tauri::generate_context!());

    if let Err(error) = result {
        tracing::error!(error = %error, "AirBattery failed to start");
    }
}

fn initialize_logging() {
    let filter = tracing_subscriber::EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| tracing_subscriber::EnvFilter::new("airbattery=info,warn"));
    let _initialization = tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(true)
        .try_init();
}
