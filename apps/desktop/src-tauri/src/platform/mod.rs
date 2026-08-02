//! Target-specific Bluetooth collection behind one desktop contract.

use crate::{
    error::CommandError,
    model::{PlatformCollection, RefreshMode},
};

#[cfg(target_os = "linux")]
mod linux;
#[cfg(target_os = "windows")]
mod windows;

/// Collects native Bluetooth state for the current target.
pub async fn collect(mode: RefreshMode) -> Result<PlatformCollection, CommandError> {
    #[cfg(target_os = "linux")]
    {
        return linux::collect(mode).await;
    }
    #[cfg(target_os = "windows")]
    {
        return windows::collect(mode).await;
    }
    #[allow(unreachable_code)]
    Ok(PlatformCollection {
        status: crate::model::BackendStatus::unavailable(
            crate::model::PlatformKind::Unsupported,
            "No Bluetooth backend is available for this operating system.",
        ),
        descriptors: Vec::new(),
        observations: Vec::new(),
    })
}
