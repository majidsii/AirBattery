# Project Status

Last updated: 2026-08-01


## AirPods Pro 2020 hardware-capture checkpoint (2026-08-01)

- A 100-second raw BlueZ/HCI capture from the user's AirPods Pro 2020 (`SHABIN`) contains valid Apple `0x004C` advertisements across lid and earbud transitions.
- The existing parser layout matches the captured payloads; regression fixtures now cover both-seated, left-out, and both-out states.
- Automatic Linux discovery now uses a serialized two-second scan on a three-second refresh cadence rather than waiting up to a minute.
- Rotating advertisements now correlate with exactly one paired Apple audio identity through the BlueZ Device ID vendor, even while the audio profile is disconnected.
- Physical rebuilt-application/UI validation is still pending on the user's Ubuntu host.
- This environment still lacks Cargo, so Rust formatting, compilation, clippy, and Rust test execution remain unverified here.

## Current phase

Phase 2 — desktop, native adapters, and GNOME integration source. The versioned session D-Bus service, GNOME 50 extension, and Windows standard-Battery source now exist. The next milestone is release engineering, long-lived backend recovery, and compilation/runtime validation.

## Repository state

- Branch: `feat/phase1-foundation`
- Worktree: `/mnt/data/airbattery/.worktrees/phase1-foundation`
- Current implementation checkpoint: `291b9d7` (`fix: bound GNOME D-Bus request handling`)
- This status document is maintained as a separate documentation checkpoint.
- Version: `0.1.0-alpha.1`
- Working tree expectation after this status update: clean.

## Completed source work

### Shared core and protocols

- Modular Rust 2024 workspace with platform-neutral, strongly typed battery/device models.
- Checked percentages that preserve the distinction between `0`, unavailable, unsupported, disconnected, and stale.
- Deterministic component-wise provider resolver with freshness and confidence ordering.
- AirPods proximity-pairing parser with synthetic fixtures and an independent Python reference verifier.
- Generic platform battery and standard BLE Battery Service providers.
- Registry retention for intermittent case data, duplicate suppression, stale expiry, and out-of-order rejection.
- Stateful `ApplicationEngine` joining private backend identifiers to privacy-safe public devices.
- RFC 3339 serialization contract for all public device/component timestamps.

### Linux

- BlueZ adapter status and known-device reads through `bluer`.
- Two-second bounded automatic discovery on a three-second serialized cadence instead of permanent high-duty scanning.
- BlueZ connection, platform battery, and manufacturer-data observation mapping.
- Privacy-safe aliases; raw Bluetooth addresses are not exposed as display names.
- Diagnostic CLI source and identifier sanitization.

### Windows

- Isolated Windows Runtime adapter source using `windows` `0.62.2`.
- Default adapter/radio state and paired BLE enumeration.
- Connection-state mapping.
- Asynchronous standard GATT Battery Service (`0x180F`) and Battery Level (`0x2A19`) reads.
- Invalid/empty battery values are ignored rather than converted to zero.
- Windows compile/runtime status remains unverified.

### Desktop frontend

- Vue 3, TypeScript, Pinia, and Vite application source.
- Battery, devices, settings, diagnostics, about, and compact widget surfaces.
- Liquid-glass design tokens with dark/light, no-blur, high-readability, and reduced-motion fallbacks.
- Keyboard skip navigation and text/ARIA battery state labels.
- Browser fallback that never invents devices or percentages.
- Event-driven device, backend-status, and native navigation bridges.
- Versioned settings normalization and Rust atomic persistence.

### Tauri host

- Single-instance lifecycle, tray menu, close-to-background behavior, per-user autostart, notification initialization, and window-state persistence source.
- Typed allow-listed commands for devices, backend status, refresh, settings, diagnostics, widget, and settings navigation.
- Persistent normalization engine across scans, retaining intermittent data only until freshness expiry.
- Independent deduplication for device snapshots and backend-health events.
- Narrow frontend capability: event listen/unlisten only; no shell or broad filesystem permission.
- `.deb`, AppImage, and per-user NSIS target configuration authored but not built.

### GNOME 50 companion

- Versioned session D-Bus contract:
  - bus `io.github.airbattery.Service`;
  - object `/io/github/airbattery/Service`;
  - interface `io.github.airbattery.Service1`.
- Complete snapshot, bounded refresh, settings, main-window, and `SnapshotChanged` operations.
- Bounded internal request queue and ten-second response timeout.
- GNOME 50 ESM top-bar extension with multiple devices, component states, stale/charging labels, manual refresh, preferences, and original symbolic icon.
- Event-driven updates with no extension-side Bluetooth parsing or continuous polling.
- Signal cleanup and service-owner recovery paths.
- Per-user install and package scripts; packaging currently blocked by the absent schema compiler.

## Remaining

- Install Rust and frontend dependencies, then resolve actual compiler, clippy, Vue SFC, and production-build findings.
- Add long-lived BlueZ property/device streams, adapter hot-plug, BlueZ restart recovery, and suspend/resume recovery.
- Compile and load the D-Bus service under a real user session bus.
- Validate the GNOME 50 extension on Ubuntu 26.04 GNOME Wayland.
- Capture AirPods Pro 2020 advertisements and complete the hardware matrix.
- Compile and validate the Windows Runtime adapter on Windows 10 and Windows 11.
- Add live Windows watchers/callback cleanup beyond snapshot collection.
- Add application icons in every required Tauri size, AppStream and desktop metadata, package hooks, CI release workflows, and checksums.
- Generate and smoke-test `.deb`, AppImage, GNOME ZIP, and NSIS artifacts.
- Complete public project/release documentation and item-by-item Definition of Done review.

## Architecture decisions

See `docs/DECISIONS.md` and `docs/superpowers/specs/2026-07-29-airbattery-design.md`.

## Known limitations

- The current environment is a Debian container, not Ubuntu 26.04/GNOME 50/Wayland or Windows.
- No Bluetooth controller, BlueZ system bus, GNOME Shell, Windows host, or physical AirPods are available.
- The AirPods protocol is unofficial and advertisement payloads are not authenticated.
- Linux background behavior currently relies on explicit/initial bounded collections; the long-lived BlueZ recovery loop is not implemented.
- Windows device collection is snapshot-based; live watcher callbacks are not implemented.
- Rust, Vue SFC, Tauri, native package, and installer source has not been compiled here.
- No application installer or release artifact exists yet.

## Current blockers

### Toolchain and package access

- `rustc`, `cargo`, `rustfmt`, and `clippy` are not installed.
- `apt-get update` and direct external downloads are unavailable from the container.
- `node_modules` is absent and the configured npm registry is inaccessible.
- `glib-compile-schemas`, `gnome-extensions`, and a GNOME Shell runtime are absent.

These are execution-environment limitations, not passed gates. Source authoring and dependency-free verification continue.

### External validation

Physical AirPods, Ubuntu/GNOME runtime, Windows runtime, signing, and installer smoke tests require external environments. None is marked passed.

## Test and verification results

### Executed successfully

```text
$ node --experimental-strip-types --test apps/desktop/tests/*.test.ts
18 passed, 0 failed, 0 skipped
exit status: 0

$ node --test apps/gnome-extension/tests/snapshot.test.mjs
6 passed, 0 failed, 0 skipped
exit status: 0

$ tsc --noEmit --target ES2022 --module ESNext --moduleResolution Bundler \
    --strict --allowImportingTsExtensions \
    apps/desktop/src/types/native-runtime.d.ts \
    apps/desktop/src/domain/*.ts apps/desktop/src/api/backend.ts
exit status: 0

$ python3 scripts/verify_reference_models.py
reference checks: 4 passed
exit status: 0

$ python3 scripts/verify-desktop-source.py
desktop source checks: 188 passed
exit status: 0

$ python3 scripts/verify-tauri-source.py
Tauri source checks: 127 passed
exit status: 0

$ python3 scripts/verify-dbus-source.py
D-Bus source checks: 51 passed
exit status: 0

$ python3 scripts/verify-gnome-source.py
GNOME source checks: 47 passed
exit status: 0

$ git diff --check
exit status: 0
```

### Explicit prerequisite failures

```text
$ apps/gnome-extension/scripts/validate.sh
6 GNOME snapshot tests passed
D-Bus source checks: 51 passed
GNOME source checks: 47 passed
glib-compile-schemas is required for the GNOME schema validation stage.
exit status: 127

$ ./scripts/package-gnome-extension.sh
glib-compile-schemas is required to package the AirBattery GNOME extension.
exit status: 1
```

No GNOME archive was generated.

The foundation script reaches the explicit Cargo prerequisite after dependency-free checks and exits `127`; no Cargo formatting, lint, test, or build stage has executed.

### Authored but not executed

- Rust unit/integration tests, including D-Bus timeout and Tauri state tests.
- `cargo fmt --all -- --check`: NOT RUN.
- `cargo clippy --workspace --all-targets -- -D warnings`: NOT RUN.
- `cargo test --workspace`: NOT RUN.
- `cargo build --workspace --release`: NOT RUN.
- Vue SFC typecheck and Vite production build: NOT RUN.
- Tauri application launch and package generation: NOT RUN.
- GNOME schema compilation/runtime loading: NOT RUN.
- Windows target compilation/runtime/installer: NOT RUN.

## Build results

- Node.js: `22.16.0` available.
- npm: `10.9.2` available; dependency registry inaccessible.
- Dependency-free TypeScript CLI: available.
- Git and Python: available.
- Rust/Cargo: unavailable.
- GNOME schema compiler/runtime: unavailable.
- Desktop binary: not generated.
- Linux packages: not generated.
- GNOME ZIP: not generated.
- Windows installer: not generated.

## Exact next action

Complete the release-engineering source checkpoint (public docs, metadata, CI, artifact/checksum scripts), then use the first network-enabled Ubuntu or CI environment to run:

```bash
cd /path/to/airbattery
./scripts/bootstrap-rust.sh
npm --prefix apps/desktop install
./scripts/verify-foundation.sh
npm --prefix apps/desktop run typecheck
npm --prefix apps/desktop run test:ui
npm --prefix apps/desktop run build
cargo check -p airbattery-desktop
apps/gnome-extension/scripts/validate.sh
```

Every actual compiler, linter, test, schema, or package failure must be fixed before hardware validation.

## Resume commands

```bash
cd /mnt/data/airbattery/.worktrees/phase1-foundation
git status --short --branch
git log --oneline -15
cat docs/PROJECT_STATUS.md
./scripts/verify-foundation.sh
```
