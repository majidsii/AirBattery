# Testing

## One-command gate

```bash
./scripts/verify-foundation.sh
```

The script runs every dependency-free gate first, scans production and documentation for unfinished markers, and then runs Rust formatting, linting, tests, and release build when Cargo is available.

In the current container the dependency-free portion passes and the script exits `127` at the explicit Cargo prerequisite check. That exit is not a Rust test result.

## Executed dependency-free verification

### Desktop domain and native bridge

```bash
node --experimental-strip-types --test apps/desktop/tests/*.test.ts
```

Recorded result: 18 passed, 0 failed, 0 skipped.

### Strict TypeScript boundary

```bash
TERM=dumb tsc --noEmit --target ES2022 --module ESNext \
  --moduleResolution Bundler --strict --allowImportingTsExtensions \
  apps/desktop/src/types/native-runtime.d.ts \
  apps/desktop/src/domain/*.ts apps/desktop/src/api/backend.ts
```

Recorded result: exit status 0. This covers dependency-free domain logic and the typed Tauri boundary. Full Vue SFC checking still requires installed packages.

### GNOME snapshot logic

```bash
node --test apps/gnome-extension/tests/snapshot.test.mjs
node --check apps/gnome-extension/extension.js
node --check apps/gnome-extension/service.js
node --check apps/gnome-extension/prefs.js
```

Recorded result: 6 tests passed, 0 failed, 0 skipped; all four JavaScript syntax checks exited 0.

### Static contracts

```bash
python3 scripts/verify_reference_models.py
python3 scripts/verify-desktop-source.py
python3 scripts/verify-tauri-source.py
python3 scripts/verify-dbus-source.py
python3 scripts/verify-gnome-source.py
```

Recorded results:

- protocol reference checks: 4 passed;
- desktop source checks: 188 passed;
- Tauri source checks: 127 passed;
- D-Bus source checks: 51 passed;
- GNOME source checks: 47 passed.

These checks validate required files, JSON/TOML/XML contracts, exact D-Bus names, command registration, narrow Tauri permissions, GNOME 50 metadata, snapshot validation, missing UI states, marker scans, raw-address literals, shell execution, and panic-prone Rust calls. They do not replace compilation or runtime tests.

## Authored Rust coverage

Rust unit and integration tests cover:

- percentage validation and unavailable serialization;
- deterministic component resolution;
- AirPods nibble, model, orientation, battery, and charging parsing;
- generic battery normalization;
- freshness expiry and intermittent case retention;
- duplicate and out-of-order updates;
- provider orchestration;
- BlueZ property mapping;
- diagnostic identifier hashing;
- Tauri transactional state and independent device/backend change detection;
- versioned D-Bus snapshots and unsupported-schema rejection.

Run when Cargo is available:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace --release
```

## Frontend package gates

Run after dependency installation:

```bash
npm --prefix apps/desktop install
npm --prefix apps/desktop run typecheck
npm --prefix apps/desktop test
npm --prefix apps/desktop run test:ui
npm --prefix apps/desktop run build
```

## Runtime and hardware gates

Automated source tests are not hardware evidence. Runtime validation must separately cover:

- Ubuntu 26.04 GNOME 50 Wayland;
- BlueZ restart, Bluetooth toggle, suspend/resume, and audio playback;
- AirPods Pro 2020 hardware states listed in `docs/HARDWARE_TESTS.md`;
- Windows 10 and Windows 11 BLE and tray behavior;
- `.deb`, AppImage, NSIS, upgrade, and uninstall smoke tests.

## Environment limitation

Cargo, frontend package dependencies, GNOME runtime tools, Windows, a Bluetooth adapter, and AirPods are unavailable in the current container. Rust compilation, Vue SFC build, native runtime, packaging, and hardware scenarios are therefore `NOT RUN`, not passed.
