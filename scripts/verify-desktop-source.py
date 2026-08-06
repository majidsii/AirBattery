#!/usr/bin/env python3
"""Dependency-free structural checks for the authored desktop application."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
DESKTOP = ROOT / "apps" / "desktop"
SRC = DESKTOP / "src"

errors: list[str] = []
checks = 0


def check(condition: bool, message: str) -> None:
    global checks
    checks += 1
    if not condition:
        errors.append(message)


required = [
    DESKTOP / "package.json",
    DESKTOP / "tsconfig.contract.json",
    DESKTOP / "index.html",
    SRC / "App.vue",
    SRC / "main.ts",
    SRC / "api" / "backend.ts",
    SRC / "domain" / "presentation.ts",
    SRC / "domain" / "settings.ts",
    SRC / "stores" / "devices.ts",
    SRC / "stores" / "settings.ts",
    SRC / "views" / "BatteryView.vue",
    SRC / "views" / "DevicesView.vue",
    SRC / "views" / "SettingsView.vue",
    SRC / "views" / "DiagnosticsView.vue",
    SRC / "views" / "AboutView.vue",
    SRC / "components" / "BrandMark.vue",
    SRC / "styles" / "tokens.css",
    SRC / "styles" / "base.css",
    SRC / "styles" / "ambient.css",
    SRC / "styles" / "glass.css",
    SRC / "domain" / "glass.ts",
]
for path in required:
    check(path.is_file(), f"missing required desktop file: {path.relative_to(ROOT)}")

ARTWORK_ROOT = SRC / "assets" / "device-artwork"
PRE_RENDERED = SRC / "domain" / "pre-rendered-artwork.ts"
check(PRE_RENDERED.is_file(), "pre-rendered artwork registry is missing")
check(not (ARTWORK_ROOT / "catalog").exists(), "unreviewed model-specific SVG catalog must not be present")
check(not (ARTWORK_ROOT / "exact").exists(), "legacy exact-photo artwork must not be present")
check(not (ARTWORK_ROOT / "studio").exists(), "synthetic studio artwork must not be present")
check(not (ROOT / "scripts" / "fetch-exact-artwork.py").exists(), "runtime artwork downloader must not be present")

catalog_source = (SRC / "domain" / "artwork-catalog.ts").read_text(encoding="utf-8")
component_source = (SRC / "components" / "DeviceArtwork.vue").read_text(encoding="utf-8")
check("preRenderedArtworkAssets" in catalog_source, "runtime artwork catalog does not use the reviewed registry")
check("candidate.modelKey !== normalizedKey" in catalog_source, "runtime artwork catalog does not enforce exact model identity")
check("candidate.mode !== mode" in catalog_source, "runtime artwork catalog does not enforce exact component orientation")
check("resolvePreRenderedArtworkAsset" in component_source, "device artwork does not use the exact review gate")
check("device-artwork__generic-fallback" in component_source, "neutral inline category fallback is missing")
check("device-artwork/catalog" not in catalog_source, "runtime artwork catalog references removed model drawings")
check(".svg'" not in catalog_source, "runtime artwork catalog imports unreviewed SVG product drawings")
check(".webp'" not in catalog_source, "runtime artwork catalog bundles unreviewed raster product art")
check("mirrorHorizontally" not in catalog_source, "reviewed product artwork must not be mirrored")
check("mirrored" not in component_source, "device artwork still mirrors an orientation")
about_source = (SRC / "views" / "AboutView.vue").read_text(encoding="utf-8")
check("Exact-or-generic artwork" in about_source, "exact-or-generic artwork policy is missing from About")
check("ATTRIBUTION.generated.json" not in about_source, "removed photo attribution is still imported")

artwork_doc = (ROOT / "docs" / "DEVICE_ARTWORK_CATALOG.md").read_text(encoding="utf-8")
check("exact-or-generic" in artwork_doc.lower(), "artwork catalog documentation does not describe the exact-or-generic policy")
check("87 SVG files" not in artwork_doc, "artwork catalog documentation still claims the removed 87-SVG runtime catalog")
check("registry starts empty" in artwork_doc.lower(), "artwork catalog documentation does not state the audited registry starts empty")

changelog = (ROOT / "CHANGELOG.md").read_text(encoding="utf-8")
check(changelog.startswith("# Changelog"), "changelog must begin with its document title")
check("Removed the former 87-file model-specific SVG catalog" in changelog, "changelog does not record removal of the misleading SVG catalog")


TAURI = DESKTOP / "src-tauri"
tauri_required = [
    TAURI / "Cargo.toml",
    TAURI / "tauri.conf.json",
    TAURI / "capabilities" / "default.json",
    TAURI / "src" / "lib.rs",
    TAURI / "src" / "main.rs",
    TAURI / "src" / "state.rs",
    TAURI / "src" / "commands.rs",
    TAURI / "src" / "platform" / "linux.rs",
    TAURI / "src" / "tray.rs",
    ]
for path in tauri_required:
    check(path.is_file(), f"missing required Tauri file: {path.relative_to(ROOT)}")

for json_path in [DESKTOP / "package.json", DESKTOP / "tsconfig.json", DESKTOP / "src-tauri" / "tauri.conf.json", DESKTOP / "src-tauri" / "capabilities" / "default.json"]:
    try:
        if not json_path.exists():
            continue
        json.loads(json_path.read_text(encoding="utf-8"))
    except Exception as error:  # noqa: BLE001 - verifier must report malformed files
        errors.append(f"invalid JSON in {json_path.relative_to(ROOT)}: {error}")
    checks += 1

package = json.loads((DESKTOP / "package.json").read_text(encoding="utf-8"))
for unused_native_dependency in (
    "@tauri-apps/plugin-autostart",
    "@tauri-apps/plugin-notification",
    "@tauri-apps/plugin-store",
):
    check(
        unused_native_dependency not in package.get("dependencies", {}),
        f"unused frontend-native dependency present: {unused_native_dependency}",
    )

for section in ("dependencies", "devDependencies"):
    for name, version in package.get(section, {}).items():
        check(
            isinstance(version, str) and not version.startswith(("^", "~", ">", "<", "*")),
            f"dependency is not exactly pinned: {name}={version}",
        )

production_files = [
    path for path in SRC.rglob("*") if path.suffix in {".ts", ".vue", ".css"}
]
marker_pattern = re.compile(r"\b(TODO|FIXME|MOCK|PLACEHOLDER)\b|\.skip\s*\(", re.IGNORECASE)
hardcoded_battery_pattern = re.compile(r"\bpercentage\s*:\s*(?:[1-9]\d?|100)\b")
raw_address_pattern = re.compile(r"\b(?:[0-9A-Fa-f]{2}:){5}[0-9A-Fa-f]{2}\b")
for path in production_files:
    text = path.read_text(encoding="utf-8")
    check(marker_pattern.search(text) is None, f"unfinished marker in {path.relative_to(ROOT)}")
    check(hardcoded_battery_pattern.search(text) is None, f"hardcoded battery value in {path.relative_to(ROOT)}")
    check(raw_address_pattern.search(text) is None, f"raw Bluetooth address in {path.relative_to(ROOT)}")
    check("v-html" not in text, f"unsafe v-html usage in {path.relative_to(ROOT)}")

template_open_pattern = re.compile(r"<template(?:\s[^>]*)?>")
for path in SRC.rglob("*.vue"):
    text = path.read_text(encoding="utf-8")
    template_openings = len(template_open_pattern.findall(text))
    template_closings = text.count("</template>")
    check(template_openings >= 1, f"missing template in {path.relative_to(ROOT)}")
    check(template_openings == template_closings, f"unbalanced template in {path.relative_to(ROOT)}")
    check(text.count("<script setup") <= 1, f"unexpected script setup count in {path.relative_to(ROOT)}")

css = (SRC / "styles" / "base.css").read_text(encoding="utf-8")
check(css.count("{") == css.count("}"), "unbalanced braces in base.css")
check("prefers-reduced-motion" in css, "reduced-motion fallback missing")
check(".skip-link" in css, "skip-link styling missing")
check("@supports not (backdrop-filter" in css, "no-blur fallback missing")
glass_css = (SRC / "styles" / "glass.css").read_text(encoding="utf-8")
tokens_css = (SRC / "styles" / "tokens.css").read_text(encoding="utf-8")
ambient_path = SRC / "styles" / "ambient.css"
ambient_css = ambient_path.read_text(encoding="utf-8") if ambient_path.is_file() else ""
app = (SRC / "App.vue").read_text(encoding="utf-8")
check("--canvas: #090909" in tokens_css.lower(), "neutral black Option 2 canvas token is missing")
check("--canvas: #f4f4f5" in tokens_css.lower(), "pearl light Option 2 canvas token is missing")
check("--wave-fill: 0, 0, 0" in tokens_css, "dark waves are not black/graphite")
check("--wave-fill: 255, 255, 255" in tokens_css, "light waves are not white/pearl")
check("rgba(var(--wave-fill)" in ambient_css, "ambient waves do not use the semantic wave token")
check("ambient-background__art--top" in app, "approved upper-right SVG ribbon is missing")
check("ambient-background__art--bottom" in app, "approved lower-left SVG ribbon is missing")
check(app.count("ambient-background__line") >= 8, "approved layered contour lines are missing")
check("ambient-background__wave" not in app, "deprecated oval ambient blobs are still rendered")
check("radial-gradient" not in ambient_css, "ambient background regressed to blurred radial blobs")
check(
    "inset: 0;" in ambient_css
    and "width: 100%;" in ambient_css
    and "height: 100%;" in ambient_css,
    "ambient artwork does not fill the viewport responsively",
)
check(
    "@media (max-width: 780px)" not in ambient_css
    and "150vw" not in ambient_css
    and "178vw" not in ambient_css,
    "ambient artwork still uses breakpoint-specific geometry",
)
check(
    app.count('preserveAspectRatio="none"') == 2,
    "ambient SVG geometry is not responsive",
)
check(
    "to right," in ambient_css
    and "to left," in ambient_css
    and ambient_css.count("transparent 88%") >= 4,
    "ambient ribbon fade masks are missing",
)
check("ambient-background" not in glass_css, "ambient artwork must not live in glass.css")
check(
    re.search(
        r"\.glass-shell \.app-content\s*\{[^}]*background:\s*transparent;[^}]*backdrop-filter:\s*none;",
        glass_css,
        re.S,
    )
    is not None,
    "main content shell still hides the ambient background",
)
check(
    "border-radius: 18px; color: var(--text);" in css,
    "sidebar brand mark is not theme-aware",
)
check(".glass-card::before" in glass_css, "Liquid Glass specular highlight layer is missing")
check("prefers-reduced-transparency" in glass_css, "reduced-transparency fallback is missing")
check("data-glass-reduced=\"true\"" in glass_css, "explicit solid glass fallback is missing")
check("--glass-popup-blur" in glass_css, "connection popup glass token is missing")
check(".overview-header" in glass_css, "refined overview hierarchy styling is missing")
check("scrollbar-color: transparent transparent" in glass_css,
      "scrollbar must remain hidden until the content surface is engaged")
check(".app-content:hover::-webkit-scrollbar-thumb" in glass_css,
      "hover-revealed WebKit scrollbar styling is missing")
check("::-webkit-scrollbar-button" in glass_css and "display: none" in glass_css,
      "native scrollbar arrow buttons must be suppressed")

glass_domain = (SRC / "domain" / "glass.ts").read_text(encoding="utf-8")
check("type NamedGlassPreset" in glass_domain, "named glass presets are missing")
check("GlassSurfaceStrengths" in glass_domain, "per-surface glass intensity is missing")
check("GLASS_STORAGE_KEY" in glass_domain, "glass preference persistence key is missing")

for view in ["BatteryView", "DevicesView", "SettingsView", "DiagnosticsView", "AboutView"]:
    check(view in app, f"{view} is not wired into App.vue")
check("Skip to content" in app, "keyboard skip link missing")
battery_view = (SRC / "views" / "BatteryView.vue").read_text(encoding="utf-8")
check('class="overview-header"' in battery_view, "refined Overview header is missing")
check("overview-hero glass-titlebar" not in battery_view,
      "legacy giant Overview title panel is still present")
presentation_source = (SRC / "domain" / "presentation.ts").read_text(encoding="utf-8")
check("'partybox'" in presentation_source and "return 'speaker-generic'" in presentation_source,
      "JBL PartyBox devices are not classified as speakers")

if errors:
    print(f"desktop source checks: {len(errors)} failed / {checks} evaluated", file=sys.stderr)
    for error in errors:
        print(f"- {error}", file=sys.stderr)
    raise SystemExit(1)

print(f"desktop source checks: {checks} passed")
