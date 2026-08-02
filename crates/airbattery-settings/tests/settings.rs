//! Integration tests for settings persistence and validation.

use std::fs;

use airbattery_settings::{AppSettings, SettingsRepository, Theme, UpdateMode};

#[test]
fn normalization_clamps_and_preserves_cross_field_invariants() {
    let mut settings = AppSettings::default();
    settings.appearance.transparency = 250;
    settings.notifications.low_threshold = 5;
    settings.notifications.critical_threshold = 90;
    settings.notifications.cooldown_minutes = 0;
    settings.hidden_device_ids = vec!["one".to_owned(), "one".to_owned(), String::new()];

    let normalized = settings.normalized();
    assert_eq!(normalized.appearance.transparency, 100);
    assert_eq!(normalized.notifications.critical_threshold, 5);
    assert_eq!(normalized.notifications.cooldown_minutes, 1);
    assert_eq!(normalized.hidden_device_ids, vec!["one"]);
}

#[test]
fn corrupt_file_returns_defaults_with_recovery_flag() {
    let directory = tempfile::tempdir().unwrap_or_else(|error| panic!("tempdir failed: {error}"));
    let path = directory.path().join("settings.json");
    fs::write(&path, "{not valid json")
        .unwrap_or_else(|error| panic!("fixture write failed: {error}"));

    let result = SettingsRepository::new(path)
        .load()
        .unwrap_or_else(|error| panic!("load failed: {error}"));
    assert!(result.recovered_from_corrupt);
    assert_eq!(result.settings, AppSettings::default());
}

#[test]
fn settings_round_trip_uses_atomic_repository_path() {
    let directory = tempfile::tempdir().unwrap_or_else(|error| panic!("tempdir failed: {error}"));
    let path = directory.path().join("nested/settings.json");
    let repository = SettingsRepository::new(path.clone());
    let mut settings = AppSettings::default();
    settings.appearance.theme = Theme::Dark;
    settings.update_mode = UpdateMode::Manual;

    repository
        .save(&settings)
        .unwrap_or_else(|error| panic!("save failed: {error}"));
    let loaded = repository
        .load()
        .unwrap_or_else(|error| panic!("load failed: {error}"));

    assert_eq!(loaded.settings.appearance.theme, Theme::Dark);
    assert_eq!(loaded.settings.update_mode, UpdateMode::Manual);
    assert!(!loaded.recovered_from_corrupt);
    assert!(path.exists());
    assert!(!path.with_extension("json.tmp").exists());
}
