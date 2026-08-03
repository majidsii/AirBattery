# Changelog

All notable changes to AirBattery are recorded here. The project follows Semantic Versioning once public releases begin.

## [Unreleased]

### Added

- Long-lived Apple accessory monitors with cancellation, reconnect handling, and one monitor per normalized Bluetooth address.
- Safe correlation of renamed AirPods with Apple proximity advertisements before starting the exact accessory channel.
- Exact-or-generic product-artwork registry for audited offline 3D renders.
- Hardware validation matrices for AirPods, GNOME Shell, generic Bluetooth devices, and future Windows releases.

### Changed

- GNOME consumes cached service snapshots and hides its indicator when no supported Bluetooth device is active.
- GNOME integration and Linux AppIndicator are mutually exclusive; Windows retains its own native tray path.
- Battery information has higher visual priority than decorative product artwork.
- Canonical repository metadata now points to `https://github.com/majidsii/AirBattery`.

### Fixed

- Treat Apple accessory values `0x7F` and `0xFF` as unavailable instead of displaying invalid percentages.
- Preserve the last verified component value when a later packet temporarily omits that component.
- Continue monitoring renamed AirPods after safe advertisement correlation.
- Stop accessory monitor tasks when BlueZ becomes unavailable or known-device collection fails.
- Prevent a missing case reading from becoming a false `0%` value.

### Removed

- Removed the former 87-file model-specific SVG catalog from production artwork selection.
- Removed the experimental licensed-photo importer and generated attribution runtime path.
- Removed synthetic studio renders and AI-generated product approximations from production assets.
- Removed model substitution and horizontal mirroring from exact artwork resolution.

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

No public release artifact has been generated. Native runtime, packaging, Windows execution, GNOME runtime, and physical AirPods validation remain platform-gated until recorded against the exact release-candidate commit.
