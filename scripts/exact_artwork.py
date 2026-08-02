#!/usr/bin/env python3
"""Shared validation, Wikimedia resolution, image normalization, and code generation.

The exact-artwork pipeline intentionally accepts only explicitly audited Wikimedia
Commons files whose file-description pages declare an approved Creative Commons
license. Product photographs are fetched at build time; the desktop app never
performs artwork network requests at runtime.
"""

from __future__ import annotations

import html
import json
import re
import urllib.parse
import urllib.request
from pathlib import Path
from typing import Any, Iterable, Mapping

from PIL import Image, ImageDraw, ImageOps

ALLOWED_LICENSES = {
    "CC0-1.0",
    "CC-BY-2.0",
    "CC-BY-3.0",
    "CC-BY-4.0",
    "CC-BY-SA-3.0",
    "CC-BY-SA-4.0",
}
ALLOWED_MODES = {"single", "pair", "case"}
ALLOWED_PROVIDERS = {"wikimedia-commons"}
ID_PATTERN = re.compile(r"^[a-z0-9][a-z0-9-]*$")
KEY_PATTERN = re.compile(r"^\s*'([^']+)'\s*:\s*'[^']+'\s*,?\s*$", re.MULTILINE)
WIKIMEDIA_API = "https://commons.wikimedia.org/w/api.php"
USER_AGENT = "AirBattery-exact-artwork/0.1 (+https://github.com/airbattery/airbattery)"


def _required_string(item: Mapping[str, Any], name: str, *, context: str) -> str:
    value = item.get(name)
    if not isinstance(value, str) or not value.strip():
        raise ValueError(f"{context}: {name} must be a non-empty string")
    return value.strip()


def _normalize_crop(value: Any, *, context: str) -> list[float]:
    if value is None:
        return [0.0, 0.0, 1.0, 1.0]
    if not isinstance(value, list) or len(value) != 4:
        raise ValueError(f"{context}: crop must contain four normalized coordinates")
    try:
        crop = [float(part) for part in value]
    except (TypeError, ValueError) as exc:
        raise ValueError(f"{context}: crop coordinates must be numeric") from exc
    if any(part < 0.0 or part > 1.0 for part in crop):
        raise ValueError(f"{context}: crop coordinates must stay within 0..1")
    left, top, right, bottom = crop
    if left >= right or top >= bottom:
        raise ValueError(f"{context}: crop must have positive width and height")
    return crop


def read_registered_artwork_keys(path: Path) -> set[str]:
    """Extract public artwork keys from the pure TypeScript alias registry."""

    text = path.read_text(encoding="utf-8")
    keys = set(KEY_PATTERN.findall(text))
    if not keys:
        raise ValueError(f"no artwork keys were found in {path}")
    return keys


def validate_manifest(data: Any, registered_keys: set[str]) -> dict[str, Any]:
    """Validate and deterministically normalize the checked-in source manifest."""

    if not isinstance(data, dict):
        raise ValueError("exact artwork manifest must be a JSON object")
    if data.get("schemaVersion") != 1:
        raise ValueError("exact artwork manifest schemaVersion must be 1")
    raw_sources = data.get("sources")
    if not isinstance(raw_sources, list):
        raise ValueError("exact artwork manifest sources must be a list")

    normalized: list[dict[str, Any]] = []
    seen_ids: set[str] = set()
    seen_slots: set[tuple[str, str]] = set()

    for index, raw in enumerate(raw_sources):
        context = f"sources[{index}]"
        if not isinstance(raw, dict):
            raise ValueError(f"{context}: source must be an object")

        source_id = _required_string(raw, "id", context=context).lower()
        if not ID_PATTERN.fullmatch(source_id):
            raise ValueError(f"{context}: id must use lowercase letters, digits, and hyphens")
        if source_id in seen_ids:
            raise ValueError(f"{context}: duplicate source id {source_id}")
        seen_ids.add(source_id)

        artwork_key = _required_string(raw, "artworkKey", context=context).lower()
        if artwork_key not in registered_keys:
            raise ValueError(f"{context}: unknown artwork key {artwork_key}")

        mode = _required_string(raw, "mode", context=context).lower()
        if mode not in ALLOWED_MODES:
            raise ValueError(f"{context}: mode must be one of {sorted(ALLOWED_MODES)}")
        slot = (artwork_key, mode)
        if slot in seen_slots:
            raise ValueError(f"{context}: duplicate exact artwork slot {artwork_key}/{mode}")
        seen_slots.add(slot)

        provider = _required_string(raw, "provider", context=context).lower()
        if provider not in ALLOWED_PROVIDERS:
            raise ValueError(f"{context}: unsupported provider {provider}")

        license_id = _required_string(raw, "license", context=context).upper()
        if license_id not in ALLOWED_LICENSES:
            raise ValueError(
                f"{context}: license must be an approved Creative Commons license; got {license_id}"
            )

        mirror_safe = raw.get("mirrorSafe", False)
        if not isinstance(mirror_safe, bool):
            raise ValueError(f"{context}: mirrorSafe must be boolean")

        normalized.append(
            {
                "id": source_id,
                "artworkKey": artwork_key,
                "mode": mode,
                "provider": provider,
                "fileTitle": _required_string(raw, "fileTitle", context=context),
                "sourcePage": _required_string(raw, "sourcePage", context=context),
                "author": _required_string(raw, "author", context=context),
                "license": license_id,
                "licenseUrl": _required_string(raw, "licenseUrl", context=context),
                "mirrorSafe": mirror_safe,
                "crop": _normalize_crop(raw.get("crop"), context=context),
                "outputName": f"{source_id}.webp",
            }
        )

    normalized.sort(key=lambda item: (item["artworkKey"], item["mode"], item["id"]))
    return {"schemaVersion": 1, "sources": normalized}


def _import_name(source_id: str) -> str:
    parts = source_id.split("-")
    return "exact" + "".join(part[:1].upper() + part[1:] for part in parts)


def generate_typescript(sources: Iterable[Mapping[str, Any]]) -> str:
    """Generate deterministic Vite imports and the exact-artwork override map."""

    ordered = sorted(sources, key=lambda item: (str(item["artworkKey"]), str(item["mode"])))
    lines = ["// Generated by scripts/fetch-exact-artwork.py. Do not hand-edit.", ""]
    for source in ordered:
        lines.append(
            f"import {_import_name(str(source['id']))} from "
            f"'../assets/device-artwork/exact/{source['outputName']}';"
        )
    if ordered:
        lines.append("")
    lines.extend(
        [
            "export interface ExactArtworkAsset {",
            "  src: string;",
            "  mirrorSafe: boolean;",
            "}",
            "",
            "export interface ExactArtworkAssetSet {",
            "  single?: ExactArtworkAsset;",
            "  pair?: ExactArtworkAsset;",
            "  case?: ExactArtworkAsset;",
            "}",
            "",
            "export type ExactArtworkAssetMap = Record<string, ExactArtworkAssetSet>;",
            "",
        ]
    )

    grouped: dict[str, list[Mapping[str, Any]]] = {}
    for source in ordered:
        grouped.setdefault(str(source["artworkKey"]), []).append(source)

    if not grouped:
        lines.append("export const exactArtworkAssets: ExactArtworkAssetMap = {};")
        lines.append("")
        return "\n".join(lines)

    lines.append("export const exactArtworkAssets: ExactArtworkAssetMap = {")
    for artwork_key in sorted(grouped):
        lines.append(f"  '{artwork_key}': {{")
        for source in sorted(grouped[artwork_key], key=lambda item: str(item["mode"])):
            mirror = "true" if source.get("mirrorSafe") is True else "false"
            lines.append(
                f"    {source['mode']}: {{ src: {_import_name(str(source['id']))}, "
                f"mirrorSafe: {mirror} }},"
            )
        lines.append("  },")
    lines.extend(["};", ""])
    return "\n".join(lines)


def _plain_metadata(value: Any) -> str:
    if not isinstance(value, dict):
        return ""
    text = value.get("value")
    if not isinstance(text, str):
        return ""
    return re.sub(r"<[^>]+>", "", html.unescape(text)).strip()


def normalize_license_id(value: str) -> str:
    normalized = value.upper().replace("_", "-").replace(" ", "-")
    normalized = normalized.replace("CREATIVE-COMMONS-", "")
    normalized = normalized.replace("ATTRIBUTION-SHAREALIKE", "BY-SA")
    normalized = normalized.replace("ATTRIBUTION-SHARE-ALIKE", "BY-SA")
    normalized = normalized.replace("ATTRIBUTION", "BY")
    normalized = normalized.replace("-INTERNATIONAL", "")
    normalized = normalized.replace("-UNPORTED", "")
    normalized = re.sub(r"-+", "-", normalized).strip("-")
    if normalized.startswith(("BY-", "BY-SA-")):
        normalized = f"CC-{normalized}"
    aliases = {
        "CC-BY-SA-4.0": "CC-BY-SA-4.0",
        "CC-BY-SA-3.0": "CC-BY-SA-3.0",
        "CC-BY-4.0": "CC-BY-4.0",
        "CC-BY-3.0": "CC-BY-3.0",
        "CC-BY-2.0": "CC-BY-2.0",
        "CC0": "CC0-1.0",
        "CC-ZERO": "CC0-1.0",
        "CCZERO": "CC0-1.0",
        "CC0-1.0": "CC0-1.0",
        "CC-0-1.0": "CC0-1.0",
        "PUBLIC-DOMAIN-CC0": "CC0-1.0",
    }
    if normalized in aliases:
        return aliases[normalized]
    for license_id in sorted(ALLOWED_LICENSES, key=len, reverse=True):
        if license_id in normalized:
            return license_id
    return normalized


def resolve_wikimedia_source(source: Mapping[str, Any], *, timeout: int = 30) -> dict[str, Any]:
    """Resolve a Commons file title to its canonical original URL and live metadata."""

    params = urllib.parse.urlencode(
        {
            "action": "query",
            "format": "json",
            "formatversion": "2",
            "prop": "imageinfo",
            "iiprop": "url|extmetadata|mime|size|sha1",
            "titles": source["fileTitle"],
        }
    )
    request = urllib.request.Request(
        f"{WIKIMEDIA_API}?{params}", headers={"User-Agent": USER_AGENT}
    )
    with urllib.request.urlopen(request, timeout=timeout) as response:
        payload = json.load(response)

    pages = payload.get("query", {}).get("pages", [])
    if len(pages) != 1 or pages[0].get("missing") is True:
        raise ValueError(f"Wikimedia file was not found: {source['fileTitle']}")
    image_info = pages[0].get("imageinfo") or []
    if len(image_info) != 1:
        raise ValueError(f"Wikimedia file has no downloadable image info: {source['fileTitle']}")
    info = image_info[0]
    resolved_url = info.get("url")
    if not isinstance(resolved_url, str) or not resolved_url.startswith("https://"):
        raise ValueError(f"Wikimedia returned an invalid image URL for {source['fileTitle']}")

    metadata = info.get("extmetadata") or {}
    license_candidates = [
        _plain_metadata(metadata.get("LicenseShortName")),
        _plain_metadata(metadata.get("UsageTerms")),
        _plain_metadata(metadata.get("License")),
    ]
    resolved_license = ""
    for candidate in license_candidates:
        normalized = normalize_license_id(candidate)
        if normalized in ALLOWED_LICENSES:
            resolved_license = normalized
            break
    if not resolved_license:
        raise ValueError(f"Wikimedia license is not approved for {source['fileTitle']}")
    if resolved_license != source["license"]:
        raise ValueError(
            f"Wikimedia license changed for {source['fileTitle']}: "
            f"manifest={source['license']} live={resolved_license}"
        )

    resolved_author = _plain_metadata(metadata.get("Artist")) or str(source["author"])
    return {
        "resolvedUrl": resolved_url,
        "resolvedAuthor": resolved_author,
        "resolvedLicense": resolved_license,
        "mime": info.get("mime"),
        "width": info.get("width"),
        "height": info.get("height"),
        "sha1": info.get("sha1"),
        "canonicalTitle": pages[0].get("title", source["fileTitle"]),
        "descriptionUrl": info.get("descriptionurl", source["sourcePage"]),
    }


def download_file(url: str, output: Path, *, timeout: int = 60) -> None:
    output.parent.mkdir(parents=True, exist_ok=True)
    temporary = output.with_suffix(output.suffix + ".part")
    request = urllib.request.Request(url, headers={"User-Agent": USER_AGENT})
    try:
        with urllib.request.urlopen(request, timeout=timeout) as response, temporary.open("wb") as handle:
            while True:
                chunk = response.read(1024 * 1024)
                if not chunk:
                    break
                handle.write(chunk)
        temporary.replace(output)
    finally:
        temporary.unlink(missing_ok=True)


def normalize_photo(source: Path, output: Path, *, crop: list[float], size: int = 1024) -> None:
    """Crop without geometric distortion and place the photo on a transparent square."""

    if size < 128 or size > 4096:
        raise ValueError("artwork size must be between 128 and 4096 pixels")
    normalized_crop = _normalize_crop(crop, context="normalize_photo")

    with Image.open(source) as opened:
        image = ImageOps.exif_transpose(opened).convert("RGBA")
    width, height = image.size
    left, top, right, bottom = normalized_crop
    pixel_box = (
        max(0, min(width - 1, round(left * width))),
        max(0, min(height - 1, round(top * height))),
        max(1, min(width, round(right * width))),
        max(1, min(height, round(bottom * height))),
    )
    cropped = image.crop(pixel_box)

    padding = max(12, round(size * 0.055))
    available = size - padding * 2
    scale = min(available / cropped.width, available / cropped.height)
    resized_size = (
        max(1, round(cropped.width * scale)),
        max(1, round(cropped.height * scale)),
    )
    resized = cropped.resize(resized_size, Image.Resampling.LANCZOS)

    # Preserve the real photograph. Only soften its rectangular crop edge so it
    # sits cleanly on dark and light cards; no subject segmentation or reshaping.
    radius = max(4, round(min(resized_size) * 0.025))
    edge_mask = Image.new("L", resized_size, 0)
    ImageDraw.Draw(edge_mask).rounded_rectangle(
        (0, 0, resized_size[0] - 1, resized_size[1] - 1), radius=radius, fill=255
    )
    existing_alpha = resized.getchannel("A")
    resized.putalpha(Image.composite(existing_alpha, Image.new("L", resized_size, 0), edge_mask))

    canvas = Image.new("RGBA", (size, size), (0, 0, 0, 0))
    position = ((size - resized.width) // 2, (size - resized.height) // 2)
    canvas.alpha_composite(resized, position)
    output.parent.mkdir(parents=True, exist_ok=True)
    canvas.save(output, "WEBP", quality=92, method=6, lossless=False)


def attribution_record(source: Mapping[str, Any], resolved: Mapping[str, Any]) -> dict[str, Any]:
    return {
        "id": source["id"],
        "artworkKey": source["artworkKey"],
        "mode": source["mode"],
        "outputName": source["outputName"],
        "provider": source["provider"],
        "fileTitle": source["fileTitle"],
        "sourcePage": source["sourcePage"],
        "author": resolved.get("resolvedAuthor") or source["author"],
        "license": resolved.get("resolvedLicense") or source["license"],
        "licenseUrl": source["licenseUrl"],
        "resolvedUrl": resolved.get("resolvedUrl"),
        "sha1": resolved.get("sha1"),
        "modifications": ["crop", "resize without stretching", "WebP conversion"],
    }


def load_and_validate_manifest(root: Path) -> dict[str, Any]:
    manifest_path = root / "assets/exact-artwork/sources.json"
    manifest_data = json.loads(manifest_path.read_text(encoding="utf-8"))
    keys = read_registered_artwork_keys(root / "apps/desktop/src/domain/artwork-keys.ts")
    return validate_manifest(manifest_data, keys)
