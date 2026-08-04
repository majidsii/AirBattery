# Live Status Logo Colors Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Color the approved battery-and-Bluetooth mark itself for live battery states in GNOME and Windows, with no separate status dot or ring.

**Architecture:** Keep the application icon static. Derive live status from fresh battery components, apply semantic CSS classes to the GNOME symbolic icon, and render the Windows tray glyph directly in the semantic color. A critical active device overrides the selected device so urgent battery state remains visible.

**Tech Stack:** GNOME Shell JavaScript, St CSS, Rust/Tauri tray rendering, Node test runner, dependency-free Python source verifiers.

## Global Constraints

- App icon remains a static white approved mark on Liquid Glass.
- Live status colors apply only to GNOME Top Bar and Windows System Tray.
- Thresholds are critical 0–10%, low 11–20%, connected 21–100%.
- Stale values never produce low or critical status.
- No separate status dot or ring is rendered.
- Critical state from any active device overrides the selected device.

---

### Task 1: GNOME status tone model

**Files:**
- Modify: `apps/gnome-extension/snapshot.js`
- Test: `apps/gnome-extension/tests/snapshot.test.mjs`

- [x] Add failing tests for connected, low, critical, unavailable, disconnected, stale values, and critical override.
- [x] Add `panelStatusTone(devices, selectedDevice)` using fresh component percentages.
- [x] Run `npm --prefix apps/gnome-extension test` and verify all tests pass.

### Task 2: GNOME live icon color

**Files:**
- Modify: `apps/gnome-extension/extension.js`
- Modify: `apps/gnome-extension/stylesheet.css`
- Modify: `scripts/verify-gnome-source.py`

- [x] Apply one semantic status class to the symbolic icon.
- [x] Remove old status classes before every update.
- [x] Define connected, low, critical, unavailable, and disconnected colors.
- [x] Run GNOME tests and source verification.

### Task 3: Windows tray glyph color

**Files:**
- Modify: `apps/desktop/src-tauri/src/tray_icon.rs`
- Modify: `apps/desktop/src-tauri/src/tray.rs`
- Modify: `scripts/verify-tauri-source.py`

- [x] Remove the status-ring renderer.
- [x] Pass the semantic status color into the battery and Bluetooth mark renderer.
- [x] Add unavailable and disconnected glyph variants.
- [x] Preserve critical-active-device override behavior.
- [x] Run source verification; run Rust tests on the Ubuntu host and GitHub Actions.

### Task 4: Final approved application icon

**Files:**
- Modify: `apps/desktop/src-tauri/icons/icon.svg`
- Modify: `apps/desktop/src-tauri/icons/icon.png`
- Modify: `apps/desktop/src-tauri/icons/32x32.png`
- Modify: `apps/desktop/src-tauri/icons/128x128.png`
- Modify: `apps/desktop/src-tauri/icons/128x128@2x.png`
- Modify: `apps/desktop/src-tauri/icons/icon.ico`
- Modify: `packaging/linux/io.github.airbattery.airbattery.svg`

- [x] Use the approved slim vertical battery, short cap, and centered white Bluetooth mark.
- [x] Keep Liquid Glass on the app tile only.
- [x] Regenerate PNG and ICO assets from the same SVG source.
- [x] Run packaging and desktop source verification.
