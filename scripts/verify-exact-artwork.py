#!/usr/bin/env python3
"""Verify exact-artwork source, generated imports, images, and attribution."""

from __future__ import annotations

import json
import re
import sys
from pathlib import Path

from PIL import Image

from exact_artwork import generate_typescript, load_and_validate_manifest

IMPORT_PATTERN = re.compile(
    r"^import\s+\w+\s+from\s+'\.\./assets/device-artwork/exact/([^']+\.webp)';$",
    re.MULTILINE,
)


def fail(message: str) -> None:
    raise ValueError(message)


def main() -> int:
    root = Path(__file__).resolve().parents[1]
    manifest = load_and_validate_manifest(root)
    all_sources = manifest["sources"]
    expected_by_name = {source["outputName"]: source for source in all_sources}

    output_dir = root / "apps/desktop/src/assets/device-artwork/exact"
    generated_path = root / "apps/desktop/src/domain/exact-artwork.generated.ts"
    attribution_path = output_dir / "ATTRIBUTION.generated.json"

    if not generated_path.exists():
        fail("generated exact-artwork TypeScript file is missing")
    generated_text = generated_path.read_text(encoding="utf-8")
    imported_names = set(IMPORT_PATTERN.findall(generated_text))
    disk_names = {path.name for path in output_dir.glob("*.webp")}
    if imported_names != disk_names:
        fail(
            "generated exact-artwork imports and WebP files differ: "
            f"imports={sorted(imported_names)} files={sorted(disk_names)}"
        )
    unknown_files = disk_names - expected_by_name.keys()
    if unknown_files:
        fail(f"unregistered exact artwork files: {sorted(unknown_files)}")

    selected_sources = [expected_by_name[name] for name in sorted(disk_names)]
    expected_generated = generate_typescript(selected_sources)
    if generated_text != expected_generated:
        fail("exact-artwork TypeScript is stale; rerun scripts/fetch-exact-artwork.py")

    for name in sorted(disk_names):
        path = output_dir / name
        with Image.open(path) as image:
            if image.format != "WEBP":
                fail(f"{name} is not WebP")
            if image.size[0] != image.size[1]:
                fail(f"{name} is not square")
            if image.size[0] < 512:
                fail(f"{name} is below the 512px minimum")

    if not attribution_path.exists():
        fail("exact-artwork attribution file is missing")
    attribution = json.loads(attribution_path.read_text(encoding="utf-8"))
    if attribution.get("schemaVersion") != 1 or not isinstance(attribution.get("sources"), list):
        fail("exact-artwork attribution schema is invalid")
    attribution_ids = {item.get("id") for item in attribution["sources"]}
    selected_ids = {source["id"] for source in selected_sources}
    if attribution_ids != selected_ids:
        fail(
            "attribution records do not match fetched artwork: "
            f"attribution={sorted(attribution_ids)} fetched={sorted(selected_ids)}"
        )

    print(f"Exact artwork manifest sources: {len(all_sources)}")
    print(f"Fetched exact artwork files: {len(disk_names)}")
    print("Exact artwork verification completed.")
    return 0


if __name__ == "__main__":
    try:
        raise SystemExit(main())
    except Exception as error:  # noqa: BLE001 - command-line validator.
        print(f"Exact artwork verification failed: {error}", file=sys.stderr)
        raise SystemExit(1)
