# Testing

## Ubuntu release-candidate gate

Run the complete gate from the repository root on Ubuntu 26.04:

```bash
CARGO_INCREMENTAL=0 \
CARGO_PROFILE_DEV_DEBUG=0 \
CARGO_PROFILE_TEST_DEBUG=0 \
CARGO_BUILD_JOBS=2 \
./RUN_UBUNTU_VALIDATION.sh
```

A successful run must finish with:

```text
VALIDATION, BUILD, AND GNOME EXTENSION INSTALL COMPLETED
```

That marker is valid only for the commit printed in the same validation log.

## Delivery-environment evidence

The following commands were run on `feature/hybrid-native` after the battery-monitor, native-surface, and artwork-policy changes.

### Desktop tests

```bash
npm --prefix apps/desktop test
```

Recorded result: **45 passed, 0 failed**.

The suite covers exact-or-generic artwork selection, truthful missing values, zero-percent handling, stale labels, active-device selection, AirPods slots, exact accessory presentation, native-surface selection, settings normalization, and visual priority.

### GNOME extension tests

```bash
node --test apps/gnome-extension/tests/*.test.mjs
```

Recorded result: **18 passed, 0 failed**.

The suite covers cached snapshot refresh, service recovery, overlap prevention, active-device visibility, fresh exact accessory evidence, component labels, compact percentages, input normalization, and AirPods `L/R/C` summaries.

### Dependency-free source contracts

```bash
python3 scripts/verify_reference_models.py
python3 scripts/verify-desktop-source.py
python3 scripts/verify-tauri-source.py
python3 scripts/verify-dbus-source.py
python3 scripts/verify-gnome-source.py
python3 scripts/verify-packaging-source.py
```

Recorded results for the current branch:

- reference models: **4 passed**;
- desktop source: **232 passed**;
- Tauri/native source: **156 passed**;
- D-Bus source: **51 passed**;
- GNOME source: **63 passed** after active-device and no-direct-Bluetooth checks;
- packaging source: **54 passed**.

The final delivery verification reruns these commands and records the fresh counts in the delivery manifest.

### Release and Ubuntu-script tests

```bash
python3 -m unittest \
  tests/release_artifacts_test.py \
  tests/ubuntu_validation_script_test.py
```

Recorded result: **13 passed, 0 failed**.

## Required native checks on Ubuntu

These checks require the Rust toolchain and native development libraries:

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
npm --prefix apps/desktop run typecheck
npm --prefix apps/desktop run build
npm --prefix apps/desktop run tauri build -- --no-bundle
```

The delivery sandbox does not contain Cargo or Ubuntu's native WebKit/BlueZ runtime, so those commands are never reported as passed there. They must be run against the exact imported commit on the user's Ubuntu host.

## Tauri launch modes

Development:

```bash
npm --prefix apps/desktop run tauri dev
```

Release binary:

```bash
npm --prefix apps/desktop run tauri build -- --no-bundle
./target/release/airbattery
```

Running the desktop host directly with `cargo run` is not the supported frontend workflow because it can bypass Tauri CLI's dev-server/build coordination.

## Hardware and platform gates

Automated tests do not prove Bluetooth hardware behavior. Before merging into `main`, record:

- Ubuntu 26.04, GNOME 50, Wayland;
- AirPods Pro first-generation left/right/case behavior;
- Bluetooth toggle, BlueZ restart, suspend/resume, disconnect/reconnect;
- GNOME indicator visibility with zero and one active devices;
- absence of a duplicate AppIndicator in GNOME;
- Windows 10/11 compile, tray, sleep/resume, installer, and uninstall behavior when Windows release support is claimed.

Use [`HARDWARE_TESTS.md`](HARDWARE_TESTS.md) for the exact matrix.
