#!/usr/bin/env python3
"""Dependency-free checks for the AirBattery Tauri host source."""

from __future__ import annotations

import json
import re
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
TAURI = ROOT / "apps" / "desktop" / "src-tauri"
SRC = TAURI / "src"
errors: list[str] = []
checks = 0


def check(condition: bool, message: str) -> None:
    global checks
    checks += 1
    if not condition:
        errors.append(message)


required = [
    TAURI / "Cargo.toml",
    TAURI / "build.rs",
    TAURI / "tauri.conf.json",
    TAURI / "capabilities" / "default.json",
    SRC / "main.rs",
    SRC / "lib.rs",
    SRC / "commands.rs",
    SRC / "error.rs",
    SRC / "model.rs",
    SRC / "native_surface.rs",
    SRC / "state.rs",
    SRC / "tray.rs",
    SRC / "windowing.rs",
    SRC / "gnome.rs",
    SRC / "platform" / "mod.rs",
    SRC / "platform" / "linux.rs",
    SRC / "platform" / "windows.rs",
]
for path in required:
    check(path.is_file(), f"missing Tauri source: {path.relative_to(ROOT)}")

if (TAURI / "Cargo.toml").is_file():
    try:
        cargo = tomllib.loads((TAURI / "Cargo.toml").read_text(encoding="utf-8"))
        check(cargo.get("package", {}).get("name") == "airbattery-desktop", "unexpected Tauri crate name")
        check(cargo.get("package", {}).get("publish") is False, "desktop host must not be published as a crate")
        dependencies = cargo.get("dependencies", {})
        for dependency in [
            "airbattery-service",
            "airbattery-settings",
            "protocol-airpods",
            "protocol-generic-battery",
            "diagnostics",
            "tauri",
            "tauri-plugin-autostart",
            "tauri-plugin-single-instance",
            "tauri-plugin-window-state",
        ]:
            check(dependency in dependencies, f"missing Tauri dependency: {dependency}")
    except Exception as error:  # noqa: BLE001
        errors.append(f"invalid Cargo.toml: {error}")
        checks += 1

for json_path in [TAURI / "tauri.conf.json", TAURI / "capabilities" / "default.json"]:
    if json_path.is_file():
        try:
            data = json.loads(json_path.read_text(encoding="utf-8"))
            check(isinstance(data, dict), f"JSON root must be an object: {json_path.relative_to(ROOT)}")
        except Exception as error:  # noqa: BLE001
            errors.append(f"invalid JSON in {json_path.relative_to(ROOT)}: {error}")
            checks += 1

check("airbattery-dbus" in (TAURI / "Cargo.toml").read_text(encoding="utf-8"), "Linux D-Bus dependency missing")

if (TAURI / "capabilities" / "default.json").is_file():
    capability = json.loads((TAURI / "capabilities" / "default.json").read_text(encoding="utf-8"))
    permissions = capability.get("permissions", [])
    forbidden = {"core:default", "fs:default", "shell:allow-execute", "shell:allow-spawn"}
    check(not forbidden.intersection(permissions), "capability grants a broad or shell permission")

if (SRC / "lib.rs").is_file():
    lib = (SRC / "lib.rs").read_text(encoding="utf-8")
    single = lib.find(".plugin(tauri_plugin_single_instance")
    autostart = lib.find(".plugin(tauri_plugin_autostart")
    check(single >= 0, "single-instance plugin is not registered")
    check(autostart >= 0, "autostart plugin is not registered")
    check(single < autostart if min(single, autostart) >= 0 else False, "single-instance plugin must be registered first")
    for command in [
        "get_devices",
        "get_backend_status",
        "refresh_devices",
        "get_settings",
        "save_settings",
        "export_diagnostics",
        "open_widget",
        "open_settings",
    ]:
        check(command in lib, f"command is not registered: {command}")


if (SRC / "gnome.rs").is_file():
    gnome = (SRC / "gnome.rs").read_text(encoding="utf-8")
    check('#[cfg(not(target_os = "linux"))]\nuse tauri::AppHandle;' in gnome, "non-Linux AppHandle import is not cfg-gated")
    check('#[cfg(not(target_os = "linux"))]\nuse crate::error::CommandError;' in gnome, "non-Linux CommandError import is not cfg-gated")
    for required_token in [
        "GnomeIntegration",
        "start_service",
        "ServiceRequest::Refresh",
        "ServiceRequest::OpenSettings",
        "ServiceSnapshot::new",
        "publish",
        "reconcile",
    ]:
        check(required_token in gnome, f"GNOME bridge token missing: {required_token}")

if (SRC / "commands.rs").is_file():
    commands = (SRC / "commands.rs").read_text(encoding="utf-8")
    check("gnome::publish" in commands, "native refresh does not publish a GNOME snapshot")
    check("gnome::reconcile" in commands, "settings changes do not reconcile GNOME integration")

if (SRC / "lib.rs").is_file():
    integration_lib = (SRC / "lib.rs").read_text(encoding="utf-8")
    check("mod gnome;" in integration_lib, "GNOME integration module is not registered")
    check("mod native_surface;" in integration_lib, "native status-surface module is not registered")
    check("GnomeIntegration::default" in integration_lib, "GNOME integration state is not managed")

production_files = list(SRC.rglob("*.rs")) if SRC.is_dir() else []
marker = re.compile(r"\b(TODO|FIXME|MOCK|PLACEHOLDER)\b", re.IGNORECASE)
raw_address = re.compile(r"(?:[0-9A-Fa-f]{2}:){5}[0-9A-Fa-f]{2}")
for path in production_files:
    text = path.read_text(encoding="utf-8")
    check(marker.search(text) is None, f"unfinished marker in {path.relative_to(ROOT)}")
    check(raw_address.search(text) is None, f"raw Bluetooth address literal in {path.relative_to(ROOT)}")
    check("Command::new" not in text, f"unreviewed shell command execution in {path.relative_to(ROOT)}")
    check("std::process::Command" not in text, f"shell process API in {path.relative_to(ROOT)}")
    check(".unwrap()" not in text, f"unwrap in production source: {path.relative_to(ROOT)}")
    check(".expect(" not in text, f"expect in production source: {path.relative_to(ROOT)}")

if errors:
    print(f"Tauri source checks: {len(errors)} failed / {checks} evaluated", file=sys.stderr)
    for error in errors:
        print(f"- {error}", file=sys.stderr)
    raise SystemExit(1)

print(f"Tauri source checks: {checks} passed")
