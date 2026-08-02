# Fast Refresh and Menu Layering Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [x]`) syntax for tracking.

**Goal:** Publish AirPods battery changes approximately every three seconds and keep navigation labels above page content.

**Architecture:** Retain serialized bounded discovery, but shorten the normal discovery window to two seconds and schedule it every three seconds. Fix the UI with explicit sibling stacking layers rather than increasing only the tooltip's local z-index.

**Tech Stack:** Rust/Tokio/Tauri, Vue 3, CSS, Node test runner.

## Global Constraints

- Normal discovery window: exactly 2 seconds.
- Automatic refresh interval: exactly 3 seconds.
- Diagnostic discovery window remains 30 seconds.
- Automatic scans must remain serialized and skip missed ticks.
- Navigation labels must render above `.app-content` without changing responsive navigation behavior.

---

### Task 1: Refresh cadence contract

**Files:**
- Create: `apps/desktop/tests/runtime-contract.test.ts`
- Modify: `apps/desktop/src-tauri/src/refresh.rs`
- Modify: `apps/desktop/src-tauri/src/platform/linux.rs`
- Modify: `apps/desktop/src-tauri/src/platform/windows.rs`

**Interfaces:**
- Consumes: existing `commands::refresh_and_emit` and `RefreshMode::BoundedDiscovery`.
- Produces: three-second scheduler and two-second normal discovery constants.

- [x] **Step 1: Write a failing source-contract test asserting 3-second refresh and 2-second discovery windows.**
- [x] **Step 2: Run `cd apps/desktop && npm test -- tests/runtime-contract.test.ts` and verify failure.**
- [x] **Step 3: Change only the three normal runtime constants and user-facing Linux detail string.**
- [x] **Step 4: Run the desktop tests and verify pass.**

### Task 2: Navigation stacking contract

**Files:**
- Modify: `apps/desktop/tests/runtime-contract.test.ts`
- Modify: `apps/desktop/src/styles/base.css`

**Interfaces:**
- Consumes: `.app-nav`, `.app-content`, and `.nav-button__label` selectors.
- Produces: explicit isolated stacking layers with navigation above content.

- [x] **Step 1: Extend the failing test to require `position`, `isolation`, and explicit z-index ordering.**
- [x] **Step 2: Run the focused test and verify the CSS contract fails.**
- [x] **Step 3: Add minimal stacking declarations without changing layout dimensions.**
- [x] **Step 4: Run tests and frontend build.**

### Task 3: Documentation and package

**Files:**
- Modify: `AIRPODS_PRO_2020_FIX_FA.md`
- Modify: `CHANGELOG.md`
- Create: updated release ZIP and SHA-256 manifest in `/mnt/data`.

**Interfaces:**
- Produces: installable source package and concise verification instructions.

- [x] **Step 1: Document the three-second target and menu fix.**
- [x] **Step 2: Run Node, Python, source-contract, and available build checks.**
- [x] **Step 3: Package the clean source tree and generate SHA-256.**
