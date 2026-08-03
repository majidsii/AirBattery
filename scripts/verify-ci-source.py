#!/usr/bin/env python3
"""Static validation for AirBattery CI, release automation, and dependency policy."""

from __future__ import annotations

import json
import re
import sys
import tomllib
from pathlib import Path
from typing import Any

import yaml

ROOT = Path(__file__).resolve().parents[1]
CI_PATH = ROOT / ".github/workflows/ci.yml"
RELEASE_PATH = ROOT / ".github/workflows/release.yml"
DEPENDABOT_PATH = ROOT / ".github/dependabot.yml"
DENY_PATH = ROOT / "deny.toml"
DESKTOP_PACKAGE_PATH = ROOT / "apps/desktop/package.json"
WINDOWS_BLUETOOTH_PATH = ROOT / "crates/bluetooth-windows/src/lib.rs"
errors: list[str] = []
checks = 0


def check(condition: bool, message: str) -> None:
    global checks
    checks += 1
    if not condition:
        errors.append(message)


def require(path: Path) -> str:
    check(path.is_file(), f"missing automation source: {path.relative_to(ROOT)}")
    if not path.is_file():
        return ""
    text = path.read_text(encoding="utf-8")
    check(bool(text.strip()), f"empty automation source: {path.relative_to(ROOT)}")
    return text


def load_yaml(text: str, label: str) -> dict[str, Any]:
    if not text:
        return {}
    try:
        value = yaml.load(text, Loader=yaml.BaseLoader)
    except yaml.YAMLError as error:
        errors.append(f"invalid {label} YAML: {error}")
        return {}
    check(isinstance(value, dict), f"{label} root must be a mapping")
    return value if isinstance(value, dict) else {}


def steps_for(job: dict[str, Any]) -> list[dict[str, Any]]:
    steps = job.get("steps", [])
    return [step for step in steps if isinstance(step, dict)] if isinstance(steps, list) else []


def uses_for(job: dict[str, Any]) -> set[str]:
    return {str(step.get("uses")) for step in steps_for(job) if step.get("uses")}


def list_value(value: Any) -> list[str]:
    if isinstance(value, list):
        return [str(item) for item in value]
    if isinstance(value, str):
        return [value]
    return []


def validate_checkout(jobs: dict[str, Any], label: str) -> None:
    for name, job in jobs.items():
        if not isinstance(job, dict):
            errors.append(f"{label} job {name} must be a mapping")
            continue
        checkout_steps = [step for step in steps_for(job) if step.get("uses") == "actions/checkout@v7"]
        check(bool(checkout_steps), f"{label} job {name} must use actions/checkout@v7")
        for step in checkout_steps:
            with_values = step.get("with", {}) if isinstance(step.get("with", {}), dict) else {}
            check(
                with_values.get("persist-credentials") == "false",
                f"{label} job {name} checkout must disable persisted credentials",
            )


ci_text = require(CI_PATH)
release_text = require(RELEASE_PATH)
dependabot_text = require(DEPENDABOT_PATH)
deny_text = require(DENY_PATH)
desktop_package_text = require(DESKTOP_PACKAGE_PATH)
windows_bluetooth_text = require(WINDOWS_BLUETOOTH_PATH)
ci = load_yaml(ci_text, "CI")
release = load_yaml(release_text, "release")

# Pull-request and main-branch CI.
events = ci.get("on", {}) if isinstance(ci.get("on", {}), dict) else {}
check("pull_request" in events, "CI must run for pull requests")
push = events.get("push", {}) if isinstance(events.get("push", {}), dict) else {}
branches = list_value(push.get("branches", []))
check("main" in branches, "CI push trigger must include main")
check("workflow_dispatch" in events, "CI must support manual dispatch")
check("pull_request_target" not in events, "CI must not use pull_request_target")

permissions = ci.get("permissions", {}) if isinstance(ci.get("permissions", {}), dict) else {}
check(permissions.get("contents") == "read", "CI top-level contents permission must be read")
check("write" not in str(permissions).lower(), "CI top-level permissions must not grant write access")

concurrency = ci.get("concurrency", {}) if isinstance(ci.get("concurrency", {}), dict) else {}
check(bool(concurrency.get("group")), "CI concurrency group missing")
check(concurrency.get("cancel-in-progress") == "true", "CI must cancel superseded runs")

jobs = ci.get("jobs", {}) if isinstance(ci.get("jobs", {}), dict) else {}
required_jobs = {"source", "frontend", "rust-linux", "rust-windows", "dependency-audit"}
check(required_jobs.issubset(jobs), f"CI jobs missing: {sorted(required_jobs - set(jobs))}")
validate_checkout(jobs, "CI")

check(jobs.get("source", {}).get("runs-on") == "ubuntu-24.04", "source job must use ubuntu-24.04")
check(jobs.get("frontend", {}).get("runs-on") == "ubuntu-24.04", "frontend job must use ubuntu-24.04")
check(jobs.get("rust-linux", {}).get("runs-on") == "ubuntu-24.04", "rust-linux job must use ubuntu-24.04")
check(jobs.get("rust-windows", {}).get("runs-on") == "windows-2022", "rust-windows job must use windows-2022")
check(jobs.get("dependency-audit", {}).get("runs-on") == "ubuntu-24.04", "dependency-audit job must use ubuntu-24.04")

for name in ("source", "frontend", "rust-linux", "dependency-audit"):
    check("actions/setup-node@v7" in uses_for(jobs.get(name, {})), f"{name} job must use actions/setup-node@v7")
for name in ("rust-linux", "rust-windows", "dependency-audit"):
    check(
        "dtolnay/rust-toolchain@stable" in uses_for(jobs.get(name, {})),
        f"{name} job must install Rust with dtolnay/rust-toolchain",
    )

ci_uses = [
    str(step.get("uses"))
    for job in jobs.values()
    if isinstance(job, dict)
    for step in steps_for(job)
    if step.get("uses")
]
check("actions/cache@v5" in ci_uses, "CI must use actions/cache@v5")
check(not any(re.search(r"@(main|master|latest)$", use) for use in ci_uses), "CI action references must not use mutable branches")
check(not any(use.startswith("actions/upload-artifact") for use in ci_uses), "CI validation must not upload release artifacts")

for workflow_name, workflow_text in (("CI", ci_text), ("release", release_text)):
    check("PyYAML==6.0.3" in workflow_text, f"{workflow_name} workflow must install pinned PyYAML")
    check("Pillow==12.3.0" in workflow_text, f"{workflow_name} workflow must install pinned Pillow")

for token in (
    "cargo fmt --all -- --check",
    "cargo clippy --workspace --all-targets -- -D warnings",
    "cargo test --workspace",
    "cargo build --workspace --release",
    "npm run typecheck",
    "npm run test",
    "npm run build",
    "python3 scripts/verify-ci-source.py",
    "python3 scripts/verify-release-source.py",
    "python3 scripts/verify-version-sync.py",
    "python3 -m unittest tests/release_artifacts_test.py",
):
    check(token in ci_text, f"CI command missing: {token}")
check(
    "cargo check -p airbattery-desktop --target x86_64-pc-windows-msvc" in ci_text,
    "Windows desktop target check missing",
)
check("cargo deny check" in ci_text, "cargo-deny audit missing")
check('rust-version: "1.97.1"' in ci_text, "cargo-deny Rust version pin missing")
check("npm audit" in ci_text, "npm audit missing")
check("npm run test:ui" not in ci_text, "CI must not run Node test suites through Vitest")
if desktop_package_text:
    desktop_package = json.loads(desktop_package_text)
    scripts = desktop_package.get("scripts", {})
    dev_dependencies = desktop_package.get("devDependencies", {})
    check("test:ui" not in scripts, "desktop package must not expose an empty Vitest suite")
    check("vitest" not in dev_dependencies, "desktop package must not depend on unused Vitest")
check(
    re.search(
        r"TypedEventHandler::<\s*BluetoothLEAdvertisementWatcher,\s*"
        r"BluetoothLEAdvertisementReceivedEventArgs,\s*>::new",
        windows_bluetooth_text,
    )
    is not None,
    "Windows advertisement handler must bind sender and event argument types explicitly",
)
check(
    "args.as_ref()" in windows_bluetooth_text,
    "Windows advertisement callback must unwrap windows_core::Ref with as_ref()",
)
check(
    "&Option<BluetoothLEAdvertisementReceivedEventArgs>" not in windows_bluetooth_text,
    "Windows advertisement callback must not use the pre-0.62 Option reference signature",
)
check("contents: write" not in ci_text, "CI workflow must not request contents write")
check("pull-requests: write" not in ci_text, "CI workflow must not request pull-request write")

# Tag-only draft release workflow.
release_events = release.get("on", {}) if isinstance(release.get("on", {}), dict) else {}
release_push = release_events.get("push", {}) if isinstance(release_events.get("push", {}), dict) else {}
release_tags = list_value(release_push.get("tags", []))
check("v*" in release_tags, "release workflow must run on v* tags")
check(not list_value(release_push.get("branches", [])), "release workflow must not run on branch pushes")
check("pull_request" not in release_events, "release workflow must not run on pull requests")
check("workflow_dispatch" not in release_events, "release workflow must remain tag-only")

release_permissions = release.get("permissions", {}) if isinstance(release.get("permissions", {}), dict) else {}
check(release_permissions.get("contents") == "read", "release top-level contents permission must be read")
check("write" not in str(release_permissions).lower(), "release top-level permissions must not grant write access")

release_jobs = release.get("jobs", {}) if isinstance(release.get("jobs", {}), dict) else {}
required_release_jobs = {"verify", "build-linux", "build-windows", "publish"}
check(
    required_release_jobs.issubset(release_jobs),
    f"release jobs missing: {sorted(required_release_jobs - set(release_jobs))}",
)
validate_checkout(release_jobs, "release")

check(release_jobs.get("verify", {}).get("runs-on") == "ubuntu-24.04", "release verify job must use ubuntu-24.04")
check(release_jobs.get("build-linux", {}).get("runs-on") == "ubuntu-24.04", "Linux release job must use ubuntu-24.04")
check(release_jobs.get("build-windows", {}).get("runs-on") == "windows-2022", "Windows release job must use windows-2022")
check(release_jobs.get("publish", {}).get("runs-on") == "ubuntu-24.04", "publish job must use ubuntu-24.04")

for name in ("build-linux", "build-windows"):
    uses = uses_for(release_jobs.get(name, {}))
    check("actions/setup-node@v7" in uses, f"{name} must use actions/setup-node@v7")
    check("dtolnay/rust-toolchain@stable" in uses, f"{name} must install the Rust toolchain")
    check("tauri-apps/tauri-action@v1" in uses, f"{name} must build with tauri-apps/tauri-action@v1")
    check("actions/upload-artifact@v7" in uses, f"{name} must upload native workflow artifacts")

publish_uses = uses_for(release_jobs.get("publish", {}))
check("actions/download-artifact@v8" in publish_uses, "publish job must download native artifacts")
publish_permissions = release_jobs.get("publish", {}).get("permissions", {})
check(
    isinstance(publish_permissions, dict) and publish_permissions.get("contents") == "write",
    "only the publish job must receive contents write permission",
)
for name in ("verify", "build-linux", "build-windows"):
    check(
        "write" not in str(release_jobs.get(name, {}).get("permissions", {})).lower(),
        f"release job {name} must not receive write permission",
    )

publish_needs = set(list_value(release_jobs.get("publish", {}).get("needs", [])))
check(
    {"verify", "build-linux", "build-windows"}.issubset(publish_needs),
    "publish job must wait for verification and both native builds",
)

release_uses = [
    str(step.get("uses"))
    for job in release_jobs.values()
    if isinstance(job, dict)
    for step in steps_for(job)
    if step.get("uses")
]
check(not any(re.search(r"@(main|master|latest)$", use) for use in release_uses), "release action references must not use mutable branches")

for token in (
    "python3 scripts/verify-version-sync.py",
    "--expected-tag",
    "--bundles deb,appimage",
    "--bundles nsis",
    "target/release/bundle",
    "python3 scripts/verify-release-artifacts.py",
    "python3 scripts/checksum-artifacts.py",
    "SHA256SUMS",
    "gh release create",
    "--draft",
    "gh release upload",
    "if-no-files-found: error",
):
    check(token in release_text, f"release command or policy missing: {token}")
check("secrets." not in release_text, "release workflow must not depend on repository secrets")
check("pull_request_target" not in release_text, "release workflow must not use pull_request_target")

# Dependabot and cargo-deny policy.
dependabot = load_yaml(dependabot_text, "Dependabot")
check(dependabot.get("version") == "2", "Dependabot schema version must be 2")
updates = dependabot.get("updates", []) if isinstance(dependabot.get("updates", []), list) else []
ecosystems = {item.get("package-ecosystem") for item in updates if isinstance(item, dict)}
check({"cargo", "npm", "github-actions"}.issubset(ecosystems), "Dependabot must cover Cargo, npm, and GitHub Actions")
for update in updates:
    if not isinstance(update, dict):
        continue
    schedule = update.get("schedule", {}) if isinstance(update.get("schedule", {}), dict) else {}
    check(schedule.get("interval") == "weekly", f"Dependabot {update.get('package-ecosystem')} schedule must be weekly")
    check(bool(update.get("groups")), f"Dependabot {update.get('package-ecosystem')} update group missing")

if deny_text:
    try:
        deny = tomllib.loads(deny_text)
    except tomllib.TOMLDecodeError as error:
        errors.append(f"invalid deny.toml: {error}")
        deny = {}
    check(deny.get("advisories", {}).get("yanked") == "deny", "cargo-deny must deny yanked crates")
    check(deny.get("bans", {}).get("wildcards") == "deny", "cargo-deny must deny wildcard dependencies")
    check(deny.get("sources", {}).get("unknown-git") == "deny", "cargo-deny must deny unknown git sources")
    allowed = deny.get("licenses", {}).get("allow", [])
    check("MIT" in allowed and "Apache-2.0" in allowed, "cargo-deny license allow-list lacks core licenses")

# Local path dependencies must also carry the exact workspace version. This
# keeps cargo-deny wildcard enforcement enabled while allowing unpublished
# workspace crates to resolve from local paths.
root_cargo = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
workspace_version = root_cargo.get("workspace", {}).get("package", {}).get("version")
expected_path_version = f"={workspace_version}" if workspace_version else ""
manifest_paths = [
    *sorted((ROOT / "crates").glob("*/Cargo.toml")),
    ROOT / "apps" / "desktop" / "src-tauri" / "Cargo.toml",
]
for manifest_path in manifest_paths:
    if not manifest_path.is_file():
        continue
    try:
        manifest = tomllib.loads(manifest_path.read_text(encoding="utf-8"))
    except tomllib.TOMLDecodeError as error:
        errors.append(f"invalid Cargo manifest {manifest_path.relative_to(ROOT)}: {error}")
        continue

    dependency_tables: list[tuple[str, dict[str, Any]]] = []
    for section_name in ("dependencies", "dev-dependencies", "build-dependencies"):
        section = manifest.get(section_name, {})
        if isinstance(section, dict):
            dependency_tables.append((section_name, section))
    targets = manifest.get("target", {})
    if isinstance(targets, dict):
        for target_name, target_config in targets.items():
            if not isinstance(target_config, dict):
                continue
            for section_name in ("dependencies", "dev-dependencies", "build-dependencies"):
                section = target_config.get(section_name, {})
                if isinstance(section, dict):
                    dependency_tables.append((f"target.{target_name}.{section_name}", section))

    for section_name, dependencies in dependency_tables:
        for dependency_name, dependency_spec in dependencies.items():
            if isinstance(dependency_spec, dict) and "path" in dependency_spec:
                check(
                    dependency_spec.get("version") == expected_path_version,
                    (
                        f"{manifest_path.relative_to(ROOT)} {section_name} dependency "
                        f"{dependency_name} must use version {expected_path_version!r} with its path"
                    ),
                )

marker = re.compile(r"\b(TODO|FIXME|MOCK|PLACEHOLDER)\b", re.IGNORECASE)
for path, text in (
    (CI_PATH, ci_text),
    (RELEASE_PATH, release_text),
    (DEPENDABOT_PATH, dependabot_text),
    (DENY_PATH, deny_text),
):
    check(marker.search(text) is None, f"unfinished marker in {path.relative_to(ROOT)}")

if errors:
    print(f"CI/release source checks: {len(errors)} failed / {checks} evaluated", file=sys.stderr)
    for error in errors:
        print(f"- {error}", file=sys.stderr)
    raise SystemExit(1)

print(f"CI/release source checks: {checks} passed")
