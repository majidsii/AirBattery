# AirBattery

AirBattery is a local-first Bluetooth battery monitor for Linux and Windows. It prioritizes truthful component data for earbuds and headsets: a missing value stays unavailable, stale values are labelled, and one aggregate percentage is never duplicated into invented left/right/case values.

The current source version is `0.1.0-alpha.1`. Ubuntu 26.04/GNOME 50 is the primary runtime target; Windows 10/11 is supported by a separate native adapter and tray surface.

## Current architecture

```text
Bluetooth / vendor protocol
          ↓
Native provider adapters
          ↓
Normalized battery registry + freshness cache
          ↓
Platform snapshot service
   ┌──────┴────────┐
Tauri desktop   Linux D-Bus
                    ↓
              GNOME extension
```

- The Rust backend owns Bluetooth, protocol parsing, freshness, and device selection.
- The Vue desktop and GNOME extension only render normalized snapshots.
- On GNOME, the Shell extension replaces the Linux AppIndicator; Windows keeps its native notification-area icon.
- The GNOME indicator is hidden when no active supported Bluetooth device exists.
- Exact AirPods values use the Apple accessory channel when available; passive advertisements remain explicitly approximate.
- Product artwork follows an exact-or-generic policy. Unreviewed photos, AI lookalikes, and approximate model substitutions are rejected.

## Verification status

| Area | Status |
|---|---|
| Desktop domain tests | 45 passing in the delivery environment |
| GNOME extension tests | 18 passing in the delivery environment |
| Dependency-free source contracts | Passing |
| Release/Ubuntu script tests | 13 passing |
| Rust format, Clippy, workspace tests, Tauri build | Must be run on the Ubuntu development host for this branch |
| Physical AirPods battery validation | Must be repeated on the exact delivery commit |
| Windows runtime and installer smoke test | Platform-gated; not yet recorded |

See [`docs/TESTING.md`](docs/TESTING.md) and [`docs/HARDWARE_TESTS.md`](docs/HARDWARE_TESTS.md). A green source test is not treated as hardware evidence.

## Run in development

Install dependencies once, then use Tauri's development command so the WebView and Vite server are started together:

```bash
npm --prefix apps/desktop install
npm --prefix apps/desktop run tauri dev
```

Do not use `cargo run --manifest-path apps/desktop/src-tauri/Cargo.toml` as the normal desktop launch command. That bypasses Tauri CLI's frontend lifecycle and can leave the WebView pointing at an unavailable development URL.

## Build the native application

```bash
npm --prefix apps/desktop run build
npm --prefix apps/desktop run tauri build -- --no-bundle
./target/release/airbattery
```

For the complete Ubuntu gate, including source checks and GNOME extension installation:

```bash
CARGO_INCREMENTAL=0 \
CARGO_PROFILE_DEV_DEBUG=0 \
CARGO_PROFILE_TEST_DEBUG=0 \
CARGO_BUILD_JOBS=2 \
./RUN_UBUNTU_VALIDATION.sh
```

Do not build or run AirBattery as root.

## Project rules

- Unknown is not zero.
- `127` and `255` are unavailable sentinels, not percentages.
- The last verified component value keeps its original timestamp and may become stale; it is not silently relabelled as live.
- Bluetooth is never enabled, paired, or reconfigured implicitly.
- Raw Bluetooth identifiers do not cross into UI, D-Bus snapshots, settings, or exported diagnostics.
- Exact product art must match the exact model and include auditable authorship and redistribution terms.

## Documentation

- [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md)
- [`docs/AIRPODS_PROTOCOL.md`](docs/AIRPODS_PROTOCOL.md)
- [`docs/ARTWORK_PIPELINE.md`](docs/ARTWORK_PIPELINE.md)
- [`docs/GNOME_EXTENSION.md`](docs/GNOME_EXTENSION.md)
- [`docs/BLUETOOTH_BACKENDS.md`](docs/BLUETOOTH_BACKENDS.md)
- [`docs/TESTING.md`](docs/TESTING.md)
- [`docs/HARDWARE_TESTS.md`](docs/HARDWARE_TESTS.md)
- [`docs/PROJECT_STATUS.md`](docs/PROJECT_STATUS.md)

## Repository and license

Canonical repository: `https://github.com/majidsii/AirBattery`

AirBattery is MIT licensed. Protocol research attribution and clean-room notes are recorded in `crates/protocol-airpods/NOTICE.md`.
