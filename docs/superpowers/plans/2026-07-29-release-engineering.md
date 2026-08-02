# Release Engineering Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Add reviewable project governance, Linux desktop metadata, deterministic release checksums, and separated CI/release workflows without claiming artifacts that have not been built.

**Architecture:** Pull-request CI performs dependency, source, Rust, frontend, and cross-platform compile checks with read-only permissions. Tag-only release jobs build Linux and Windows bundles on native runners, package the GNOME extension on Linux, generate SHA-256 manifests, and upload artifacts to a draft GitHub Release. Package metadata remains under `packaging/`, while Tauri consumes generated icon resources from `apps/desktop/src-tauri/icons/`.

**Tech Stack:** GitHub Actions, Tauri 2, Rust 1.97, Node.js 22, AppStream XML, freedesktop desktop entry, SVG/PNG/ICO resources, Bash, Python 3.

## Global Constraints

- Never mark a package or installer as generated unless the file exists and its checksum is produced.
- CI jobs use least-privilege permissions and no secrets on pull requests.
- Release publication is draft-only until a maintainer explicitly publishes it.
- Linux and Windows packages are built only on their native runners.
- Generated identifiers and metadata use `io.github.airbattery.airbattery` consistently.
- No signing credential is embedded or assumed.

---

### Task 1: Public project governance

**Files:**
- Create: `CHANGELOG.md`
- Create: `CONTRIBUTING.md`
- Create: `SECURITY.md`
- Create: `CODE_OF_CONDUCT.md`
- Create: `docs/RELEASE.md`
- Create: `docs/TROUBLESHOOTING.md`

**Interfaces:**
- Consumes: current feature and validation status in `docs/PROJECT_STATUS.md`.
- Produces: contributor, vulnerability, conduct, support, and release contracts.

- [x] Write alpha-accurate governance documents with no public-release claim.
- [x] Add a release checklist that requires artifact existence, checksums, native smoke tests, and Definition-of-Done review.
- [x] Run the documentation/marker verifier.
- [x] Commit the governance batch.

### Task 2: Linux application identity and icons

**Files:**
- Create: `packaging/linux/io.github.airbattery.airbattery.desktop`
- Create: `packaging/linux/io.github.airbattery.airbattery.metainfo.xml`
- Create: `packaging/linux/io.github.airbattery.airbattery.svg`
- Create: `apps/desktop/src-tauri/icons/icon.svg`
- Create: `scripts/generate-icons.py`
- Modify: `apps/desktop/src-tauri/tauri.conf.json`

**Interfaces:**
- Consumes: Tauri identifier `io.github.airbattery.airbattery` and original AirBattery visual tokens.
- Produces: freedesktop/AppStream identity plus deterministic Tauri icon inputs.

- [x] Add a failing packaging-source verifier for identity, desktop entry, AppStream, and icon requirements.
- [x] Create the original vector icon and metadata.
- [x] Generate available PNG/ICO resources only when local image libraries support them; record missing formats honestly.
- [x] Run XML/desktop/icon validation and make the verifier pass.
- [x] Commit application identity resources.

### Task 3: Checksums and artifact validation

**Files:**
- Create: `scripts/checksum-artifacts.py`
- Create: `scripts/verify-release-artifacts.py`
- Create: `tests/release_artifacts_test.py`

**Interfaces:**
- Consumes: a directory containing concrete release files.
- Produces: sorted `SHA256SUMS` and a machine-readable failure when required artifacts are absent.

- [x] Write failing tests for deterministic checksums, empty directories, and missing required bundles.
- [x] Implement checksum generation without following symlinks.
- [x] Implement platform-aware artifact validation.
- [x] Run the Python tests and static checks.
- [x] Commit release artifact tooling.

### Task 4: Pull-request and main-branch CI

**Files:**
- Create: `.github/workflows/ci.yml`
- Create: `.github/dependabot.yml`
- Create: `scripts/verify-ci-source.py`
- Create: `deny.toml`

**Interfaces:**
- Consumes: repository verification scripts and package manifests.
- Produces: read-only Linux quality jobs, Windows compile jobs, dependency audit, and dependency update configuration.

- [ ] Add a failing CI verifier for triggers, permissions, action versions, required jobs, and forbidden release permissions.
- [ ] Add source/frontend/Rust Linux jobs and a Windows compile/test job.
- [ ] Add cargo-deny policy and Dependabot groups.
- [ ] Parse YAML/TOML and make the verifier pass.
- [ ] Commit CI validation.

### Task 5: Tag-only release workflow

**Files:**
- Create: `.github/workflows/release.yml`
- Modify: `docs/RELEASE.md`
- Modify: `scripts/verify-ci-source.py`

**Interfaces:**
- Consumes: semantic tag `v*`, Tauri bundle configuration, GNOME package script, release artifact tools.
- Produces: native Linux/Windows artifacts uploaded to one draft release with checksums.

- [ ] Extend the failing verifier to require tag-only execution, draft release behavior, native runners, checksums, and restricted permissions.
- [ ] Add Linux `.deb`/AppImage/GNOME jobs and Windows NSIS job.
- [ ] Add a final release-manifest job that downloads artifacts, validates them, creates `SHA256SUMS`, and attaches them to a draft release.
- [ ] Parse and statically verify the workflow.
- [ ] Commit release automation.

### Task 6: Consolidated checkpoint

**Files:**
- Modify: `README.md`
- Modify: `docs/PROJECT_STATUS.md`
- Modify: `docs/TESTING.md`
- Modify: `docs/INSTALLATION.md`
- Modify: `docs/BUILDING.md`

**Interfaces:**
- Consumes: all preceding release-engineering output.
- Produces: accurate resume commands and explicit native-runtime blockers.

- [ ] Run every dependency-free test and source verifier.
- [ ] Run `git diff --check` and unfinished-marker scan.
- [ ] Record exact evidence and external blockers.
- [ ] Commit the release-engineering checkpoint.
