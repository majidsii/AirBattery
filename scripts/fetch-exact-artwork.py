#!/usr/bin/env python3
"""Fetch audited exact product photos and generate desktop artwork imports."""

from __future__ import annotations

import argparse
import json
import shutil
import sys
from pathlib import Path
from urllib.parse import urlparse

from exact_artwork import (
    attribution_record,
    download_file,
    generate_typescript,
    load_and_validate_manifest,
    normalize_photo,
    resolve_wikimedia_source,
)


def project_root() -> Path:
    return Path(__file__).resolve().parents[1]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(
        description="Fetch license-audited Wikimedia product photos for AirBattery."
    )
    parser.add_argument(
        "--only",
        action="append",
        default=[],
        metavar="SOURCE_ID",
        help="Fetch only one manifest source id; repeat for multiple ids.",
    )
    parser.add_argument(
        "--size",
        type=int,
        default=1024,
        help="Square WebP output size in pixels (default: 1024).",
    )
    parser.add_argument(
        "--refresh-cache",
        action="store_true",
        help="Redownload source photographs even when the cache already exists.",
    )
    return parser.parse_args()


def cache_name(source_id: str, resolved_url: str) -> str:
    suffix = Path(urlparse(resolved_url).path).suffix.lower()
    if suffix not in {".jpg", ".jpeg", ".png", ".webp", ".tif", ".tiff"}:
        suffix = ".image"
    return f"{source_id}{suffix}"


def main() -> int:
    args = parse_args()
    root = project_root()
    manifest = load_and_validate_manifest(root)
    sources = manifest["sources"]

    selected_ids = set(args.only)
    known_ids = {source["id"] for source in sources}
    unknown = selected_ids - known_ids
    if unknown:
        print(f"Unknown source id(s): {', '.join(sorted(unknown))}", file=sys.stderr)
        return 2
    if selected_ids:
        sources = [source for source in sources if source["id"] in selected_ids]

    output_dir = root / "apps/desktop/src/assets/device-artwork/exact"
    cache_dir = root / ".cache/exact-artwork"
    generated_path = root / "apps/desktop/src/domain/exact-artwork.generated.ts"
    attribution_path = output_dir / "ATTRIBUTION.generated.json"
    output_dir.mkdir(parents=True, exist_ok=True)
    cache_dir.mkdir(parents=True, exist_ok=True)

    staging = root / ".cache/exact-artwork-staging"
    shutil.rmtree(staging, ignore_errors=True)
    staging.mkdir(parents=True, exist_ok=True)

    records: list[dict[str, object]] = []
    completed: list[dict[str, object]] = []
    try:
        for source in sources:
            print(f"Resolving {source['id']} ({source['fileTitle']})")
            resolved = resolve_wikimedia_source(source)
            cached = cache_dir / cache_name(source["id"], str(resolved["resolvedUrl"]))
            if args.refresh_cache or not cached.exists():
                print(f"  downloading licensed original -> {cached}")
                download_file(str(resolved["resolvedUrl"]), cached)
            else:
                print(f"  using cached original -> {cached}")

            target = staging / source["outputName"]
            normalize_photo(cached, target, crop=source["crop"], size=args.size)
            completed.append(source)
            records.append(attribution_record(source, resolved))
            print(f"  generated {target.name}")

        if not selected_ids:
            for existing in output_dir.glob("*.webp"):
                existing.unlink()
        for generated in staging.glob("*.webp"):
            generated.replace(output_dir / generated.name)

        if selected_ids:
            # Partial fetches are useful for reviewing one model, but the generated
            # map must include only files that are actually present on disk.
            completed_names = {path.name for path in output_dir.glob("*.webp")}
            all_completed = [
                source for source in manifest["sources"] if source["outputName"] in completed_names
            ]
            prior_records: list[dict[str, object]] = []
            if attribution_path.exists():
                prior_data = json.loads(attribution_path.read_text(encoding="utf-8"))
                prior_records = [
                    item for item in prior_data.get("sources", [])
                    if item.get("id") not in selected_ids
                ]
            records = sorted(prior_records + records, key=lambda item: str(item["id"]))
            completed = all_completed

        generated_path.write_text(generate_typescript(completed), encoding="utf-8")
        attribution_path.write_text(
            json.dumps(
                {
                    "schemaVersion": 1,
                    "notice": (
                        "Exact product photographs are redistributed under the per-file "
                        "Creative Commons licenses listed below. AirBattery does not claim "
                        "ownership or endorsement by the depicted brands."
                    ),
                    "sources": sorted(records, key=lambda item: str(item["id"])),
                },
                indent=2,
                ensure_ascii=False,
            )
            + "\n",
            encoding="utf-8",
        )
    except Exception as error:  # noqa: BLE001 - CLI must preserve the last good generated map.
        print(f"Exact artwork fetch failed: {error}", file=sys.stderr)
        print("No staged files were installed; the existing fallback remains intact.", file=sys.stderr)
        return 1
    finally:
        shutil.rmtree(staging, ignore_errors=True)

    print(f"Fetched {len(completed)} exact artwork slot(s).")
    print(f"Generated imports: {generated_path}")
    print(f"Attribution: {attribution_path}")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
