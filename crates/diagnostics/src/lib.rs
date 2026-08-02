//! Privacy-preserving diagnostic model and identifier sanitization.

use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use time::OffsetDateTime;

/// Creates a stable, non-reversible short identifier for logs.
#[must_use]
pub fn sanitize_identifier(identifier: &str) -> String {
    let digest = Sha256::digest(identifier.as_bytes());
    format!("bt-{}", hex::encode(&digest[..8]))
}

/// Sanitized backend status suitable for export.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DiagnosticReport {
    /// Application version.
    pub version: String,
    /// Operating system description.
    pub operating_system: String,
    /// Bluetooth backend state.
    pub backend_state: String,
    /// Sanitized device identifiers.
    pub devices: Vec<String>,
    /// Report timestamp.
    pub generated_at: OffsetDateTime,
}
