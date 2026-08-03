# AirBattery Hybrid Native Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Stabilize exact AirPods battery state, enforce native-surface ownership, and replace misleading product imagery with an exact-or-generic artwork pipeline.

**Architecture:** A long-lived native monitor feeds provider observations into the existing normalized registry. UI surfaces consume cached snapshots through Tauri events or D-Bus. Artwork selection is deterministic and refuses approximate product photos.

**Tech Stack:** Rust 2024, Tokio, BlueZ/bluer, Tauri 2, zbus/D-Bus, Vue 3, TypeScript, GNOME Shell JavaScript, Node test runner, Python verification scripts.

## Global Constraints

- Preserve the existing `main` history; work only on `feature/hybrid-native`.
- Write a failing regression test before production behavior changes.
- Never treat 127 or 255 as a valid percentage.
- Never erase a valid component with a temporarily unavailable observation.
- GNOME integration and AppIndicator must not be visible simultaneously.
- Final product artwork must be exact, licensed, and model-specific; otherwise use a neutral fallback.
- Do not add runtime 3D rendering or a large new UI dependency.

---

### Task 1: Persistent Apple accessory monitor lifecycle

**Files:**
- Modify: `crates/bluetooth-linux/src/accessory.rs`
- Modify: `crates/bluetooth-linux/src/lib.rs`
- Modify: `apps/desktop/src-tauri/src/platform/linux.rs`
- Test: `crates/bluetooth-linux/src/accessory.rs`
- Test: `apps/desktop/src-tauri/tests/state_contract.rs`

**Interfaces:**
- Produces: `sync_apple_accessory_monitors(addresses: &BTreeSet<String>)`
- Produces: `latest_apple_accessory_battery(address: &str) -> Option<AppleAccessoryPacket>`

- [ ] Add regression tests proving monitor keys are normalized and disconnected devices are pruned.
- [ ] Run the focused Rust tests and confirm the new tests fail for missing lifecycle support.
- [ ] Implement monitor synchronization, cancellation, and last-packet retention.
- [ ] Update Linux collection to synchronize monitors before reading packets.
- [ ] Run focused tests and then the Rust workspace test suite.
- [ ] Commit as `fix: stabilize Apple accessory monitor lifecycle`.

### Task 2: Snapshot retention and battery regression contracts

**Files:**
- Modify: `crates/airbattery-core/src/registry.rs`
- Modify: `crates/airbattery-core/tests/registry.rs`
- Modify: `apps/desktop/src-tauri/tests/state_contract.rs`

**Interfaces:**
- Consumes: provider reports containing optional percentages and original observation timestamps.
- Produces: snapshots that retain previous valid values until normal freshness expiry.

- [ ] Add tests for unavailable exact case values, older platform aggregate values, and delayed connection-state updates.
- [ ] Verify the tests fail for any regression.
- [ ] Implement only the minimum registry/resolver changes needed by those contracts.
- [ ] Run focused and workspace tests.
- [ ] Commit as `fix: preserve verified component state across refreshes`.

### Task 3: GNOME visibility and native surface ownership

**Files:**
- Modify: `apps/gnome-extension/snapshot.js`
- Modify: `apps/gnome-extension/extension.js`
- Modify: `apps/gnome-extension/tests/snapshot.test.mjs`
- Modify: `apps/desktop/src-tauri/src/native_surface.rs`
- Modify: `apps/desktop/src-tauri/tests/state_contract.rs`

**Interfaces:**
- Produces: `snapshotHasActiveDevice(snapshot)` for indicator visibility.
- Consumes: normalized D-Bus snapshots only.

- [ ] Add failing GNOME tests for hidden empty/disconnected snapshots and visible verified active snapshots.
- [ ] Implement visibility from normalized activity rather than service ownership alone.
- [ ] Add Rust contracts for one native surface per platform/session.
- [ ] Run GNOME, Node, and Rust tests.
- [ ] Commit as `fix: enforce one native status surface per session`.

### Task 4: Exact-or-generic artwork pipeline

**Files:**
- Modify: `apps/desktop/src/domain/artwork-catalog.ts`
- Modify: `apps/desktop/src/domain/pre-rendered-artwork.ts`
- Modify: `apps/desktop/src/components/DeviceArtwork.vue`
- Modify: `apps/desktop/tests/artwork-catalog.test.ts`
- Modify: `apps/desktop/tests/artwork-svg-contract.test.ts`
- Create: `docs/ARTWORK_PIPELINE.md`

**Interfaces:**
- Produces: deterministic per-mode assets with `kind: 'exact' | 'fallback'`.
- Consumes: exact asset manifest entries only for exact model keys.

- [ ] Add tests that reject photo reuse, mirrored right earbuds, and exact assets without all required metadata.
- [ ] Remove current low-quality exact product photographs from default selection.
- [ ] Keep neutral SVG fallback assets and document offline 3D requirements.
- [ ] Run desktop tests and production build/typecheck.
- [ ] Commit as `refactor: enforce exact-or-generic product artwork`.

### Task 5: Diagnostics, verification, and delivery

**Files:**
- Modify: `scripts/verify-desktop-source.py`
- Modify: `scripts/verify-tauri-source.py`
- Modify: `scripts/verify-gnome-source.py`
- Modify: `docs/TESTING.md`
- Modify: `README.md`

**Interfaces:**
- Produces: source checks covering monitor lifecycle, battery retention, native surfaces, and artwork policy.

- [ ] Add verification checks for every new contract.
- [ ] Run Python checks, Node tests, GNOME tests, format, Clippy, Rust tests, typecheck, and release build.
- [ ] Record any hardware-only verification that cannot run outside Ubuntu with real AirPods.
- [ ] Commit as `test: verify hybrid native runtime contracts`.
- [ ] Create a Git bundle and numbered patches from `main..feature/hybrid-native`.
