# Universal AirBattery Runtime Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build a multi-source Bluetooth battery runtime that resolves real per-component values when devices expose them, presents model-aware or category-aware artwork, updates automatically, and surfaces the same state in the desktop UI, GNOME top bar, and Windows system tray.

**Architecture:** Platform backends collect immutable observations and device metadata. Protocol providers normalize observations into independent battery components. The resolver selects the freshest and most reliable value per component. A presentation profile derived from manufacturer, model, BlueZ metadata, and name heuristics drives universal artwork without inventing battery values. Desktop, GNOME, and Windows tray read the same normalized snapshot.

**Tech Stack:** Rust 1.97.1, BlueR/bluer 0.17, Windows Runtime APIs through windows 0.62.2, Tauri 2.11, Vue 3.5, TypeScript 6, GNOME Shell JavaScript.

## Global Constraints

- Never copy an aggregate percentage into left, right, or case components.
- Never invent a missing percentage; render unavailable values as `—`.
- AirPods advertisement values remain unauthenticated protocol evidence, not proof of device identity.
- Automatic refresh must be bounded and serialized; no overlapping scans.
- The UI must support left/right/case, left/right, and single aggregate layouts.
- Exact-model artwork is used only when detection confidence is high; otherwise use category fallback artwork.
- GNOME and Windows tray must consume the same normalized device snapshot as the main window.
- Existing settings, diagnostics, privacy-safe identifiers, and stale-value semantics remain compatible.

---

### Task 1: Universal device metadata and presentation profile

**Files:**
- Modify: `crates/shared-models/src/device.rs`
- Modify: `crates/airbattery-service/src/lib.rs`
- Create: `crates/airbattery-service/src/classification.rs`
- Test: `crates/airbattery-service/tests/classification.rs`
- Modify: `apps/desktop/src/domain/types.ts`

**Interfaces:**
- Produces: `DeviceFamily` variants for earbuds, headset, speaker, mouse, keyboard, game controller, stylus, generic BLE, and unknown.
- Produces: `DeviceVisual { key: String, confidence: VisualConfidence }` on `BluetoothAudioDevice`.
- Consumes: descriptor name, manufacturer, model, BlueZ icon, class, appearance, UUIDs.

- [ ] Write failing classification tests for AirPods Pro, Galaxy Buds, Soundcore earbuds, Sony headset, JBL speaker, Logitech mouse, Keychron keyboard, controller, stylus, and unknown device.
- [ ] Run classification tests and confirm missing APIs fail.
- [ ] Implement the minimal classifier and model-aware visual keys.
- [ ] Run classification tests and existing service tests.
- [ ] Commit `feat: classify universal bluetooth devices`.

### Task 2: Rich Linux observations and reliable BLE discovery

**Files:**
- Modify: `crates/bluetooth-linux/src/mapping.rs`
- Modify: `crates/bluetooth-linux/src/backend.rs`
- Test: `crates/bluetooth-linux/tests/mapping.rs`
- Modify: `apps/desktop/src-tauri/src/platform/linux.rs`
- Test: `apps/desktop/src-tauri/tests/state_contract.rs`

**Interfaces:**
- Produces: `BluezDeviceSnapshot` with icon, class, appearance, UUIDs, service data, RSSI, paired state, and manufacturer data.
- Produces: `RawObservation::ServiceData` and standard/platform battery observations.
- Uses: `DiscoveryFilter { duplicate_data: true, transport: Auto, ..Default::default() }` and `discover_devices_with_changes()`.

- [ ] Write failing mapping tests proving service data and metadata survive the backend boundary.
- [ ] Run mapping tests and confirm failure.
- [ ] Add rich snapshot fields and raw service-data observations.
- [ ] Configure duplicate advertisement delivery before bounded discovery and collect every property-change event, not only first discovery.
- [ ] Increase normal bounded scan to 12 seconds and add a 30-second diagnostic scan mode without overlapping scans.
- [ ] Run mapping tests and Linux source verification.
- [ ] Commit `feat: collect rich BlueZ advertisement data`.

### Task 3: Multi-source component resolver and diagnostics provenance

**Files:**
- Modify: `crates/shared-models/src/battery.rs`
- Modify: `crates/device-protocols/src/provider.rs`
- Modify: `crates/device-protocols/src/resolver.rs`
- Test: `crates/device-protocols/tests/resolver.rs`
- Modify: `crates/airbattery-core/src/registry.rs`
- Test: `crates/airbattery-core/tests/registry.rs`
- Modify: `apps/desktop/src-tauri/src/commands.rs`

**Interfaces:**
- Produces: additional sources `ServiceData`, `HidBattery`, `VendorProtocol`, and source provenance in diagnostics.
- Resolver rule: known percentage, freshness, confidence, source priority, timestamp, provider priority, provider id.
- Aggregate and split components coexist; split values never replace aggregate identity and aggregate never fills split components.

- [ ] Write failing resolver tests for mixed aggregate/split data, stale case retention, and fresher provider selection.
- [ ] Run resolver tests and confirm failure.
- [ ] Extend source types and raw observations.
- [ ] Implement source-aware component resolution and provider-level diagnostics counters.
- [ ] Run resolver, registry, processor, and diagnostics tests.
- [ ] Commit `feat: resolve battery data per component`.

### Task 4: AirPods provider hardening and model coverage

**Files:**
- Modify: `crates/protocol-airpods/src/model.rs`
- Modify: `crates/protocol-airpods/src/parser.rs`
- Modify: `crates/protocol-airpods/src/provider.rs`
- Test: `crates/protocol-airpods/tests/parser.rs`
- Modify: `docs/AIRPODS_PROTOCOL.md`

**Interfaces:**
- Produces: complete left/right/case values and charging state from valid Apple proximity-pairing payloads.
- Produces: model labels for current known AirPods and Beats identifiers when evidence is reliable.
- Keeps: strict length validation and unavailable nibble handling.

- [ ] Add failing fixtures for AirPods Pro first generation orientation, missing case, charging combinations, malformed payloads, and extended payload lengths.
- [ ] Run parser tests and confirm the new cases fail.
- [ ] Correct parser offsets and orientation rules only where fixtures prove required behavior.
- [ ] Add model aliases without changing battery semantics.
- [ ] Run parser/provider tests.
- [ ] Commit `fix: harden AirPods battery parsing`.

### Task 5: Generic standard, HID, and vendor-provider framework

**Files:**
- Modify: `crates/protocol-generic-battery/src/lib.rs`
- Create: `crates/device-protocols/src/component_hint.rs`
- Modify: `crates/device-protocols/src/lib.rs`
- Test: `crates/protocol-generic-battery/tests/normalization.rs`
- Test: `crates/device-protocols/tests/component_hint.rs`

**Interfaces:**
- Produces: component hints from Battery Service presentation descriptors, service UUID context, HID metadata, and vendor service-data rules.
- Fallback: when component identity is unknown, emit one `Aggregate` or `Headset` value only.

- [ ] Write failing tests for single standard battery, explicit left/right/case hints, HID aggregate battery, and unknown vendor service data.
- [ ] Run tests and confirm failure.
- [ ] Implement minimal component-hint normalization.
- [ ] Register generic providers without vendor-specific false positives.
- [ ] Run provider and core tests.
- [ ] Commit `feat: normalize generic bluetooth batteries`.

### Task 6: Automatic runtime refresh and tray synchronization

**Files:**
- Modify: `apps/desktop/src-tauri/src/lib.rs`
- Modify: `apps/desktop/src-tauri/src/state.rs`
- Modify: `apps/desktop/src-tauri/src/commands.rs`
- Create: `apps/desktop/src-tauri/src/refresh.rs`
- Modify: `apps/desktop/src-tauri/src/tray.rs`
- Test: `apps/desktop/src-tauri/tests/state_contract.rs`

**Interfaces:**
- Produces: one serialized refresh scheduler with initial refresh, periodic 15-second known-device reads, periodic bounded scans, resume refresh, and immediate UI/D-Bus/tray publication.
- Produces: tray title/tooltip summary from the preferred or connected device.

- [ ] Write failing pure tests for refresh cadence decisions and tray summary selection.
- [ ] Run tests and confirm failure.
- [ ] Implement scheduler decisions as pure functions.
- [ ] Wire scheduler into Tauri startup and state publication.
- [ ] Update tray tooltip after every snapshot; on Windows use a generated percentage icon when supported and retain static fallback elsewhere.
- [ ] Run Tauri state tests and source verification.
- [ ] Commit `feat: refresh battery state automatically`.

### Task 7: Universal desktop artwork and battery layouts

**Files:**
- Create: `apps/desktop/src/components/DeviceArtwork.vue`
- Create: `apps/desktop/src/components/DeviceBatteryPanel.vue`
- Modify: `apps/desktop/src/views/BatteryView.vue`
- Modify: `apps/desktop/src/components/BatteryComponentCard.vue`
- Modify: `apps/desktop/src/domain/presentation.ts`
- Modify: `apps/desktop/src/domain/types.ts`
- Modify: `apps/desktop/src/styles/base.css`
- Test: `apps/desktop/tests/presentation.test.ts`
- Create: `apps/desktop/tests/device-visual.test.ts`

**Interfaces:**
- Produces: `buildBatteryLayout(device)` returning `split-three`, `split-two`, or `single`.
- Produces: artwork keys for exact AirPods models and category fallbacks.
- Single-earbud aggregate layout shows two earbuds and one percentage.

- [ ] Write failing tests for full AirPods, earbuds without case, aggregate earbuds, headset, speaker, mouse, keyboard, stylus, and unknown fallback layouts.
- [ ] Run Node tests and confirm failure.
- [ ] Implement pure presentation helpers.
- [ ] Add accessible inline SVG artwork and the new layout component.
- [ ] Remove the manual refresh button from the normal battery surface; retain refresh in diagnostics.
- [ ] Fix sidebar overflow by separating scrollable navigation from the anchored About item.
- [ ] Run Node tests and frontend typecheck/build when dependencies are available.
- [ ] Commit `feat: add universal device battery presentation`.

### Task 8: Compact widget surface

**Files:**
- Modify: `apps/desktop/src/App.vue`
- Modify: `apps/desktop/src/views/BatteryView.vue`
- Modify: `apps/desktop/src/styles/base.css`
- Test: `apps/desktop/tests/window-mode.test.ts`
- Test: `apps/desktop/tests/presentation.test.ts`

**Interfaces:**
- Widget shows only device name, connection indicator, artwork, and battery values.
- Widget has no device switcher, large metadata row, freshness explanation, or refresh button.

- [ ] Write failing layout-model tests for widget content decisions.
- [ ] Run Node tests and confirm failure.
- [ ] Add a `compact` prop and widget-specific rendering.
- [ ] Add responsive CSS for split and aggregate layouts.
- [ ] Run Node tests.
- [ ] Commit `fix: simplify the desktop battery widget`.

### Task 9: GNOME top-bar extension

**Files:**
- Modify: `apps/gnome-extension/snapshot.js`
- Modify: `apps/gnome-extension/extension.js`
- Modify: `apps/gnome-extension/stylesheet.css`
- Modify: `apps/gnome-extension/prefs.js`
- Modify: `apps/gnome-extension/schemas/io.github.airbattery.gnome.gschema.xml`
- Test: `apps/gnome-extension/tests/snapshot.test.mjs`
- Modify: `docs/GNOME_EXTENSION.md`

**Interfaces:**
- Top bar supports `L 82 R 79 C 64` and compact minimum-earbud percentage modes.
- Popup shows device name, connection state, independent components, charging state, and open-app action.
- Extension updates from `SnapshotChanged`; manual refresh is moved to a diagnostics submenu.

- [ ] Write failing snapshot-format tests for split and aggregate devices.
- [ ] Run GNOME tests and confirm failure.
- [ ] Implement configurable panel summaries and richer popup rows.
- [ ] Remove the prominent manual refresh action and keep a fallback service reconnect path.
- [ ] Validate schema and source checks.
- [ ] Commit `feat: show component batteries in GNOME`.

### Task 10: Windows advertisement collection and system tray

**Files:**
- Modify: `crates/bluetooth-windows/Cargo.toml`
- Modify: `crates/bluetooth-windows/src/lib.rs`
- Modify: `apps/desktop/src-tauri/src/platform/windows.rs`
- Modify: `apps/desktop/src-tauri/src/tray.rs`
- Create: `apps/desktop/src-tauri/src/tray_icon.rs`
- Test: `apps/desktop/src-tauri/tests/state_contract.rs`
- Modify: `docs/WINDOWS_BACKEND.md`

**Interfaces:**
- Windows collection merges paired GATT devices with advertisements received by `BluetoothLEAdvertisementWatcher` during the bounded window.
- Manufacturer data enters the same provider pipeline as Linux.
- Dynamic tray icon displays selected percentage; tooltip contains left/right/case summary.

- [ ] Write failing pure tests for Windows advertisement-to-observation mapping and dynamic tray icon pixel generation.
- [ ] Run target-independent tests and confirm failure.
- [ ] Add Windows Advertisement namespace features and watcher collection.
- [ ] Map company data and service data into raw observations.
- [ ] Generate accessible dynamic tray icons without network assets.
- [ ] Run source verification; cross-compile or native Windows build when a Windows toolchain is available.
- [ ] Commit `feat: add Windows BLE advertisements and battery tray`.

### Task 11: Release validation and hardware diagnostic workflow

**Files:**
- Modify: `apps/desktop/src/views/DiagnosticsView.vue`
- Modify: `apps/desktop/src-tauri/src/commands.rs`
- Modify: `docs/HARDWARE_TESTS.md`
- Modify: `RUN_UBUNTU_VALIDATION.sh`
- Create: `scripts/validate-universal-runtime.sh`

**Interfaces:**
- Diagnostics export lists each component, source, confidence, freshness, selected visual key, and sanitized observation counts.
- Hardware checklist covers AirPods Pro 2020 case open/closed, each ear removed independently, charging, reconnect, sleep/resume, GNOME, and Windows tray.

- [ ] Write failing frontend tests for diagnostics presentation helpers.
- [ ] Run tests and confirm failure.
- [ ] Extend sanitized diagnostics without exporting MAC addresses or raw manufacturer payloads.
- [ ] Add complete hardware checklist and validation script.
- [ ] Run all available Rust, Node, GNOME, packaging, and source checks.
- [ ] Commit `test: add universal battery validation workflow`.
