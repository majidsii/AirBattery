#!/usr/bin/env python3
"""Validate that concrete AirBattery release artifacts exist and match checksums."""

from __future__ import annotations

import argparse
import hashlib
import json
import re
import sys
from pathlib import Path
from typing import Callable

CHUNK_SIZE = 1024 * 1024
CHECKSUM_LINE = re.compile(r"^([0-9a-f]{64})  ([^\r\n]+)$")
GNOME_ARCHIVE = "airbattery@airbattery.github.io.zip"


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        while chunk := source.read(CHUNK_SIZE):
            digest.update(chunk)
    return digest.hexdigest()


def artifact_files(directory: Path) -> list[Path]:
    files: list[Path] = []
    for path in sorted(directory.rglob("*"), key=lambda item: item.relative_to(directory).as_posix()):
        if path.is_symlink():
            raise ValueError(f"symbolic-link:{path.relative_to(directory).as_posix()}")
        if path.is_file() and path.name != "SHA256SUMS":
            files.append(path)
    return files


def requirements(platform: str) -> list[tuple[str, Callable[[Path], bool]]]:
    linux = [
        ("debian-package", lambda path: path.name.lower().startswith("airbattery") and path.name.endswith(".deb")),
        ("appimage", lambda path: path.name.lower().startswith("airbattery") and path.name.endswith(".AppImage")),
        ("gnome-extension", lambda path: path.name == GNOME_ARCHIVE),
    ]
    windows = [
        (
            "windows-installer",
            lambda path: path.name.lower().startswith("airbattery") and path.name.lower().endswith("-setup.exe"),
        )
    ]
    if platform == "linux":
        return linux
    if platform == "windows":
        return windows
    return linux + windows


def read_checksums(directory: Path, manifest_name: str) -> tuple[dict[str, str], list[str]]:
    manifest = Path(manifest_name)
    manifest = manifest if manifest.is_absolute() else directory / manifest
    if manifest.is_symlink():
        return {}, ["checksum-manifest-symbolic-link"]
    if not manifest.is_file():
        return {}, ["checksum-manifest-missing"]

    entries: dict[str, str] = {}
    errors: list[str] = []
    for line_number, line in enumerate(manifest.read_text(encoding="utf-8").splitlines(), start=1):
        match = CHECKSUM_LINE.fullmatch(line)
        if not match:
            errors.append(f"checksum-manifest-invalid-line:{line_number}")
            continue
        digest, relative_name = match.groups()
        path = Path(relative_name)
        if path.is_absolute() or ".." in path.parts:
            errors.append(f"checksum-manifest-unsafe-path:{line_number}")
            continue
        entries[path.as_posix()] = digest
    return entries, errors


def payload_for(directory: Path, platform: str, checksum_manifest: str | None) -> dict[str, object]:
    missing: list[str] = []
    errors: list[str] = []
    selected: set[Path] = set()

    try:
        files = artifact_files(directory)
    except ValueError as error:
        files = []
        errors.append(str(error))

    for label, predicate in requirements(platform):
        matches = [path for path in files if predicate(path) and path.stat().st_size > 0]
        if not matches:
            missing.append(label)
        selected.update(matches)

    selected_names = sorted(path.relative_to(directory).as_posix() for path in selected)
    if checksum_manifest is not None:
        checksums, checksum_errors = read_checksums(directory, checksum_manifest)
        errors.extend(checksum_errors)
        manifest_path = Path(checksum_manifest)
        manifest_path = manifest_path if manifest_path.is_absolute() else directory / manifest_path
        checksum_targets = [path for path in files if path.resolve() != manifest_path.resolve()]
        target_names = {path.relative_to(directory).as_posix() for path in checksum_targets}
        for path in checksum_targets:
            relative_name = path.relative_to(directory).as_posix()
            expected = checksums.get(relative_name)
            actual = sha256_file(path)
            if expected is None:
                errors.append("checksum-entry-missing")
            elif expected != actual:
                errors.append("checksum-mismatch")
        if set(checksums) - target_names:
            errors.append("checksum-entry-orphaned")

    unique_errors = list(dict.fromkeys(errors))
    return {
        "status": "ok" if not missing and not unique_errors else "error",
        "platform": platform,
        "artifacts": selected_names,
        "missing": missing,
        "errors": unique_errors,
    }


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory", type=Path, help="directory containing release artifacts")
    parser.add_argument("--platform", choices=("linux", "windows", "all"), required=True)
    parser.add_argument("--checksums", default=None, help="optional checksum manifest inside the directory")
    parser.add_argument("--json", action="store_true", help="emit one JSON object")
    return parser.parse_args()


def main() -> int:
    arguments = parse_args()
    directory = arguments.directory.resolve()
    if not directory.is_dir():
        payload = {
            "status": "error",
            "platform": arguments.platform,
            "artifacts": [],
            "missing": [],
            "errors": ["artifact-directory-missing"],
        }
    else:
        payload = payload_for(directory, arguments.platform, arguments.checksums)

    if arguments.json:
        print(json.dumps(payload, sort_keys=True, separators=(",", ":")))
    elif payload["status"] == "ok":
        print(f"release artifacts verified: {len(payload['artifacts'])} files")
    else:
        for missing in payload["missing"]:
            print(f"missing required artifact: {missing}", file=sys.stderr)
        for error in payload["errors"]:
            print(f"artifact validation error: {error}", file=sys.stderr)
    return 0 if payload["status"] == "ok" else 1


if __name__ == "__main__":
    raise SystemExit(main())
