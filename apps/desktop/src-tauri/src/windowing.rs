//! Main and floating widget lifecycle.

use tauri::{AppHandle, Manager, WebviewUrl, WebviewWindowBuilder};

use crate::error::CommandError;

/// Shows and focuses the main application window.
pub fn show_main(app: &AppHandle) -> Result<(), CommandError> {
    let window = app.get_webview_window("main").ok_or_else(|| {
        CommandError::new(
            "window_missing",
            "The main AirBattery window is unavailable.",
        )
    })?;
    window.show().map_err(CommandError::backend)?;
    window.unminimize().map_err(CommandError::backend)?;
    window.set_focus().map_err(CommandError::backend)
}

/// Opens or focuses the optional always-on-top widget window.
pub fn show_widget(app: &AppHandle) -> Result<(), CommandError> {
    if let Some(window) = app.get_webview_window("widget") {
        window.show().map_err(CommandError::backend)?;
        return window.set_focus().map_err(CommandError::backend);
    }

    let window = WebviewWindowBuilder::new(
        app,
        "widget",
        WebviewUrl::App("index.html?window=widget".into()),
    )
    .title("AirBattery Widget")
    .inner_size(430.0, 370.0)
    .min_inner_size(340.0, 280.0)
    .resizable(true)
    .decorations(false)
    .transparent(true)
    .always_on_top(true)
    .skip_taskbar(true)
    .visible(true)
    .build()
    .map_err(CommandError::backend)?;
    window.set_focus().map_err(CommandError::backend)
}
