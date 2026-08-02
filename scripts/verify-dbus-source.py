#!/usr/bin/env python3
"""Dependency-free contract checks for AirBattery's versioned session D-Bus API."""

from __future__ import annotations

import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CRATE = ROOT / "crates/airbattery-dbus"


def require(path: Path) -> str:
    if not path.is_file():
        raise AssertionError(f"missing required file: {path.relative_to(ROOT)}")
    text = path.read_text(encoding="utf-8")
    if not text.strip():
        raise AssertionError(f"required file is empty: {path.relative_to(ROOT)}")
    return text


def expect(text: str, pattern: str, label: str) -> None:
    if re.search(pattern, text, flags=re.MULTILINE | re.DOTALL) is None:
        raise AssertionError(f"missing {label}: /{pattern}/")


def main() -> int:
    checks = 0
    workspace = require(ROOT / "Cargo.toml")
    for pattern, label in (
        (r'"crates/airbattery-dbus"', "D-Bus workspace member"),
        (r'zbus\s*=\s*\{\s*version\s*=\s*"=5\.18\.0"', "pinned zbus 5.18.0"),
        (r'time\s*=\s*\{\s*version\s*=\s*"=0\.3\.54"', "pinned time 0.3.54"),
        (r'tokio\s*=.*"sync"', "Tokio sync feature"),
    ):
        expect(workspace, pattern, label)
        checks += 1

    manifest = require(CRATE / "Cargo.toml")
    for dependency in ("serde.workspace = true", "serde_json.workspace = true", "time.workspace = true", "tokio.workspace = true", "zbus.workspace = true"):
        if dependency not in manifest:
            raise AssertionError(f"D-Bus manifest missing {dependency}")
        checks += 1

    source = require(CRATE / "src/lib.rs")
    constants = {
        "BUS_NAME": "io.github.airbattery.Service",
        "OBJECT_PATH": "/io/github/airbattery/Service",
        "INTERFACE_NAME": "io.github.airbattery.Service1",
    }
    for name, value in constants.items():
        expect(source, rf'pub const {name}: &str = "{re.escape(value)}";', name)
        checks += 1
    expect(source, r'pub const IPC_SCHEMA_VERSION: u16 = 1;', "IPC schema version")
    checks += 1
    for member in ("get_snapshot", "refresh", "open_settings", "show_main_window"):
        expect(source, rf'async fn {member}\b', f"{member} method")
        checks += 1
    for request in ("Refresh", "OpenSettings", "ShowMainWindow"):
        expect(source, rf'ServiceRequest::{request}', f"{request} request routing")
        checks += 1
    expect(source, r'#\[zbus\(signal\)\]\s*async fn snapshot_changed', "SnapshotChanged signal")
    checks += 1
    expect(source, r'SignalEmitter::new\(&self\.connection, OBJECT_PATH\)', "versioned signal emitter")
    checks += 1
    expect(source, r'Builder::session\(\)', "session bus builder")
    checks += 1
    expect(source, r'\.name\(BUS_NAME\)', "well-known bus name")
    checks += 1
    expect(source, r'\.serve_at\(OBJECT_PATH', "object path registration")
    checks += 1
    expect(source, r'const REQUEST_TIMEOUT: Duration = Duration::from_secs\(10\);', "bounded D-Bus request timeout")
    checks += 1
    expect(source, r'self\.requests\.try_send\(request\)', "non-blocking bounded request enqueue")
    checks += 1
    expect(source, r'receive_response_with_timeout\(receiver, REQUEST_TIMEOUT\)', "production request timeout")
    checks += 1
    expect(source, r'timeout\(wait, receiver\)', "bounded D-Bus response wait")
    checks += 1
    expect(source, r'pub preferred_device_id: Option<String>', "preferred device in snapshot")
    checks += 1
    expect(source, r'#\[serde\(default, with = "time::serde::rfc3339::option"\)\]\s*pub generated_at: Option<OffsetDateTime>', "typed RFC3339 snapshot timestamp")
    checks += 1

    unit_tests = source
    if "request_response_times_out" not in unit_tests:
        raise AssertionError("D-Bus unit test missing request_response_times_out")
    checks += 1

    tests = require(CRATE / "tests/snapshot.rs")
    for token in ("round_trips_versioned_snapshot_with_rfc3339_timestamps", "rejects_unsupported_snapshot_schema", "lastSeenAt", "preferredDeviceId"):
        if token not in tests:
            raise AssertionError(f"D-Bus test missing {token}")
        checks += 1

    battery = require(ROOT / "crates/shared-models/src/battery.rs")
    device = require(ROOT / "crates/shared-models/src/device.rs")
    expect(battery, r'#\[serde\(with = "time::serde::rfc3339"\)\]\s*pub updated_at', "component RFC3339 timestamp")
    checks += 1
    for field in ("last_seen_at", "last_updated_at"):
        expect(device, rf'#\[serde\(default, with = "time::serde::rfc3339::option"\)\]\s*pub {field}', f"{field} RFC3339 timestamp")
        checks += 1

    bridge = require(ROOT / "apps/desktop/src-tauri/src/gnome.rs")
    for token in ("start_service", "ServiceRequest::Refresh", "ServiceRequest::OpenSettings", "ServiceRequest::ShowMainWindow", "ServiceSnapshot::new", "settings.preferred_device_id"):
        if token not in bridge:
            raise AssertionError(f"Tauri GNOME bridge missing {token}")
        checks += 1

    commands = require(ROOT / "apps/desktop/src-tauri/src/commands.rs")
    for token in ("BACKEND_STATUS_EVENT", "result.status_changed", "gnome::publish", "gnome::reconcile"):
        if token not in commands:
            raise AssertionError(f"Tauri command boundary missing {token}")
        checks += 1

    combined = "\n".join((source, tests, bridge))
    if re.search(r"\b(?:TODO|FIXME|MOCK|PLACEHOLDER)\b", combined, flags=re.IGNORECASE):
        raise AssertionError("unfinished marker found in D-Bus production or tests")
    checks += 1
    if re.search(r"\b(?:[0-9A-Fa-f]{2}:){5}[0-9A-Fa-f]{2}\b", combined):
        raise AssertionError("raw Bluetooth address found in D-Bus production or tests")
    checks += 1

    print(f"D-Bus source checks: {checks} passed")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except AssertionError as error:
        print(f"D-Bus source check failed: {error}", file=sys.stderr)
        raise SystemExit(1)
