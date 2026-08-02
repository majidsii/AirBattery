# AirBattery Desktop and GNOME Shell Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a typed Vue desktop experience, a narrowly permissioned Tauri shell, and a GNOME top-bar companion that consume normalized device snapshots without duplicating Bluetooth logic.

**Architecture:** Pure TypeScript presentation functions define all unavailable/stale/charging labels and are executable without Vue. Vue components consume a Pinia store backed by a Tauri bridge. Rust commands expose snapshots and settings; GNOME consumes a versioned session D-Bus contract owned by the backend.

**Tech Stack:** Vue 3.5, Pinia 4, Vite 8, TypeScript, Tauri 2, CSS custom properties, Node built-in tests, GJS/ES modules for GNOME Shell.

## Global Constraints

- No mock battery values in production code.
- Unavailable, unsupported, stale, disconnected, and zero are distinct states.
- Every interactive control is keyboard reachable and labelled.
- Reduced motion and reduced transparency are first-class fallbacks.
- GNOME never parses Bluetooth packets and does not poll continuously.
- Tauri capabilities expose only required commands/plugins.
- Source authoring may continue without package downloads, but no build claim is allowed.

---

### Task 1: Presentation domain and tests

**Files:**
- Create: `apps/desktop/src/domain/types.ts`
- Create: `apps/desktop/src/domain/presentation.ts`
- Test: `apps/desktop/tests/presentation.test.ts`

**Interfaces:**
- Produces: `presentBatteryComponent`, `selectPreferredDevice`, `summarizeDevice`, and strict frontend domain types mirroring Rust serde names.

- [ ] Write failing Node tests for zero, unavailable, stale, disconnected, preferred-device fallback, and charging labels.
- [ ] Run with `node --experimental-strip-types --test apps/desktop/tests/presentation.test.ts` and observe missing-module failure.
- [ ] Implement the pure presentation functions.
- [ ] Rerun the test and require exit 0.
- [ ] Commit.

### Task 2: Vue application and liquid-glass design system

**Files:**
- Create: `apps/desktop/package.json`, Vite/TypeScript configuration, and `index.html`.
- Create: `apps/desktop/src/App.vue`, `main.ts`, stores, API bridge, components, views, and styles.

**Interfaces:**
- Consumes: normalized frontend types and Tauri events.
- Produces: battery popup, devices, settings, diagnostics, and about surfaces with compact/expanded modes.

- [ ] Add package metadata pinned to researched current stable versions.
- [ ] Implement a typed backend bridge with browser-safe unavailable fallback, not fake data.
- [ ] Implement Pinia device/settings state.
- [ ] Implement accessible navigation and reusable battery/status components.
- [ ] Implement all required screens and responsive widget layout.
- [ ] Add static scans for forbidden hardcoded battery fixtures in production source.
- [ ] Commit.

### Task 3: Tauri service shell

**Files:**
- Create: `apps/desktop/src-tauri/Cargo.toml`, `tauri.conf.json`, capabilities, build script, Rust state/commands/tray entrypoints.
- Modify: workspace members and dependencies.

**Interfaces:**
- Produces commands `get_devices`, `get_backend_status`, `refresh_devices`, `get_settings`, `save_settings`, and window/tray lifecycle actions; emits `airbattery://devices-changed`.

- [ ] Define settings and runtime-state tests first.
- [ ] Implement typed commands and safe shared state.
- [ ] Register store, autostart, notification, single-instance, and window-state plugins with narrow capabilities.
- [ ] Implement tray actions and close-to-background behavior.
- [ ] Record that compile/runtime verification is blocked until Cargo and system libraries are available.
- [ ] Commit.

### Task 4: GNOME session D-Bus contract and extension

**Files:**
- Create: `apps/gnome-extension/extension.js`, `prefs.js`, metadata, schemas, stylesheet, and install scripts.
- Create: `crates/airbattery-dbus` contract/service crate.
- Create: `docs/GNOME_EXTENSION.md`.

**Interfaces:**
- D-Bus name `io.github.airbattery.Service`, path `/io/github/airbattery/Service`, interface `io.github.airbattery.Service1`.
- Methods: `GetSnapshot`, `Refresh`, `OpenSettings`, `ShowMainWindow`.
- Signal: `SnapshotChanged(json)`.

- [ ] Define contract serialization tests first.
- [ ] Implement backend-owned service interface.
- [ ] Implement GNOME indicator and menu using GJS ESM imports and D-Bus proxy signals.
- [ ] Implement preferences and schema for compact percentage and preferred device.
- [ ] Add install/development scripts without system-wide writes.
- [ ] Commit.

### Task 5: Verification checkpoint

**Files:**
- Modify: `docs/PROJECT_STATUS.md`, `docs/TESTING.md`, `docs/DECISIONS.md`.

- [ ] Run Node pure-domain tests.
- [ ] Run package/TOML/JSON/XML syntax checks available without dependencies.
- [ ] Attempt npm, Cargo, and GNOME validation commands and record exact exits.
- [ ] Scan production source for placeholders, skipped tests, raw identifiers, and hardcoded battery values.
- [ ] Commit the evidence-backed checkpoint.
