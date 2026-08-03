#!/usr/bin/env python3
"""Dependency-light validation for AirBattery package identity and icon resources."""

from __future__ import annotations

import json
import sys
import xml.etree.ElementTree as ET
from pathlib import Path

from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
IDENTIFIER = "io.github.airbattery.airbattery"
LINUX = ROOT / "packaging/linux"
ICONS = ROOT / "apps/desktop/src-tauri/icons"
errors: list[str] = []
checks = 0


def check(condition: bool, message: str) -> None:
    global checks
    checks += 1
    if not condition:
        errors.append(message)


def require(path: Path) -> str:
    check(path.is_file(), f"missing file: {path.relative_to(ROOT)}")
    if not path.is_file():
        return ""
    text = path.read_text(encoding="utf-8")
    check(bool(text.strip()), f"empty file: {path.relative_to(ROOT)}")
    return text


desktop_text = require(LINUX / f"{IDENTIFIER}.desktop")
for line in (
    "[Desktop Entry]",
    "Type=Application",
    "Name=AirBattery",
    "Exec=airbattery",
    f"Icon={IDENTIFIER}",
    "Terminal=false",
):
    check(line in desktop_text.splitlines(), f"desktop entry missing {line}")
check("Utility;" in desktop_text, "desktop entry Utility category missing")
check("Bluetooth;" in desktop_text, "desktop entry Bluetooth keyword missing")

metainfo_path = LINUX / f"{IDENTIFIER}.metainfo.xml"
require(metainfo_path)
if metainfo_path.is_file():
    try:
        root = ET.parse(metainfo_path).getroot()
        check(root.tag == "component", "AppStream root must be component")
        check(root.attrib.get("type") == "desktop-application", "AppStream type mismatch")
        check(root.findtext("id") == IDENTIFIER, "AppStream id mismatch")
        check(root.findtext("name") == "AirBattery", "AppStream name mismatch")
        check(root.findtext("project_license") == "MIT", "AppStream project license mismatch")
        launchable = root.find("launchable")
        check(launchable is not None and launchable.attrib.get("type") == "desktop-id", "desktop-id launchable missing")
        check(launchable is not None and launchable.text == f"{IDENTIFIER}.desktop", "desktop-id launchable mismatch")
        check(root.find("description") is not None, "AppStream description missing")
        check(root.find("content_rating") is not None, "AppStream content rating missing")
        urls = {(node.attrib.get("type"), node.text) for node in root.findall("url")}
        check(("homepage", "https://github.com/majidsii/AirBattery") in urls,
              "AppStream homepage URL mismatch")
        check(("bugtracker", "https://github.com/majidsii/AirBattery/issues") in urls,
              "AppStream bugtracker URL mismatch")
    except Exception as error:  # noqa: BLE001
        errors.append(f"invalid AppStream XML: {error}")
        checks += 1

for path in (LINUX / f"{IDENTIFIER}.svg", ICONS / "icon.svg"):
    text = require(path)
    check("<svg" in text and "viewBox=\"0 0 512 512\"" in text, f"unexpected SVG canvas: {path.relative_to(ROOT)}")
    check("Apple" not in text and "airpods" not in text.lower(), f"protected product branding in icon: {path.relative_to(ROOT)}")

expected_pngs = {
    "32x32.png": (32, 32),
    "128x128.png": (128, 128),
    "128x128@2x.png": (256, 256),
    "icon.png": (512, 512),
}
for name, size in expected_pngs.items():
    path = ICONS / name
    check(path.is_file(), f"missing generated icon: {path.relative_to(ROOT)}")
    if path.is_file():
        with Image.open(path) as image:
            check(image.size == size, f"wrong icon dimensions for {name}: {image.size}")
            check(image.mode in {"RGBA", "RGB"}, f"unexpected icon mode for {name}: {image.mode}")

ico = ICONS / "icon.ico"
check(ico.is_file(), "Windows icon.ico missing")
if ico.is_file():
    with Image.open(ico) as image:
        sizes = set(image.info.get("sizes", set()))
        check({(16, 16), (32, 32), (48, 48), (256, 256)}.issubset(sizes), f"ICO sizes incomplete: {sorted(sizes)}")

config_path = ROOT / "apps/desktop/src-tauri/tauri.conf.json"
config = json.loads(require(config_path)) if config_path.is_file() else {}
bundle = config.get("bundle", {})
check(bundle.get("icon") == [
    "icons/32x32.png",
    "icons/128x128.png",
    "icons/128x128@2x.png",
    "icons/icon.png",
    "icons/icon.ico",
], "Tauri bundle icon list mismatch")

script = require(ROOT / "scripts/generate-icons.py")
for token in ("cairosvg.svg2png", "Image.open", "icon.ico", "128x128@2x.png"):
    check(token in script, f"icon generator missing {token}")

if errors:
    print(f"packaging source checks: {len(errors)} failed / {checks} evaluated", file=sys.stderr)
    for error in errors:
        print(f"- {error}", file=sys.stderr)
    raise SystemExit(1)

print(f"packaging source checks: {checks} passed")
