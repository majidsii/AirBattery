# AirBattery Design

Date: 2026-07-29
Status: Approved by autonomous execution mandate

## Product scope

AirBattery is a local-first Linux and Windows desktop application that normalizes Bluetooth battery information from native operating-system APIs and manufacturer-specific advertisements. Ubuntu 26/GNOME/Wayland is the primary target. Windows 10/11 is a secondary target. The first release prioritizes AirPods while preserving accurate partial-capability behavior for generic devices.

## Chosen approach

Use a Rust workspace with platform-independent domain and provider crates, Tauri 2 as the desktop shell, Vue 3/TypeScript for the UI, a Linux BlueZ adapter, a Windows WinRT adapter, and a GNOME Shell extension consuming a documented session D-Bus API.

Alternatives considered:

1. Electron: faster UI prototyping but materially higher idle memory and weaker native-service separation.
2. One monolithic Tauri crate: simpler initially but poor testability and platform isolation.
3. Standalone background daemon plus separate UI: strongest process isolation but unnecessarily complex for the first release. The design preserves a service boundary so this can be split later without changing domain types.

## Architecture

- `shared-models`: serializable domain types and value validation.
- `device-protocols`: provider trait, observations, deterministic merge/conflict resolution.
- `protocol-airpods`: pure Apple Continuity proximity-pairing parser and fixture tests.
- `protocol-generic-battery`: standard Battery Service and aggregate battery normalization.
- `airbattery-core`: device registry, freshness/staleness engine, settings, notifications, orchestration.
- `bluetooth-linux`: BlueZ discovery and property events; no UI dependencies.
- `bluetooth-windows`: WinRT device enumeration and GATT battery access behind `cfg(windows)`.
- `diagnostics`: structured sanitized diagnostic reports.
- `apps/desktop`: Tauri commands/events, tray, single instance, popup/widget windows, Vue UI.
- `apps/gnome-extension`: GJS top-bar client of `io.github.airbattery.Service1`.

## Data flow

1. A platform adapter emits raw device and advertisement observations.
2. Providers parse compatible observations into typed battery reports with source, confidence, and timestamp.
3. The resolver selects values component-by-component using validity, freshness, confidence, and deterministic provider priority.
4. The registry applies expiry rules and publishes normalized device snapshots.
5. Tauri emits snapshots to Vue and exports the same snapshots over session D-Bus for GNOME.
6. UI renders unknown, unsupported, unavailable, disconnected, and stale states distinctly; it never substitutes zero.

## AirPods strategy

Initial support parses unencrypted Apple company-id `0x004C` Continuity proximity-pairing message type `0x07`. Battery nibbles represent deciles; values above 10 are unavailable. Orientation determines left/right mapping. Charging bits are only surfaced when the message is complete and valid. Case data expires sooner than connected earbud data because case advertisements are intermittent.

Protocol facts are independently reimplemented and documented. MIT-licensed `hudsonbrendon/apple-ble` is used as a research reference. GPL projects such as LibrePods, CAPod, OpenPods, and AirStatus may be cited for interoperability research but no GPL source code is copied into the MIT codebase.

## Linux strategy

Use BlueZ over D-Bus through the maintained `bluer` crate for adapter/device/discovery events. Prefer event-driven updates and bounded scan windows. A scheduler enables discovery only when the preferred device lacks fresh data, after resume, or on explicit refresh. BlueZ `org.bluez.Battery1` and manufacturer data are both consumed. Suspend/resume and BlueZ owner changes trigger state reconciliation.

## Windows strategy

Use the `windows` crate and WinRT namespaces for `DeviceWatcher`, `BluetoothLEDevice`, `GattDeviceService`, and the standard Battery Service. Apple-specific component support is reported only when advertisements expose verifiable data. Windows build and runtime validation remain platform-gated in this Linux environment.

## GNOME integration

Export a session-bus service named `io.github.airbattery.Service` at `/io/github/airbattery/Service` with interface `io.github.airbattery.Service1`. Methods return current snapshots and open settings. Signals announce device and backend changes. The extension performs no Bluetooth parsing and disconnects signals cleanly on disable.

## UI

A compact battery popup, devices screen, settings, diagnostics, and about screen share a lightweight CSS design system. Translucency is progressive enhancement; opaque fallback and high-contrast variables preserve readability. All interactive controls are keyboard accessible, screen-reader labeled, and respect reduced motion.

## Persistence and security

Settings are versioned JSON stored in the OS app config directory with atomic write/rename and safe defaults on corruption. Bluetooth identifiers are hashed for logs unless explicit diagnostic detail is enabled. Tauri commands are narrowly typed; no arbitrary shell or path access is exposed.

## Testing

- Unit: domain validation, stale transitions, resolver, AirPods fixtures, settings migrations, cooldowns.
- Integration: registry event flow, mocked platform adapter, D-Bus serialization contract.
- Frontend: store and component states with Vitest; accessibility assertions where feasible.
- Packaging: build and launch smoke scripts.
- Hardware: a one-command Linux capture/diagnostic workflow; physical AirPods Pro 2020 results are never inferred.

## Delivery sequence

1. Workspace, domain model, parser, resolver, tests.
2. Linux BlueZ adapter and diagnostic CLI.
3. Tauri/Vue application and persistence.
4. GNOME extension and D-Bus contract.
5. Windows adapter and CI build path.
6. Packaging, audits, documentation, release verification.

## Explicit acceptance limitations in this environment

The current execution environment is Debian 13 in a container, without a Bluetooth adapter, GNOME Shell session, Windows host, or physical AirPods. Automated implementation and compilation can be completed here once dependencies are available. Physical-device, GNOME runtime, Windows runtime, and installer runtime validation require external environments and will remain marked unverified until evidence is captured.
