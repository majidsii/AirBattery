#!/usr/bin/env python3
"""Dependency-free source checks for AirBattery release engineering."""

from __future__ import annotations

import configparser
import json
import re
import sys
import tomllib
import xml.etree.ElementTree as ET
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
errors: list[str] = []
checks = 0


def check(condition: bool, message: str) -> None:
    global checks
    checks += 1
    if not condition:
        errors.append(message)


def read(path: Path) -> str:
    return path.read_text(encoding="utf-8") if path.is_file() else ""


required_docs = [
    ROOT / "CHANGELOG.md",
    ROOT / "CONTRIBUTING.md",
    ROOT / "SECURITY.md",
    ROOT / "CODE_OF_CONDUCT.md",
    ROOT / "docs" / "RELEASE.md",
    ROOT / "docs" / "TROUBLESHOOTING.md",
]
required_packaging = [
    ROOT / "packaging" / "linux" / "io.github.airbattery.airbattery.desktop",
    ROOT / "packaging" / "linux" / "io.github.airbattery.airbattery.metainfo.xml",
    ROOT / "packaging" / "linux" / "io.github.airbattery.airbattery.svg",
    ROOT / "apps" / "desktop" / "src-tauri" / "icons" / "icon.svg",
    ROOT / "apps" / "desktop" / "src-tauri" / "icons" / "32x32.png",
    ROOT / "apps" / "desktop" / "src-tauri" / "icons" / "128x128.png",
    ROOT / "apps" / "desktop" / "src-tauri" / "icons" / "128x128@2x.png",
    ROOT / "apps" / "desktop" / "src-tauri" / "icons" / "icon.png",
    ROOT / "apps" / "desktop" / "src-tauri" / "icons" / "icon.ico",
]
required_scripts = [
    ROOT / "scripts" / "generate-icons.py",
    ROOT / "scripts" / "checksum-artifacts.py",
    ROOT / "scripts" / "verify-release-artifacts.py",
    ROOT / "scripts" / "verify-ci-source.py",
    ROOT / "scripts" / "verify-version-sync.py",
]
required_ci = [
    ROOT / ".github" / "workflows" / "ci.yml",
    ROOT / ".github" / "workflows" / "release.yml",
    ROOT / ".github" / "dependabot.yml",
    ROOT / "deny.toml",
]

for path in required_docs + required_packaging + required_scripts + required_ci:
    check(path.is_file(), f"missing release source: {path.relative_to(ROOT)}")
    if path.is_file():
        check(path.stat().st_size > 0, f"empty release source: {path.relative_to(ROOT)}")

for path in required_docs:
    text = read(path)
    check("TODO" not in text and "TBD" not in text, f"unfinished marker in {path.relative_to(ROOT)}")

changelog = read(ROOT / "CHANGELOG.md")
check("## [Unreleased]" in changelog, "CHANGELOG lacks Unreleased section")
check("0.1.0-alpha.1" in changelog, "CHANGELOG lacks current alpha version")

security = read(ROOT / "SECURITY.md")
check("GitHub Security Advisory" in security, "SECURITY lacks private advisory route")
check("do not open a public issue" in security.lower(), "SECURITY lacks public-disclosure warning")

release_doc = read(ROOT / "docs" / "RELEASE.md")
for token in ["draft", "SHA256SUMS", ".deb", "AppImage", ".exe", "hardware", "Definition of Done"]:
    check(token.lower() in release_doc.lower(), f"release guide missing: {token}")

identifier = "io.github.airbattery.airbattery"
desktop_path = ROOT / "packaging" / "linux" / f"{identifier}.desktop"
if desktop_path.is_file():
    parser = configparser.ConfigParser(interpolation=None, strict=True)
    parser.optionxform = str
    try:
        parser.read_string(read(desktop_path))
        entry = parser["Desktop Entry"]
        check(entry.get("Type") == "Application", "desktop entry Type must be Application")
        check(entry.get("Name") == "AirBattery", "desktop entry Name mismatch")
        check(entry.get("Exec") == "airbattery", "desktop entry Exec must match binary")
        check(entry.get("Icon") == identifier, "desktop entry Icon mismatch")
        check(entry.get("Terminal") == "false", "desktop entry must not use a terminal")
        check("Utility" in entry.get("Categories", ""), "desktop entry lacks Utility category")
    except Exception as error:  # noqa: BLE001
        errors.append(f"invalid desktop entry: {error}")
        checks += 1

metainfo_path = ROOT / "packaging" / "linux" / f"{identifier}.metainfo.xml"
if metainfo_path.is_file():
    try:
        root = ET.fromstring(read(metainfo_path))
        check(root.tag == "component", "AppStream root must be component")
        check(root.findtext("id") == identifier, "AppStream id mismatch")
        check(root.findtext("name") == "AirBattery", "AppStream name mismatch")
        check(root.findtext("project_license") == "MIT", "AppStream license mismatch")
        launchable = root.find("launchable")
        check(launchable is not None, "AppStream launchable missing")
        if launchable is not None:
            check(launchable.get("type") == "desktop-id", "AppStream launchable type mismatch")
            check(launchable.text == f"{identifier}.desktop", "AppStream desktop id mismatch")
        check(root.find("screenshots") is None, "AppStream must not include unverified screenshots")
    except Exception as error:  # noqa: BLE001
        errors.append(f"invalid AppStream XML: {error}")
        checks += 1

for icon_path in [
    ROOT / "packaging" / "linux" / f"{identifier}.svg",
    ROOT / "apps" / "desktop" / "src-tauri" / "icons" / "icon.svg",
]:
    if icon_path.is_file():
        text = read(icon_path)
        check("<svg" in text, f"icon is not SVG: {icon_path.relative_to(ROOT)}")
        check(identifier not in text, f"icon unexpectedly embeds application identifier: {icon_path.relative_to(ROOT)}")
        check("apple" not in text.lower(), f"icon references Apple assets: {icon_path.relative_to(ROOT)}")

try:
    from PIL import Image
except ImportError:
    Image = None

icon_specs = {
    "32x32.png": (32, 32),
    "128x128.png": (128, 128),
    "128x128@2x.png": (256, 256),
    "icon.png": (512, 512),
}
for filename, dimensions in icon_specs.items():
    path = ROOT / "apps" / "desktop" / "src-tauri" / "icons" / filename
    if path.is_file() and Image is not None:
        try:
            with Image.open(path) as image:
                check(image.size == dimensions, f"unexpected icon dimensions: {filename}")
                check(image.mode in {"RGBA", "RGB"}, f"unexpected icon mode: {filename}")
        except Exception as error:  # noqa: BLE001
            errors.append(f"invalid raster icon {filename}: {error}")
            checks += 1

if (ROOT / "apps" / "desktop" / "src-tauri" / "tauri.conf.json").is_file():
    config = json.loads(read(ROOT / "apps" / "desktop" / "src-tauri" / "tauri.conf.json"))
    check(config.get("identifier") == identifier, "Tauri identifier mismatch")
    icons = config.get("bundle", {}).get("icon", [])
    expected_icons = {"icons/32x32.png", "icons/128x128.png", "icons/128x128@2x.png", "icons/icon.png", "icons/icon.ico"}
    check(isinstance(icons, list) and expected_icons.issubset(set(icons)), "Tauri bundle icon resources missing")

if (ROOT / "Cargo.toml").is_file():
    cargo = tomllib.loads(read(ROOT / "Cargo.toml"))
    check(cargo.get("workspace", {}).get("package", {}).get("version") == "0.1.0-alpha.1", "workspace version mismatch")

for workflow_name in ["ci.yml", "release.yml"]:
    workflow = read(ROOT / ".github" / "workflows" / workflow_name)
    check("permissions:" in workflow, f"{workflow_name} lacks explicit permissions")
    check("actions/checkout@v7" in workflow, f"{workflow_name} does not pin checkout major")
    check("actions/setup-node@v6" in workflow, f"{workflow_name} does not pin setup-node major")
    check("dtolnay/rust-toolchain@stable" in workflow, f"{workflow_name} lacks Rust toolchain setup")

ci = read(ROOT / ".github" / "workflows" / "ci.yml")
check("pull_request:" in ci, "CI does not run on pull requests")
check("contents: read" in ci, "CI permissions are not read-only")
check("windows-latest" in ci, "CI lacks Windows runner")
check("cargo test --workspace" in ci, "CI lacks Rust tests")
check("./scripts/verify-foundation.sh" in ci, "CI lacks consolidated verification")
check("contents: write" not in ci, "CI must not write repository contents")

release = read(ROOT / ".github" / "workflows" / "release.yml")
check("tags:" in release and "v*" in release, "release workflow is not tag-gated")
check("draft: true" in release or "releaseDraft: true" in release, "release workflow does not create a draft")
check("tauri-apps/tauri-action@v1" in release, "release workflow lacks official Tauri action")
check("ubuntu-24.04" in release, "release workflow lacks native Linux runner")
check("windows-latest" in release, "release workflow lacks native Windows runner")
check("SHA256SUMS" in release, "release workflow lacks checksum manifest")
check("contents: write" in release, "release workflow lacks release permission")

marker = re.compile(r"\b(TODO|FIXME|MOCK|PLACEHOLDER)\b", re.IGNORECASE)
for path in required_scripts + required_ci + required_packaging:
    if path.is_file() and path.suffix.lower() in {".py", ".yml", ".yaml", ".toml", ".desktop", ".xml", ".svg"}:
        check(marker.search(read(path)) is None, f"unfinished marker in {path.relative_to(ROOT)}")

if errors:
    print(f"Release source checks: {len(errors)} failed / {checks} evaluated", file=sys.stderr)
    for error in errors:
        print(f"- {error}", file=sys.stderr)
    raise SystemExit(1)

print(f"Release source checks: {checks} passed")
