#!/usr/bin/env python3
"""Static validation for AirBattery GitHub Actions and dependency policy."""

from __future__ import annotations

import re
import sys
import tomllib
from pathlib import Path
from typing import Any

import yaml

ROOT = Path(__file__).resolve().parents[1]
CI_PATH = ROOT / ".github/workflows/ci.yml"
DEPENDABOT_PATH = ROOT / ".github/dependabot.yml"
DENY_PATH = ROOT / "deny.toml"
errors: list[str] = []
checks = 0


def check(condition: bool, message: str) -> None:
    global checks
    checks += 1
    if not condition:
        errors.append(message)


def require(path: Path) -> str:
    check(path.is_file(), f"missing CI source: {path.relative_to(ROOT)}")
    if not path.is_file():
        return ""
    text = path.read_text(encoding="utf-8")
    check(bool(text.strip()), f"empty CI source: {path.relative_to(ROOT)}")
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


ci_text = require(CI_PATH)
dependabot_text = require(DEPENDABOT_PATH)
deny_text = require(DENY_PATH)
ci = load_yaml(ci_text, "CI")

events = ci.get("on", {}) if isinstance(ci.get("on", {}), dict) else {}
check("pull_request" in events, "CI must run for pull requests")
push = events.get("push", {}) if isinstance(events.get("push", {}), dict) else {}
branches = push.get("branches", []) if isinstance(push.get("branches", []), list) else []
check("main" in branches, "CI push trigger must include main")
check("workflow_dispatch" in events, "CI must support manual dispatch")

permissions = ci.get("permissions", {}) if isinstance(ci.get("permissions", {}), dict) else {}
check(permissions.get("contents") == "read", "CI top-level contents permission must be read")
check("write" not in str(permissions).lower(), "CI top-level permissions must not grant write access")

concurrency = ci.get("concurrency", {}) if isinstance(ci.get("concurrency", {}), dict) else {}
check(bool(concurrency.get("group")), "CI concurrency group missing")
check(concurrency.get("cancel-in-progress") == "true", "CI must cancel superseded runs")

jobs = ci.get("jobs", {}) if isinstance(ci.get("jobs", {}), dict) else {}
required_jobs = {"source", "frontend", "rust-linux", "rust-windows", "dependency-audit"}
check(required_jobs.issubset(jobs), f"CI jobs missing: {sorted(required_jobs - set(jobs))}")

for name, job in jobs.items():
    if not isinstance(job, dict):
        errors.append(f"CI job {name} must be a mapping")
        continue
    job_permissions = job.get("permissions", {})
    check("contents: write" not in str(job_permissions).lower(), f"CI job {name} grants contents write")
    steps = steps_for(job)
    checkout_steps = [step for step in steps if step.get("uses") == "actions/checkout@v7"]
    check(bool(checkout_steps), f"CI job {name} must use actions/checkout@v7")
    for step in checkout_steps:
        with_values = step.get("with", {}) if isinstance(step.get("with", {}), dict) else {}
        check(with_values.get("persist-credentials") == "false", f"CI job {name} checkout must disable persisted credentials")

check(jobs.get("source", {}).get("runs-on") == "ubuntu-24.04", "source job must use ubuntu-24.04")
check(jobs.get("frontend", {}).get("runs-on") == "ubuntu-24.04", "frontend job must use ubuntu-24.04")
check(jobs.get("rust-linux", {}).get("runs-on") == "ubuntu-24.04", "rust-linux job must use ubuntu-24.04")
check(jobs.get("rust-windows", {}).get("runs-on") == "windows-2022", "rust-windows job must use windows-2022")

for name in ("frontend", "dependency-audit"):
    uses = {step.get("uses") for step in steps_for(jobs.get(name, {}))}
    check("actions/setup-node@v6" in uses, f"{name} job must use actions/setup-node@v6")

for name in ("rust-linux", "rust-windows", "dependency-audit"):
    uses = {step.get("uses") for step in steps_for(jobs.get(name, {}))}
    check("dtolnay/rust-toolchain@stable" in uses, f"{name} job must install Rust with dtolnay/rust-toolchain")

uses_all = [
    str(step.get("uses"))
    for job in jobs.values()
    if isinstance(job, dict)
    for step in steps_for(job)
    if step.get("uses")
]
check("actions/cache@v5" in uses_all, "CI must use actions/cache@v5")
check(not any(re.search(r"@(main|master|latest)$", use) for use in uses_all), "CI action references must not use mutable branches")
check(not any(use.startswith("actions/upload-artifact") for use in uses_all), "CI validation must not upload release artifacts")

for token in (
    "cargo fmt --all -- --check",
    "cargo clippy --workspace --all-targets -- -D warnings",
    "cargo test --workspace",
    "cargo build --workspace --release",
    "npm run typecheck",
    "npm run test",
    "npm run test:ui",
    "npm run build",
    "python3 -m unittest tests/release_artifacts_test.py",
):
    check(token in ci_text, f"CI command missing: {token}")
check("cargo check -p airbattery-desktop --target x86_64-pc-windows-msvc" in ci_text, "Windows desktop target check missing")
check("cargo deny check" in ci_text, "cargo-deny audit missing")
check("npm audit" in ci_text, "npm audit missing")
check("contents: write" not in ci_text, "CI workflow must not request contents write")
check("pull-requests: write" not in ci_text, "CI workflow must not request pull-request write")

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

marker = re.compile(r"\b(TODO|FIXME|MOCK|PLACEHOLDER)\b", re.IGNORECASE)
for path, text in ((CI_PATH, ci_text), (DEPENDABOT_PATH, dependabot_text), (DENY_PATH, deny_text)):
    check(marker.search(text) is None, f"unfinished marker in {path.relative_to(ROOT)}")

if errors:
    print(f"CI source checks: {len(errors)} failed / {checks} evaluated", file=sys.stderr)
    for error in errors:
        print(f"- {error}", file=sys.stderr)
    raise SystemExit(1)

print(f"CI source checks: {checks} passed")
