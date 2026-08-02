# AirBattery Foundation and Linux MVP Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build and verify the platform-independent battery domain, provider resolver, AirPods parser, freshness registry, and a Linux BlueZ-backed diagnostic path.

**Architecture:** A Rust workspace separates serializable domain types, protocol providers, application orchestration, and Linux I/O. All protocol parsing is pure and fixture-tested. Linux Bluetooth events enter through a trait so registry and provider behavior remain testable without a real adapter.

**Tech Stack:** Rust 2024 edition, Cargo workspace, serde, thiserror, time, tokio, async-trait, bluer, futures, tracing, clap.

## Global Constraints

- Never synthesize battery values or charging states.
- Unknown and unsupported are distinct from zero.
- Platform-specific code must not leak into shared models.
- Production code is written only after observing a failing test.
- No `unwrap()` in production paths unless an invariant is proven in the same scope.
- Linux scanning must be bounded and disabled when unnecessary.
- The current container cannot validate physical Bluetooth behavior; runtime claims remain unverified.

---

## File map

- `Cargo.toml`: workspace members and shared dependency versions.
- `rust-toolchain.toml`: pinned stable toolchain and components.
- `crates/shared-models/src/*`: domain enums, battery components, devices, validation.
- `crates/device-protocols/src/*`: provider observations and deterministic resolver.
- `crates/protocol-airpods/src/*`: Apple proximity-pairing parser.
- `crates/protocol-generic-battery/src/*`: aggregate and standard battery normalization.
- `crates/airbattery-core/src/*`: registry, freshness expiration, application events.
- `crates/bluetooth-linux/src/*`: BlueZ adapter and observation mapping.
- `crates/diagnostics/src/*`: sanitized backend report.
- `crates/airbattery-cli/src/main.rs`: safe Linux diagnostic command.

### Task 1: Workspace bootstrap

**Files:**
- Create: `Cargo.toml`
- Create: `rust-toolchain.toml`
- Create: `crates/*/Cargo.toml`
- Create: `crates/*/src/lib.rs`

**Interfaces:**
- Produces: compilable workspace crate boundaries used by every later task.

- [ ] **Step 1: Confirm Rust tooling is unavailable or below the required version**

Run: `rustc --version && cargo --version`
Expected: either command-not-found or Rust >= 1.85.

- [ ] **Step 2: Install a supported Rust toolchain using Debian packages or rustup**

Prefer Debian 13 packages when version >= 1.85. Install `rustfmt` and `clippy` components.

- [ ] **Step 3: Create workspace manifests and empty libraries**

Use resolver `2`, edition `2024`, and shared versions for `serde`, `thiserror`, `time`, `tokio`, `async-trait`, `tracing`, `futures`, `clap`, `bluer`, and `zbus`.

- [ ] **Step 4: Verify the empty workspace**

Run: `cargo check --workspace`
Expected: PASS with no warnings.

- [ ] **Step 5: Commit**

```bash
git add Cargo.toml rust-toolchain.toml crates
git commit -m "build: bootstrap Rust workspace"
```

### Task 2: Shared battery domain

**Files:**
- Create: `crates/shared-models/src/battery.rs`
- Create: `crates/shared-models/src/device.rs`
- Modify: `crates/shared-models/src/lib.rs`
- Test: unit tests colocated in `battery.rs` and `device.rs`

**Interfaces:**
- Produces: `BatteryPercentage::new(u8)`, `BatteryComponent`, `BluetoothAudioDevice`, `ComponentType`, `ChargingState`, `DataSource`, `DataConfidence`, `ConnectionState`, `Capability`.

- [ ] **Step 1: Write failing tests for percentage validation and unknown-state serialization**

Tests must prove `100` is valid, `101` is rejected, and `None` serializes as unavailable rather than `0`.

- [ ] **Step 2: Run tests and observe failure**

Run: `cargo test -p shared-models`
Expected: compile failure because domain types do not exist.

- [ ] **Step 3: Implement minimal strongly typed domain models**

`BatteryPercentage` is a private-field newtype with checked constructor. `BatteryComponent.percentage` is `Option<BatteryPercentage>`. Every timestamp uses `OffsetDateTime` with serde support.

- [ ] **Step 4: Run tests and clippy**

Run: `cargo test -p shared-models && cargo clippy -p shared-models -- -D warnings`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/shared-models
git commit -m "feat: add battery domain model"
```

### Task 3: Provider reports and deterministic resolver

**Files:**
- Create: `crates/device-protocols/src/provider.rs`
- Create: `crates/device-protocols/src/resolver.rs`
- Modify: `crates/device-protocols/src/lib.rs`
- Test: unit tests in `resolver.rs`

**Interfaces:**
- Consumes: `BatteryComponent`, `ComponentType`, `DataConfidence`, `DataSource`.
- Produces: `ProviderReport`, `RawObservation`, `DeviceProvider` trait, `resolve_reports(&[ProviderReport], OffsetDateTime) -> Vec<BatteryComponent>`.

- [ ] **Step 1: Write failing tests**

Cover: fresher report wins at equal confidence; higher confidence wins when both are fresh; stale report cannot overwrite fresh data; resolver merges left from one provider and case from another; ties use stable provider priority and provider id.

- [ ] **Step 2: Run and observe expected failure**

Run: `cargo test -p device-protocols`
Expected: missing resolver/types.

- [ ] **Step 3: Implement resolver ordering**

Order candidates by: valid percentage, non-stale status, confidence rank, source priority, `updated_at`, provider id. Resolve independently per `ComponentType`.

- [ ] **Step 4: Run tests and clippy**

Run: `cargo test -p device-protocols && cargo clippy -p device-protocols -- -D warnings`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/device-protocols
git commit -m "feat: resolve battery provider reports"
```

### Task 4: AirPods proximity-pairing parser

**Files:**
- Create: `crates/protocol-airpods/src/parser.rs`
- Create: `crates/protocol-airpods/src/model.rs`
- Modify: `crates/protocol-airpods/src/lib.rs`
- Create: `crates/protocol-airpods/tests/fixtures.rs`
- Create: `crates/protocol-airpods/NOTICE.md`

**Interfaces:**
- Produces: `parse_proximity_pairing(&[u8]) -> Result<Option<AirPodsAdvertisement>, AirPodsParseError>` and `decode_battery_nibble(u8) -> Option<BatteryPercentage>`.

- [ ] **Step 1: Write failing parser tests**

Tests cover wrong message type, truncated 27-byte payload, battery nibble `0..=10`, unknown nibble `11..=15`, left/right orientation flip, charging bits, AirPods Pro model nibble `0xE`, and Pro 2 nibble `0x4`.

- [ ] **Step 2: Run and observe expected failure**

Run: `cargo test -p protocol-airpods`
Expected: missing parser API.

- [ ] **Step 3: Implement strict parser**

Require company payload message type `0x07` and at least 27 bytes. Use documented nibble offsets: model `7`, orientation `10`, right `12`, left `13`, charge `14`, case `15` in the 54-character hex representation. Do not emit charging state for an absent component.

- [ ] **Step 4: Run tests and clippy**

Run: `cargo test -p protocol-airpods && cargo clippy -p protocol-airpods -- -D warnings`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/protocol-airpods
git commit -m "feat: parse AirPods battery advertisements"
```

### Task 5: Generic battery provider

**Files:**
- Create: `crates/protocol-generic-battery/src/lib.rs`
- Test: unit tests in the same file.

**Interfaces:**
- Produces: `normalize_platform_battery(device_id, percentage, timestamp) -> Result<ProviderReport, GenericBatteryError>` and `normalize_standard_battery(...)`.

- [ ] **Step 1: Write failing tests**

Prove values >100 are rejected, platform aggregate maps to `ComponentType::Aggregate`, and BLE Battery Service maps to `ComponentType::Headset` with no invented charging state.

- [ ] **Step 2: Run and observe expected failure**

Run: `cargo test -p protocol-generic-battery`
Expected: missing APIs.

- [ ] **Step 3: Implement minimal normalizers**

Assign explicit source and confidence. Charging state is always `Unknown` unless input includes reliable charging evidence.

- [ ] **Step 4: Verify**

Run: `cargo test -p protocol-generic-battery && cargo clippy -p protocol-generic-battery -- -D warnings`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/protocol-generic-battery
git commit -m "feat: normalize generic battery sources"
```

### Task 6: Registry and freshness engine

**Files:**
- Create: `crates/airbattery-core/src/registry.rs`
- Create: `crates/airbattery-core/src/freshness.rs`
- Modify: `crates/airbattery-core/src/lib.rs`
- Test: unit tests in both modules.

**Interfaces:**
- Consumes: `ProviderReport`, `resolve_reports`.
- Produces: `DeviceRegistry::apply_report`, `DeviceRegistry::snapshot`, `FreshnessPolicy`, `expire_components`.

- [ ] **Step 1: Write failing tests**

Cover: case expires before earbuds; disconnected device retains last value but marks stale; new fresh report clears stale; independent multi-device state; duplicate report does not emit a changed snapshot.

- [ ] **Step 2: Run and observe expected failure**

Run: `cargo test -p airbattery-core`
Expected: missing registry.

- [ ] **Step 3: Implement registry**

Store bounded report history per device/provider. Re-resolve on updates and expiry ticks. Equality ignores internal history but includes user-visible freshness state.

- [ ] **Step 4: Verify**

Run: `cargo test -p airbattery-core && cargo clippy -p airbattery-core -- -D warnings`
Expected: PASS.

- [ ] **Step 5: Commit**

```bash
git add crates/airbattery-core
git commit -m "feat: track device freshness and state"
```

### Task 7: Linux observation adapter and diagnostic CLI

**Files:**
- Create: `crates/bluetooth-linux/src/backend.rs`
- Create: `crates/bluetooth-linux/src/mapping.rs`
- Modify: `crates/bluetooth-linux/src/lib.rs`
- Create: `crates/diagnostics/src/lib.rs`
- Create: `crates/airbattery-cli/src/main.rs`
- Test: mapping tests with synthetic BlueZ property snapshots.

**Interfaces:**
- Produces: `LinuxBluetoothBackend::new`, `scan_window(Duration)`, `known_devices`, and CLI subcommands `status`, `scan`, `parse-airpods`.

- [ ] **Step 1: Write failing mapping and diagnostic tests**

Prove Apple manufacturer id `76` becomes an AirPods raw observation; BlueZ percentage maps to a generic platform observation; missing adapter returns a typed backend state; diagnostics hash addresses.

- [ ] **Step 2: Run and observe expected failure**

Run: `cargo test -p bluetooth-linux -p diagnostics`
Expected: missing modules.

- [ ] **Step 3: Implement event-driven BlueZ adapter**

Use `bluer::Session`, default adapter lookup, adapter events, device property reads, and a bounded discovery guard dropped after the requested window. Never power on the adapter automatically.

- [ ] **Step 4: Implement CLI**

`parse-airpods --hex` works without Bluetooth. `status` and `scan` report typed errors and never require root.

- [ ] **Step 5: Verify**

Run: `cargo test -p bluetooth-linux -p diagnostics -p airbattery-cli && cargo clippy -p bluetooth-linux -p diagnostics -p airbattery-cli -- -D warnings`
Expected: compile and tests pass; live status may report adapter unavailable in container.

- [ ] **Step 6: Commit**

```bash
git add crates/bluetooth-linux crates/diagnostics crates/airbattery-cli
git commit -m "feat: add Linux Bluetooth diagnostics"
```

### Task 8: Documentation and phase verification

**Files:**
- Modify: `docs/PROJECT_STATUS.md`
- Create: `docs/ARCHITECTURE.md`
- Create: `docs/AIRPODS_PROTOCOL.md`
- Create: `docs/BLUETOOTH_BACKENDS.md`
- Create: `docs/HARDWARE_TESTS.md`
- Create: `docs/TESTING.md`

**Interfaces:**
- Produces: evidence-backed phase checkpoint and exact user hardware command.

- [ ] **Step 1: Run full verification**

```bash
cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace --release
rg -n "TODO|FIXME|MOCK|PLACEHOLDER|skip|ignored" . --glob '!target/**' --glob '!.git/**'
```

- [ ] **Step 2: Record exact outputs and limitations**

Update `PROJECT_STATUS.md` with command exit status, test count, release binary paths, and unverified hardware/platform items.

- [ ] **Step 3: Document one-command hardware validation**

Document `cargo run -p airbattery-cli -- scan --seconds 20 --json` and the exact AirPods lid-open action. Keep raw identifiers sanitized by default.

- [ ] **Step 4: Commit**

```bash
git add docs
git commit -m "docs: record foundation verification"
```
