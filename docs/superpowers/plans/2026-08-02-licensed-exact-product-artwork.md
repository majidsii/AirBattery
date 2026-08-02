# Licensed Exact Product Artwork Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add a license-audited exact product-photo override system while restoring SVG as the truthful fallback.

**Architecture:** A curated Wikimedia source manifest feeds a Python build-time importer that generates WebP assets, attribution metadata, and a TypeScript import map. The application resolves model-level exact assets first and falls back per mode to the existing SVG family catalog.

**Tech Stack:** Vue 3, TypeScript, Vite, Node test runner, Python 3, Pillow, Wikimedia Commons API.

## Global Constraints

- No AI-generated product artwork.
- No unlicensed manufacturer imagery.
- No network request at application runtime.
- Missing exact modes fall back to existing SVG assets.
- Every processed photograph includes machine-readable attribution and license metadata.

---

### Task 1: Restore truthful fallback and define exact-artwork interfaces

**Files:**
- Modify: `apps/desktop/src/domain/artwork-catalog.ts`
- Create: `apps/desktop/src/domain/exact-artwork.generated.ts`
- Modify: `apps/desktop/src/components/DeviceArtwork.vue`
- Modify: `apps/desktop/src/styles/base.css`
- Test: `apps/desktop/tests/artwork-catalog.test.ts`
- Test: `apps/desktop/tests/artwork-svg-contract.test.ts`

- [ ] Write failing tests requiring SVG fallback and optional exact per-key overrides.
- [ ] Run desktop tests and verify the new tests fail.
- [ ] Restore SVG imports and merge generated exact overrides per mode.
- [ ] Update rendering classes for exact photos without mirroring unapproved photographs.
- [ ] Run desktop tests and verify they pass.

### Task 2: Add curated licensed-source manifest and validator

**Files:**
- Create: `assets/exact-artwork/sources.json`
- Create: `scripts/exact_artwork.py`
- Create: `scripts/fetch-exact-artwork.py`
- Create: `scripts/verify-exact-artwork.py`
- Test: `scripts/tests/test_exact_artwork.py`

- [ ] Write failing Python tests for manifest validation, license allowlist, filename safety, and generated TypeScript.
- [ ] Run tests and verify failure.
- [ ] Implement validation and deterministic generator helpers.
- [ ] Add curated Wikimedia Commons entries with attribution metadata.
- [ ] Run tests and verify success.

### Task 3: Integrate attribution and packaging checks

**Files:**
- Create: `apps/desktop/src/assets/device-artwork/exact/README.md`
- Create: `apps/desktop/src/assets/device-artwork/exact/ATTRIBUTION.generated.json`
- Create: `docs/EXACT_PRODUCT_ARTWORK.md`
- Modify: `scripts/verify-desktop-source.py`
- Modify: `RUN_UBUNTU_VALIDATION.sh`

- [ ] Write source-verification assertions for attribution and forbidden studio assets.
- [ ] Run verifier and confirm failure.
- [ ] Add generated placeholders and documentation.
- [ ] Add exact-artwork verification to the Ubuntu validation path.
- [ ] Run all static and unit tests.

### Task 4: Produce a distributable patch and source archive

**Files:**
- Create: distribution patch, ZIP, SHA-256 manifest.

- [ ] Verify desktop tests.
- [ ] Verify Python tests.
- [ ] Verify source validators.
- [ ] Verify ZIP contents and checksums.
