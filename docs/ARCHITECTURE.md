# AirBattery Architecture

## Goals

AirBattery normalizes battery information from operating-system Bluetooth APIs and manufacturer advertisements without inventing unavailable values. A single desktop process owns Bluetooth access and publishes privacy-safe snapshots to every presentation surface.

## Workspace boundaries

| Component | Responsibility | Platform coupling |
|---|---|---|
| `shared-models` | Validated percentages, components, device state, capabilities, RFC 3339 serialization | None |
| `device-protocols` | Raw observations, provider contracts, deterministic component resolution | None |
| `protocol-airpods` | Pure Apple proximity-pairing parser and provider | None |
| `protocol-generic-battery` | Standard and aggregate battery normalization | None |
| `airbattery-core` | Observation routing, freshness, registry, provider orchestration | None |
| `airbattery-service` | Persistent application engine over the core registry | None |
| `airbattery-settings` | Versioned settings normalization and atomic persistence | Filesystem only |
| `bluetooth-linux` | BlueZ status, known-device reads, bounded discovery, observation mapping | Linux |
| `bluetooth-windows` | WinRT radio, paired BLE, connection, and standard Battery Service collection | Windows |
| `airbattery-dbus` | Versioned session D-Bus snapshot and request contract | Linux desktop session |
| `diagnostics` | Sanitized identifiers and diagnostic report model | None |
| `airbattery-cli` | Hardware/parser diagnostic entry point | Linux in the current phase |
| `apps/desktop` | Vue presentation, Tauri lifecycle, tray, widget, native commands | Linux and Windows |
| `apps/gnome-extension` | GNOME 50 top-bar presentation over D-Bus | GNOME Shell 50 |

## Bluetooth data flow

1. A platform adapter emits a `RawObservation` with an internal backend identifier, source payload, and observation timestamp.
2. `ObservationProcessor` handles connection observations directly and offers battery/manufacturer observations only to compatible providers.
3. Providers validate and normalize data into `ProviderReport` values.
4. `DeviceRegistry` rejects duplicate and out-of-order reports and preserves intermittent values only until their original freshness deadline.
5. `resolve_reports` selects each component independently. Its ordering favors a known value, non-stale data, confidence, source quality, freshness, provider priority, and deterministic provider id.
6. `expire_components` marks old or disconnected values stale without converting them to zero.
7. The persistent `ApplicationEngine` produces privacy-safe `BluetoothAudioDevice` snapshots.

## Desktop process

The Tauri process is single-instance and owns:

- the persistent normalization engine;
- native platform collection;
- settings persistence;
- tray and window lifecycle;
- diagnostic export;
- the optional GNOME session service.

The Vue frontend receives complete device and backend snapshots through allow-listed commands and events. It has no direct filesystem, shell, Bluetooth, autostart, or notification plugin permission.

## GNOME process boundary

When enabled, the desktop process owns `io.github.airbattery.Service` on the user session bus. The GNOME extension calls a versioned interface and renders the returned JSON. It contains no Bluetooth or manufacturer protocol logic.

The `Refresh` D-Bus method is routed through a bounded native refresh and returns the resulting snapshot. Settings and window actions are routed back to the Tauri process. Dropping the service handle releases the D-Bus ownership when integration is disabled.

## Persistence and privacy

Raw Bluetooth addresses remain inside platform collection boundaries. Public device ids are privacy-safe aliases. Settings store only the preferred and hidden privacy-safe ids plus user preferences. Diagnostic exports contain sanitized ids and application-controlled metadata.

Settings are normalized, versioned, written to a temporary file, and atomically replaced. Invalid or corrupt settings fall back safely instead of crashing the application.

## Error policy

- Recognized malformed payloads produce typed errors.
- Unrecognized payloads produce no report rather than a fabricated report.
- Missing adapter, powered-off adapter, permission failure, and backend failure remain distinguishable.
- BlueZ is never powered on or reconfigured implicitly.
- Native integration failures are logged with sanitized messages and do not crash the frontend.
- No production path fabricates a battery percentage or charging state.

## Validation status

Dependency-free TypeScript tests and source-contract checks are passing in the current container. Rust compilation, native application launch, GNOME runtime, Linux Bluetooth hardware, AirPods hardware, Windows runtime, and package installation remain unverified because the required toolchains/platforms/hardware are unavailable here.
