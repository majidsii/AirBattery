//! Windows Runtime collection bridge.

use std::time::Duration;

use crate::{
    error::CommandError,
    model::{BackendStatus, PlatformCollection, PlatformKind, RefreshMode},
};

const DISCOVERY_WINDOW: Duration = Duration::from_secs(2);
const DIAGNOSTIC_DISCOVERY_WINDOW: Duration = Duration::from_secs(30);

pub async fn collect(mode: RefreshMode) -> Result<PlatformCollection, CommandError> {
    let discovery_window = match mode {
        RefreshMode::KnownOnly => None,
        RefreshMode::BoundedDiscovery => Some(DISCOVERY_WINDOW),
        RefreshMode::DiagnosticDiscovery => Some(DIAGNOSTIC_DISCOVERY_WINDOW),
    };
    let collection = match bluetooth_windows::collect_devices(discovery_window).await {
        Ok(collection) => collection,
        Err(error) => {
            tracing::warn!(error = %error, "Windows Bluetooth collection failed");
            return Ok(PlatformCollection {
                status: BackendStatus::unavailable(
                    PlatformKind::Windows,
                    "Windows Bluetooth data could not be read.",
                ),
                descriptors: Vec::new(),
                observations: Vec::new(),
            });
        }
    };
    Ok(PlatformCollection {
        status: BackendStatus {
            platform: PlatformKind::Windows,
            available: collection.available,
            adapter_name: collection.adapter_name,
            powered: collection.powered,
            discovering: Some(discovery_window.is_some()),
            detail: collection.detail,
        },
        descriptors: collection.descriptors,
        observations: collection.observations,
    })
}
