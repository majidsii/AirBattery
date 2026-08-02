//! Versioned, validated, and crash-resistant `AirBattery` settings persistence.

use std::{
    collections::BTreeSet,
    fs::{self, OpenOptions},
    io::Write,
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Current settings schema version.
pub const CURRENT_SCHEMA_VERSION: u16 = 1;

/// Application update behavior.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum UpdateMode {
    /// Download and apply supported updates automatically.
    Automatic,
    /// Notify the user before installing an update.
    Notify,
    /// Never check automatically.
    Manual,
}

/// Application color theme.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub enum Theme {
    /// Follow the operating-system preference.
    System,
    /// Always use the light palette.
    Light,
    /// Always use the dark palette.
    Dark,
}

/// Appearance preferences.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct AppearanceSettings {
    /// Selected color theme.
    pub theme: Theme,
    /// Glass opacity percentage in `0..=100`.
    pub transparency: u16,
    /// Use tighter component spacing.
    pub compact_layout: bool,
    /// Enable decorative interface animation.
    pub animations: bool,
    /// Force reduced motion independently of the operating system.
    pub reduced_motion: bool,
}

impl Default for AppearanceSettings {
    fn default() -> Self {
        Self {
            theme: Theme::System,
            transparency: 78,
            compact_layout: false,
            animations: true,
            reduced_motion: false,
        }
    }
}

/// Local notification preferences.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct NotificationSettings {
    /// Whether battery notifications are enabled.
    pub enabled: bool,
    /// Low-battery threshold percentage.
    pub low_threshold: u16,
    /// Critical-battery threshold percentage.
    pub critical_threshold: u16,
    /// Whether alerts should name the affected component.
    pub component_specific: bool,
    /// Minimum minutes before repeating the same alert.
    pub cooldown_minutes: u16,
    /// Whether meaningful connection transitions generate notifications.
    pub connection_events: bool,
}

impl Default for NotificationSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            low_threshold: 20,
            critical_threshold: 10,
            component_specific: true,
            cooldown_minutes: 30,
            connection_events: false,
        }
    }
}

/// Complete persisted application settings.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase", default)]
#[allow(clippy::struct_excessive_bools)]
pub struct AppSettings {
    /// Version used for future migrations.
    pub schema_version: u16,
    /// Start `AirBattery` for the current user after sign-in.
    pub start_with_system: bool,
    /// Hide rather than exit when the main window closes.
    pub run_in_background: bool,
    /// Create a tray/status icon where supported.
    pub show_tray_icon: bool,
    /// Publish the session D-Bus API for the GNOME extension.
    pub enable_gnome_integration: bool,
    /// Enable the optional floating widget.
    pub enable_desktop_widget: bool,
    /// Update-checking behavior.
    pub update_mode: UpdateMode,
    /// Privacy-safe preferred device identifier.
    pub preferred_device_id: Option<String>,
    /// Privacy-safe device identifiers hidden from the UI.
    pub hidden_device_ids: Vec<String>,
    /// Appearance preferences.
    pub appearance: AppearanceSettings,
    /// Notification preferences.
    pub notifications: NotificationSettings,
}

impl Default for AppSettings {
    fn default() -> Self {
        Self {
            schema_version: CURRENT_SCHEMA_VERSION,
            start_with_system: false,
            run_in_background: true,
            show_tray_icon: true,
            enable_gnome_integration: true,
            enable_desktop_widget: false,
            update_mode: UpdateMode::Notify,
            preferred_device_id: None,
            hidden_device_ids: Vec::new(),
            appearance: AppearanceSettings::default(),
            notifications: NotificationSettings::default(),
        }
    }
}

impl AppSettings {
    /// Applies schema migration and validation without failing on user input.
    #[must_use]
    pub fn normalized(mut self) -> Self {
        self.schema_version = CURRENT_SCHEMA_VERSION;
        self.appearance.transparency = self.appearance.transparency.min(100);
        self.notifications.low_threshold = self.notifications.low_threshold.min(100);
        self.notifications.critical_threshold = self
            .notifications
            .critical_threshold
            .min(self.notifications.low_threshold);
        self.notifications.cooldown_minutes = self.notifications.cooldown_minutes.clamp(1, 1_440);
        self.preferred_device_id = self
            .preferred_device_id
            .and_then(|value| non_empty(value.trim()).map(ToOwned::to_owned));

        let mut seen = BTreeSet::new();
        self.hidden_device_ids.retain(|identifier| {
            let trimmed = identifier.trim();
            !trimmed.is_empty() && seen.insert(trimmed.to_owned())
        });
        for identifier in &mut self.hidden_device_ids {
            *identifier = identifier.trim().to_owned();
        }
        self
    }
}

fn non_empty(value: &str) -> Option<&str> {
    (!value.is_empty()).then_some(value)
}

/// Result of loading settings, including non-fatal corruption recovery.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SettingsLoadResult {
    /// Valid normalized settings.
    pub settings: AppSettings,
    /// Whether invalid JSON was replaced in memory with defaults.
    pub recovered_from_corrupt: bool,
}

/// Settings storage failure unrelated to malformed user content.
#[derive(Debug, Error)]
pub enum SettingsError {
    /// File-system operation failed.
    #[error("settings I/O failed: {0}")]
    Io(#[from] std::io::Error),
    /// Serialization of validated settings failed.
    #[error("settings serialization failed: {0}")]
    Serialize(#[from] serde_json::Error),
}

/// Repository bound to one application-controlled settings path.
#[derive(Debug, Clone)]
pub struct SettingsRepository {
    path: PathBuf,
}

impl SettingsRepository {
    /// Creates a repository for an application-owned path.
    #[must_use]
    pub fn new(path: impl Into<PathBuf>) -> Self {
        Self { path: path.into() }
    }

    /// Returns the configured settings path.
    #[must_use]
    pub fn path(&self) -> &Path {
        &self.path
    }

    /// Loads settings, recovering from missing or malformed content.
    ///
    /// # Errors
    ///
    /// Returns [`SettingsError`] when the settings directory cannot be read,
    /// the stored file cannot be parsed, or invalid data cannot be quarantined.
    pub fn load(&self) -> Result<SettingsLoadResult, SettingsError> {
        if !self.path.exists() {
            return Ok(SettingsLoadResult {
                settings: AppSettings::default(),
                recovered_from_corrupt: false,
            });
        }

        let content = fs::read_to_string(&self.path)?;
        match serde_json::from_str::<AppSettings>(&content) {
            Ok(settings) => Ok(SettingsLoadResult {
                settings: settings.normalized(),
                recovered_from_corrupt: false,
            }),
            Err(_) => Ok(SettingsLoadResult {
                settings: AppSettings::default(),
                recovered_from_corrupt: true,
            }),
        }
    }

    /// Writes normalized settings through a temporary file in the same directory.
    ///
    /// # Errors
    ///
    /// Returns [`SettingsError`] when settings validation, serialization,
    /// directory creation, writing, or atomic replacement fails.
    pub fn save(&self, settings: &AppSettings) -> Result<(), SettingsError> {
        if let Some(parent) = self.path.parent()
            && !parent.as_os_str().is_empty()
        {
            fs::create_dir_all(parent)?;
        }

        let normalized = settings.clone().normalized();
        let serialized = serde_json::to_vec_pretty(&normalized)?;
        let temporary = self.path.with_extension("json.tmp");
        let mut file = OpenOptions::new()
            .create(true)
            .truncate(true)
            .write(true)
            .open(&temporary)?;
        file.write_all(&serialized)?;
        file.write_all(b"\n")?;
        file.sync_all()?;
        drop(file);

        replace_file(&temporary, &self.path)?;
        Ok(())
    }
}

#[cfg(not(target_os = "windows"))]
fn replace_file(source: &Path, destination: &Path) -> std::io::Result<()> {
    fs::rename(source, destination)
}

#[cfg(target_os = "windows")]
fn replace_file(source: &Path, destination: &Path) -> std::io::Result<()> {
    if destination.exists() {
        fs::remove_file(destination)?;
    }
    fs::rename(source, destination)
}
