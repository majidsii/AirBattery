//! Chooses the native status surface for the current operating system and desktop session.

use airbattery_settings::AppSettings;

use crate::model::PlatformKind;

/// Desktop shell relevant to native status-surface selection.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DesktopEnvironment {
    /// GNOME Shell or an Ubuntu GNOME session.
    Gnome,
    /// Any other desktop shell.
    Other,
}

/// Detects GNOME without spawning a process or depending on shell commands.
#[must_use]
pub fn detect_desktop_environment() -> DesktopEnvironment {
    desktop_environment_from_values(
        std::env::var("XDG_CURRENT_DESKTOP").ok().as_deref(),
        std::env::var("XDG_SESSION_DESKTOP").ok().as_deref(),
    )
}

/// Pure parser used by runtime detection and unit tests.
#[must_use]
pub fn desktop_environment_from_values(
    current_desktop: Option<&str>,
    session_desktop: Option<&str>,
) -> DesktopEnvironment {
    let is_gnome = [current_desktop, session_desktop]
        .into_iter()
        .flatten()
        .flat_map(|value| value.split([':', ';']))
        .map(str::trim)
        .any(|value| {
            value.eq_ignore_ascii_case("gnome")
                || value.eq_ignore_ascii_case("ubuntu")
                || value.to_ascii_lowercase().contains("gnome")
        });

    if is_gnome {
        DesktopEnvironment::Gnome
    } else {
        DesktopEnvironment::Other
    }
}

/// Returns whether the Tauri tray/AppIndicator should be visible.
///
/// GNOME uses the Shell extension as its only status surface when integration
/// is enabled. Other Linux desktops use the `AppIndicator` fallback, while
/// Windows uses the native system tray.
#[must_use]
pub const fn should_show_native_tray(
    platform: PlatformKind,
    desktop: DesktopEnvironment,
    settings: &AppSettings,
) -> bool {
    if !settings.show_tray_icon {
        return false;
    }

    match platform {
        PlatformKind::Linux => {
            !(matches!(desktop, DesktopEnvironment::Gnome) && settings.enable_gnome_integration)
        }
        PlatformKind::Windows | PlatformKind::Unsupported => true,
    }
}

#[cfg(test)]
mod tests {
    use airbattery_settings::AppSettings;

    use super::{DesktopEnvironment, desktop_environment_from_values, should_show_native_tray};
    use crate::model::PlatformKind;

    #[test]
    fn ubuntu_and_gnome_session_names_are_detected_without_case_sensitivity() {
        assert_eq!(
            desktop_environment_from_values(Some("ubuntu:GNOME"), None),
            DesktopEnvironment::Gnome
        );
        assert_eq!(
            desktop_environment_from_values(None, Some("Gnome-Wayland")),
            DesktopEnvironment::Gnome
        );
        assert_eq!(
            desktop_environment_from_values(Some("KDE"), Some("plasma")),
            DesktopEnvironment::Other
        );
    }

    #[test]
    fn gnome_extension_replaces_the_linux_tray_but_not_the_windows_tray() {
        let settings = AppSettings::default();

        assert!(!should_show_native_tray(
            PlatformKind::Linux,
            DesktopEnvironment::Gnome,
            &settings
        ));
        assert!(should_show_native_tray(
            PlatformKind::Linux,
            DesktopEnvironment::Other,
            &settings
        ));
        assert!(should_show_native_tray(
            PlatformKind::Windows,
            DesktopEnvironment::Gnome,
            &settings
        ));
    }

    #[test]
    fn disabling_gnome_integration_restores_the_linux_fallback_tray() {
        let settings = AppSettings {
            enable_gnome_integration: false,
            ..AppSettings::default()
        };

        assert!(should_show_native_tray(
            PlatformKind::Linux,
            DesktopEnvironment::Gnome,
            &settings
        ));
    }
}
