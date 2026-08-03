# Project Status

## Current branch goal

`feature/hybrid-native` stabilizes the Apple accessory monitor lifecycle, supports renamed AirPods after safe advertisement correlation, enforces one native status surface per platform/session, and removes misleading product photos from production selection.

## Implemented in source

- normalized provider/registry architecture;
- BlueZ known-device and bounded advertisement collection;
- long-lived Apple accessory `L2CAP` monitors with cancellation and reconnect;
- exact packet timestamps retained for registry-managed freshness;
- safe renamed-AirPods correlation without guessing among ambiguous devices;
- `127`/`255` unavailable handling and last-verified-value retention;
- Linux session D-Bus snapshot service;
- GNOME 50 extension that hides without an active device;
- GNOME extension/AppIndicator mutual exclusion;
- Windows-specific native tray selection;
- exact-or-generic artwork registry with an empty review gate for offline 3D assets.

## Delivery-environment evidence

- desktop Node tests: 45 passed;
- GNOME Node tests: 18 passed;
- release/Ubuntu Python tests: 13 passed;
- dependency-free source verifiers: passing.

Fresh exact counts and commit IDs are generated in the final delivery manifest.

## Still platform-gated

- Rust formatting, Clippy, workspace tests, and release build for the final branch;
- Ubuntu 26.04/GNOME 50 runtime after importing the final bundle;
- physical AirPods exact battery and reconnect matrix;
- Windows compile/runtime/NSIS smoke tests;
- production-quality model-accurate 3D asset creation and visual approval.

No platform-gated item is represented as passed without recorded evidence from that platform and exact commit.

## Next release gate

1. import `feature/hybrid-native` onto the canonical repository;
2. run `RUN_UBUNTU_VALIDATION.sh`;
3. execute the hardware matrix in `docs/HARDWARE_TESTS.md`;
4. fix any native-only failure on the feature branch;
5. merge with `--ff-only` after the branch is green;
6. push `main` to `https://github.com/majidsii/AirBattery`.
