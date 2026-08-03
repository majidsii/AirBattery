# Hybrid Native Delivery

This branch is designed to sit on top of the existing `main` history. It is not a replacement repository and does not require a force push.

## Branch

```text
feature/hybrid-native
```

## Implemented commits

The branch contains separate commits for:

- architecture and implementation plan;
- long-lived Apple accessory monitor lifecycle;
- renamed-AirPods advertisement correlation;
- exact-or-generic artwork policy;
- battery-first visual hierarchy;
- BlueZ failure-path monitor cleanup;
- canonical repository metadata;
- verification and hardware workflow documentation;
- audited offline-3D artwork metadata requirements.

Use this command after importing the delivery bundle to see the exact hashes:

```bash
git log --oneline main..feature/hybrid-native
```

## Verified in the delivery environment

- Desktop Node tests: 45 passed, 0 failed.
- GNOME extension Node tests: 18 passed, 0 failed.
- Strict dependency-free TypeScript boundary: passed.
- GNOME JavaScript syntax checks: passed.
- Reference checks: 4 passed.
- Desktop source checks: 238 passed.
- Tauri/native source checks: 156 passed.
- D-Bus source checks: 51 passed.
- GNOME source checks: 63 passed.
- Packaging source checks: 54 passed.
- Release/Ubuntu Python tests: 13 passed.
- `git diff --check`: passed.
- Git object integrity: passed; only harmless unreachable trees from local rebases were reported.

## Required before merging to `main`

The final branch must be validated on the Ubuntu development host because the delivery environment has no Cargo toolchain, native WebKit/BlueZ runtime, GNOME Shell session, Bluetooth controller, or physical AirPods.

```bash
CARGO_INCREMENTAL=0 \
CARGO_PROFILE_DEV_DEBUG=0 \
CARGO_PROFILE_TEST_DEBUG=0 \
CARGO_BUILD_JOBS=2 \
./RUN_UBUNTU_VALIDATION.sh
```

Then execute the AirPods and GNOME matrix in `docs/HARDWARE_TESTS.md` against the exact commit being merged.

## Import and merge policy

Import onto the existing repository, validate the feature branch, and merge with `--ff-only`. Do not replace the repository directory and do not force push.
