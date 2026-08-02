# AirBattery

AirBattery is an in-development, local-first Bluetooth battery monitor for Linux and Windows. Its goal is an accurate, premium component-level experience for AirPods and other Bluetooth audio devices without fabricating unavailable values.

> The repository is at `0.1.0-alpha.1`. Source and dependency-free tests exist, but no native release artifact or hardware validation is complete yet.

## Current feature status

| Capability | Source status | Validation status |
|---|---|---|
| Cross-platform battery domain | Implemented | Dependency-free/reference checks pass; Rust not compiled here |
| Per-component provider resolver | Implemented | Fixture/reference tests authored; Cargo tests not run |
| AirPods proximity-pairing parser | Implemented | Synthetic fixtures only; physical hardware unverified |
| Linux BlueZ known-device/bounded refresh | Implemented | Runtime unverified |
| Standard/aggregate battery normalization | Implemented | Source/reference checks pass |
| Privacy-safe identifier handling | Implemented | Source/reference checks pass |
| Vue/Tauri desktop UI and widget | Implemented in source | Node domain tests pass; SFC/native build unverified |
| GNOME 50 top-bar companion | Implemented in source | 6 logic tests and source contracts pass; Shell runtime unverified |
| Windows WinRT standard battery backend | Implemented in source | Windows compile/runtime unverified |
| `.deb`, AppImage, GNOME ZIP, NSIS | Config/scripts authored | No artifact generated |

## Principles

- Unknown is not zero.
- Stale data is never shown as live data.
- Charging state is reported only when supported by reliable evidence.
- Bluetooth is never enabled or reconfigured implicitly.
- No telemetry, account, cloud dependency, or advertising is required.
- Raw Bluetooth identifiers do not leave native backend boundaries.

## Architecture

A single Rust/Tauri process owns Bluetooth access, protocol providers, freshness, settings, diagnostics, and platform lifecycle. Vue consumes allow-listed commands/events. The GNOME extension consumes a versioned per-user D-Bus snapshot and contains no Bluetooth parsing or polling.

See [`docs/ARCHITECTURE.md`](docs/ARCHITECTURE.md) and [`docs/DECISIONS.md`](docs/DECISIONS.md).

## Development

On a network-enabled development system:

```bash
./scripts/bootstrap-rust.sh
npm --prefix apps/desktop install
./scripts/verify-foundation.sh
npm --prefix apps/desktop run typecheck
npm --prefix apps/desktop run build
```

Linux diagnostic commands after a successful Rust build:

```bash
cargo run -p airbattery-cli -- status
cargo run -p airbattery-cli -- scan --seconds 20 --json
```

Do not build or run AirBattery as root.

## Installation status

No installable AirBattery release exists at this checkpoint. Intended commands and honest artifact prerequisites are documented in [`docs/INSTALLATION.md`](docs/INSTALLATION.md).

## Privacy

AirBattery is local-first and has no telemetry by default. Bluetooth addresses are retained only inside native collection boundaries and transformed into privacy-safe identifiers before reaching UI, D-Bus, settings, or diagnostic exports.

## Documentation

- [`docs/PROJECT_STATUS.md`](docs/PROJECT_STATUS.md) — exact state, evidence, blockers, and next action
- [`docs/BUILDING.md`](docs/BUILDING.md)
- [`docs/INSTALLATION.md`](docs/INSTALLATION.md)
- [`docs/BLUETOOTH_BACKENDS.md`](docs/BLUETOOTH_BACKENDS.md)
- [`docs/AIRPODS_PROTOCOL.md`](docs/AIRPODS_PROTOCOL.md)
- [`docs/GNOME_EXTENSION.md`](docs/GNOME_EXTENSION.md)
- [`docs/WINDOWS_BACKEND.md`](docs/WINDOWS_BACKEND.md)
- [`docs/TESTING.md`](docs/TESTING.md)
- [`docs/HARDWARE_TESTS.md`](docs/HARDWARE_TESTS.md)

## Contributing and release status

Public contribution, security, release, and support documents are being prepared before the first release. Support claims must always match recorded test evidence.

## License

MIT. Protocol research attribution and clean-room notes are recorded in `crates/protocol-airpods/NOTICE.md`.
