#!/usr/bin/env python3
"""Dependency-free structural checks for the GNOME Shell companion source."""

from __future__ import annotations

import json
import re
import sys
import xml.etree.ElementTree as ET
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
EXTENSION = ROOT / "apps" / "gnome-extension"

REQUIRED = (
    "metadata.json",
    "extension.js",
    "service.js",
    "snapshot.js",
    "refresh-controller.js",
    "prefs.js",
    "stylesheet.css",
    "package.json",
    "icons/airbattery-symbolic.svg",
    "dbus/io.github.airbattery.Service1.xml",
    "schemas/io.github.airbattery.gnome.gschema.xml",
    "tests/snapshot.test.mjs",
    "tests/refresh-controller.test.mjs",
    "scripts/validate.sh",
    "scripts/install-user.sh",
)
ROOT_SCRIPTS = (
    "scripts/install-gnome-extension.sh",
    "scripts/package-gnome-extension.sh",
)
BUS_NAME = "io.github.airbattery.Service"
OBJECT_PATH = "/io/github/airbattery/Service"
INTERFACE_NAME = "io.github.airbattery.Service1"
METHODS = {"GetSnapshot", "Refresh", "OpenSettings", "ShowMainWindow"}
SIGNALS = {"SnapshotChanged"}
FORBIDDEN_MARKERS = re.compile(r"\b(?:TODO|FIXME|MOCK|PLACEHOLDER)\b", re.IGNORECASE)
FORBIDDEN_POLLING = ("setInterval", "setTimeout")
RAW_ADDRESS = re.compile(r"\b(?:[0-9A-Fa-f]{2}:){5}[0-9A-Fa-f]{2}\b")


def require(condition: bool, message: str, failures: list[str]) -> None:
    if not condition:
        failures.append(message)


def main() -> int:
    failures: list[str] = []
    checks = 0

    for relative in REQUIRED:
        checks += 1
        require((EXTENSION / relative).is_file(), f"missing required file: {relative}", failures)
    for relative in ROOT_SCRIPTS:
        checks += 1
        require((ROOT / relative).is_file(), f"missing required file: {relative}", failures)
    if failures:
        for failure in failures:
            print(f"error: {failure}", file=sys.stderr)
        return 1

    metadata = json.loads((EXTENSION / "metadata.json").read_text(encoding="utf-8"))
    checks += 5
    require(metadata.get("uuid") == "airbattery@airbattery.github.io", "unexpected extension UUID", failures)
    require(metadata.get("shell-version") == ["50"], "shell-version must claim only GNOME 50", failures)
    require(metadata.get("settings-schema") == "io.github.airbattery.gnome",
            "metadata settings schema does not match the installed schema", failures)
    require("version" not in metadata, "deprecated metadata version must be omitted", failures)
    require(metadata.get("url") == "https://github.com/majidsii/AirBattery",
            "extension repository URL does not point to majidsii/AirBattery", failures)

    contract_root = ET.parse(EXTENSION / "dbus/io.github.airbattery.Service1.xml").getroot()
    interface = contract_root.find("interface")
    checks += 5
    require(interface is not None, "D-Bus contract has no interface", failures)
    if interface is not None:
        require(interface.attrib.get("name") == INTERFACE_NAME, "unexpected D-Bus interface name", failures)
        methods = {node.attrib.get("name"): node for node in interface.findall("method")}
        require(set(methods) == METHODS, "D-Bus methods do not match the public contract", failures)
        refresh_args = methods.get("Refresh", ET.Element("method")).findall("arg")
        require(any(arg.attrib == {"name": "snapshot_json", "type": "s", "direction": "out"}
                    for arg in refresh_args), "Refresh must return snapshot JSON", failures)
        require({node.attrib.get("name") for node in interface.findall("signal")} == SIGNALS,
                "D-Bus signals do not match the public contract", failures)

    schema_files = sorted((EXTENSION / "schemas").glob("*.xml"))
    checks += 4
    require(len(schema_files) == 1, "exactly one GSettings schema must be shipped", failures)
    schema_root = ET.parse(EXTENSION / "schemas/io.github.airbattery.gnome.gschema.xml").getroot()
    schema = schema_root.find("schema")
    require(schema is not None, "GSettings schema is missing", failures)
    if schema is not None:
        require(schema.attrib.get("id") == "io.github.airbattery.gnome", "unexpected GSettings schema id", failures)
        require({key.attrib.get("name") for key in schema.findall("key")}
                == {"show-percentage", "preferred-device-id"},
                "GSettings keys do not match the extension contract", failures)

    extension_source = (EXTENSION / "extension.js").read_text(encoding="utf-8")
    refresh_source = (EXTENSION / "refresh-controller.js").read_text(encoding="utf-8")
    service_source = (EXTENSION / "service.js").read_text(encoding="utf-8")
    prefs_source = (EXTENSION / "prefs.js").read_text(encoding="utf-8")
    all_text = "\n".join(
        path.read_text(encoding="utf-8", errors="replace")
        for path in (*EXTENSION.rglob("*"), *(ROOT / "scripts").glob("*gnome-extension.sh"))
        if path.is_file() and path.suffix in {".js", ".mjs", ".json", ".xml", ".css", ".sh", ".svg"}
    )

    source_requirements = (
        (f"BUS_NAME = '{BUS_NAME}'" in service_source, "service module has the wrong bus name"),
        (f"OBJECT_PATH = '{OBJECT_PATH}'" in service_source, "service module has the wrong object path"),
        (INTERFACE_NAME in service_source, "service module has the wrong interface name"),
        ("connectSignal" in extension_source, "extension does not subscribe to SnapshotChanged"),
        ("notify::g-name-owner" in extension_source, "extension does not recover from service restarts"),
        ("GetSnapshot" in extension_source, "extension does not request an initial snapshot"),
        ("Refresh" in extension_source, "extension does not expose refresh"),
        ("OpenSettings" in extension_source, "extension does not expose settings launch"),
        ("ShowMainWindow" in extension_source, "extension does not expose main-window launch"),
        ("Main.panel.addToStatusArea" in extension_source, "extension does not register a top-bar indicator"),
        ("ExtensionPreferences" in prefs_source, "preferences entrypoint is missing"),
        ("preferredDeviceId" in extension_source, "application preferred device is not consumed"),
        ("open-state-changed" in extension_source, "menu opening does not request a refresh"),
        ("GLib.timeout_add" in extension_source, "bounded GNOME fallback timer is missing"),
        ("GLib.Source.remove" in extension_source, "GNOME fallback timer is not removed"),
        ("REFRESH_INTERVAL_MS = 3000" in refresh_source, "fallback interval must be exactly 3000 ms"),
        ("requestSnapshot" in refresh_source, "cached snapshot fallback is missing"),
        ("_inFlight" in refresh_source, "refresh overlap guard is missing"),
        ("selectPanelDevice" in extension_source, "extension does not select an active panel device"),
        ("this._indicator.visible = shouldShow" in extension_source,
         "extension visibility is not tied to an active device"),
    )
    checks += len(source_requirements)
    for condition, message in source_requirements:
        require(condition, message, failures)

    checks += len(FORBIDDEN_POLLING) + 6
    for token in FORBIDDEN_POLLING:
        require(token not in extension_source, f"unbounded polling token found: {token}", failures)
    require(extension_source.count("GLib.timeout_add") == 1,
            "extension must contain exactly one bounded GLib timer", failures)
    require("GetSnapshot" in extension_source and "Refresh" in extension_source,
            "event-first refresh must keep both cached and active D-Bus paths", failures)
    require(FORBIDDEN_MARKERS.search(all_text) is None, "unfinished marker found in GNOME source", failures)
    require(RAW_ADDRESS.search(all_text) is None, "raw Bluetooth address found in GNOME source", failures)
    require("sudo" not in all_text, "GNOME extension scripts must not require root", failures)
    for forbidden_bluetooth_token in ("bluetoothctl", "org.bluez", "ManufacturerData", "navigator.bluetooth"):
        checks += 1
        require(forbidden_bluetooth_token not in extension_source,
                f"GNOME extension must not access Bluetooth directly: {forbidden_bluetooth_token}", failures)
    require("gnome-shell/extensions/$UUID" in (ROOT / "scripts/install-gnome-extension.sh").read_text(encoding="utf-8"),
            "installer does not target the per-user GNOME extension directory", failures)

    if failures:
        for failure in failures:
            print(f"error: {failure}", file=sys.stderr)
        return 1

    print(f"GNOME source checks: {checks} passed")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
