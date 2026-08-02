//! Serialized automatic refresh scheduling.

use std::time::Duration;

use tauri::AppHandle;

use crate::{commands, model::RefreshMode};

const REFRESH_INTERVAL: Duration = Duration::from_secs(3);

/// Chooses bounded advertisement discovery for every automatic refresh.
///
/// `AirPods` battery values are carried by short-lived advertisements. Reading
/// only the known-device cache between scans makes the UI appear frozen for up
/// to a minute, so each serialized tick performs one short bounded scan.
#[must_use]
pub const fn scheduled_refresh_mode(_tick: u64) -> RefreshMode {
    RefreshMode::BoundedDiscovery
}

/// Runs the automatic refresh loop for the application lifetime.
pub async fn run(app: AppHandle) {
    let mut interval = tokio::time::interval(REFRESH_INTERVAL);
    interval.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    // The startup path performs the first bounded scan. Discard interval's immediate tick.
    interval.tick().await;
    let mut tick = 1_u64;
    loop {
        interval.tick().await;
        let mode = scheduled_refresh_mode(tick);
        if let Err(error) = commands::refresh_and_emit(&app, mode).await {
            tracing::warn!(
                code = error.code,
                message = %error.message,
                ?mode,
                "automatic Bluetooth refresh failed"
            );
        }
        tick = tick.saturating_add(1);
    }
}
