//! Serializable errors crossing the Rust-to-webview boundary.

use serde::Serialize;
use thiserror::Error;

/// Stable command error returned to the first-party frontend.
#[derive(Debug, Clone, Serialize, Error)]
#[serde(rename_all = "camelCase")]
#[error("{message}")]
pub struct CommandError {
    /// Stable machine-readable category.
    pub code: &'static str,
    /// User-safe explanation without raw Bluetooth identifiers.
    pub message: String,
}

impl CommandError {
    /// Creates a command error with a stable code.
    #[must_use]
    pub fn new(code: &'static str, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }

    /// Maps an internal error into a safe backend failure.
    #[must_use]
    pub fn backend(error: impl std::fmt::Display) -> Self {
        Self::new("backend_failed", error.to_string())
    }

    /// Maps an application-controlled filesystem failure.
    #[must_use]
    pub fn storage(error: impl std::fmt::Display) -> Self {
        Self::new("storage_failed", error.to_string())
    }
}
