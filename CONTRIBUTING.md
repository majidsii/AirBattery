# Contributing to AirBattery

AirBattery welcomes focused, test-backed contributions. The project is still in alpha development, so compatibility claims must remain evidence-based.

## Before opening a change

1. Read `docs/ARCHITECTURE.md`, `docs/DECISIONS.md`, and `docs/PROJECT_STATUS.md`.
2. Search existing issues and pull requests when the public repository is available.
3. Keep platform-specific code behind the existing adapter boundaries.
4. Do not add telemetry, cloud requirements, raw Bluetooth identifier logging, or fabricated battery values.

## Development setup

```bash
cd /path/to/airbattery
./scripts/bootstrap-rust.sh
npm --prefix apps/desktop install
./scripts/verify-foundation.sh
npm --prefix apps/desktop run typecheck
npm --prefix apps/desktop run test:ui
```

See `docs/BUILDING.md` for platform dependencies.

## Change requirements

- Write a failing test or contract check before production behavior changes.
- Preserve the distinction between known zero, unavailable, unsupported, stale, and disconnected.
- Keep AirPods parser fixtures separate from physical-device evidence.
- Add typed errors instead of silent fallback in native code.
- Avoid `unwrap()`, `expect()`, broad Tauri permissions, and shell execution in production paths.
- Update `docs/PROJECT_STATUS.md` and relevant architecture/protocol documentation.
- Add an ADR when a change affects a long-lived architectural contract.

## Required verification

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace --release
npm --prefix apps/desktop run typecheck
npm --prefix apps/desktop test
npm --prefix apps/desktop run test:ui
npm --prefix apps/desktop run build
```

Run platform-specific runtime and package smoke tests when the change affects Linux, GNOME, or Windows. A skipped or unavailable test must be reported as `NOT RUN`, never passed.

## Commit and pull-request quality

- Keep commits reviewable and scoped to one behavior or checkpoint.
- Explain the evidence behind protocol and compatibility changes.
- Include exact commands and results in the pull request.
- Do not include captured Bluetooth addresses, secrets, signing credentials, or private diagnostic bundles.

## Licensing

Contributions are accepted under the MIT license. Protocol contributions must be original, cleanly reimplemented from legally usable interoperability facts, and accompanied by source/license notes.
