# AirBattery Glass Phase 1 Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Build the adjustable AirBattery Glass material foundation and approved desktop Overview without changing Bluetooth behavior.

**Architecture:** Pure glass preference logic generates normalized state and CSS variables. Pinia applies and persists it. A dedicated CSS layer styles main/widget roles and accessibility fallbacks; current battery components remain the source of truthful UI data.

**Tech Stack:** Vue 3.5, Pinia 4, TypeScript, authored CSS, Node test runner, Python source verifier.

## Global Constraints

- No new runtime or development dependency.
- Glass intensity is bounded to 0–100.
- Named presets are Off, Subtle, Balanced, Deep, and Crystal.
- Main window, Desktop widget, and Connection popup are independently adjustable.
- No Bluetooth, battery normalization, device selection, or artwork-policy changes.
- Reduce-transparency and no-backdrop-filter fallbacks are mandatory.

---

### Task 1: Glass preference domain

**Files:**
- Create: `apps/desktop/src/domain/glass.ts`
- Create: `apps/desktop/tests/glass.test.ts`

- [x] Write failing normalization, preset, and CSS-variable tests.
- [x] Confirm tests fail because the module is absent.
- [x] Implement normalized presets, per-surface intensity, safe persistence, and CSS variables.
- [x] Run domain tests and confirm all pass.

### Task 2: Runtime settings integration

**Files:**
- Modify: `apps/desktop/src/stores/settings.ts`
- Modify: `apps/desktop/src/App.vue`
- Modify: `apps/desktop/src/main.ts`

- [x] Add source-contract expectations for window role and glass data attributes.
- [x] Load and save local glass preferences without changing native settings schema.
- [x] Apply preset, accessibility, and CSS-variable state to the root document.
- [x] Bind the app shell to the current Tauri window role.

### Task 3: Appearance controls

**Files:**
- Modify: `apps/desktop/src/views/SettingsView.vue`
- Create: `apps/desktop/src/styles/glass.css`

- [x] Add five preset controls and a 0–100 master slider.
- [x] Add independent main, widget, and popup sliders.
- [x] Add adaptive-glass and reduce-transparency controls.
- [x] Add a live, data-free material preview.

### Task 4: Approved Overview hierarchy

**Files:**
- Modify: `apps/desktop/src/views/BatteryView.vue`
- Modify: `apps/desktop/src/styles/glass.css`

- [x] Add the AirBattery Glass Overview hero and active-device count.
- [x] Preserve rendering of every automatically selected active device.
- [x] Preserve all existing `DeviceBatteryPanel` truthfulness behavior.
- [x] Add responsive main and compact-widget styles.

### Task 5: Verification contracts

**Files:**
- Create: `apps/desktop/tests/glass-ui-contract.test.ts`
- Modify: `scripts/verify-desktop-source.py`

- [x] Verify presets, independent controls, window roles, material layers, and fallbacks.
- [x] Run all 53 dependency-free desktop tests.
- [x] Run the desktop source verifier with 255 checks.
- [ ] Run Vue typecheck and production build on the development host.
- [ ] Run the complete GitHub Actions matrix after push.
