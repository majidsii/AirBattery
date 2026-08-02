
## 2026-08-02 — Native surfaces and licensed exact artwork

- Removed synthetic studio renders from the product-artwork path.
- Added a build-time Wikimedia Commons importer that validates an allowlisted Creative Commons license before downloading exact model photographs.
- Added per-model/per-mode exact-photo overrides with the original 87 SVG assets retained as truthful fallbacks.
- Added generated in-app product photo credits and a machine-readable attribution manifest.
- Added an audited phase-one source catalog for AirPods Pro 1, Galaxy Buds Live, original Galaxy Buds, Nothing Ear (1), Pixel Buds Pro, Huawei FreeBuds 6, Xiaomi Buds 5, and Beats Studio Buds+.
- GNOME Shell now hides the AirBattery indicator when no Bluetooth device is actually connected and restores it immediately when a device connects.
- GNOME only shows the Shell extension when integration is enabled; the duplicate Linux AppIndicator is suppressed automatically.
- Windows continues to use the native Windows system tray, while non-GNOME Linux desktops retain the AppIndicator fallback.
- Platform-specific integration controls now show the correct wording and options for Linux and Windows.

# Changelog

All notable changes to AirBattery are recorded here. The project follows Semantic Versioning once public releases begin.

## [Unreleased]

### Fixed

- Treat Apple accessory battery value `0xFF` as temporarily unavailable, accept valid sibling components, and preserve the last valid case reading.
- Automatically show every connected Bluetooth device on the main battery dashboard without requiring manual selection.
- Add an original model-aware SVG catalog for Apple, QCY, Xiaomi, Soundcore, Samsung, Sony, JBL, Pixel, Nothing, OnePlus, Huawei, and Beats product families.
- Expand exact QCY matching across current earbuds, open-ear products, and H-series headphones.
- Refresh Apple BLE advertisements with a serialized two-second discovery window on a three-second desktop cadence instead of waiting up to a minute.
- Correlate rotating AirPods advertisements with one paired Apple audio identity using the BlueZ Device ID vendor (`0x004C`), including while the case is open but the audio profile is disconnected.
- Keep correlation conservative when more than one paired Apple device is present.
- Add AirPods Pro 2020 regression fixtures captured from real lid/earbud transitions.
- Keep navigation tooltip labels above the glass content panel with explicit stacking contexts.


## [0.1.0-alpha.1] - Unreleased

This version is a development target, not a published release. No installer or package is claimed until native builds, checksums, and smoke tests exist.


### Added

- Modular Rust workspace for shared battery models, provider resolution, settings, diagnostics, and platform adapters.
- AirPods proximity-pairing parser with synthetic fixtures and independent reference checks.
- BlueZ known-device and bounded discovery source.
- Windows Runtime standard Battery Service source.
- Vue 3/Tauri desktop application source with tray, floating widget, settings, diagnostics, and accessibility states.
- GNOME Shell 50 companion extension and versioned session D-Bus service.
- Dependency-free verification harnesses and hardware validation procedure.

### Security

- Narrow Tauri webview permissions.
- Privacy-safe device identifiers across frontend, diagnostics, and GNOME boundaries.
- Bounded, non-blocking GNOME request queue with a ten-second response timeout.

### Validation status

No public release artifact has been generated. Rust compilation, native runtime, packaging, Windows execution, GNOME runtime, and physical AirPods validation remain unverified in the current environment.

### Professional connected-device dashboard and artwork catalog

- Keep the last valid AirPods case value when AACP reports `0xFF` (temporarily unavailable) instead of rejecting the full battery report.
- Automatically render every connected Bluetooth device on the desktop battery page; manual selection is no longer required for the main dashboard.
- Add 87 original local SVG assets across 29 visual families and 98 model aliases, including Apple AirPods and QCY product families.
- Keep passive BLE values visibly approximate while preserving exact one-percent AACP values.
