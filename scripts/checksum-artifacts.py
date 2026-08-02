#!/usr/bin/env python3
"""Create a deterministic SHA-256 manifest for concrete release artifacts."""

from __future__ import annotations

import argparse
import hashlib
import os
import sys
import tempfile
from pathlib import Path

DEFAULT_MANIFEST = "SHA256SUMS"
CHUNK_SIZE = 1024 * 1024


def sha256_file(path: Path) -> str:
    digest = hashlib.sha256()
    with path.open("rb") as source:
        while chunk := source.read(CHUNK_SIZE):
            digest.update(chunk)
    return digest.hexdigest()


def collect_artifacts(directory: Path, manifest: Path) -> list[Path]:
    artifacts: list[Path] = []
    for path in sorted(directory.rglob("*"), key=lambda item: item.relative_to(directory).as_posix()):
        if path.is_symlink():
            raise ValueError(f"symbolic link is not allowed: {path.relative_to(directory).as_posix()}")
        if not path.is_file() or path == manifest:
            continue
        artifacts.append(path)
    if not artifacts:
        raise ValueError("no release artifacts found")
    return artifacts


def write_manifest(directory: Path, manifest: Path) -> list[str]:
    artifacts = collect_artifacts(directory, manifest)
    lines = [
        f"{sha256_file(path)}  {path.relative_to(directory).as_posix()}"
        for path in artifacts
    ]
    manifest.parent.mkdir(parents=True, exist_ok=True)
    if manifest.is_symlink():
        raise ValueError("symbolic link is not allowed for checksum manifest")
    descriptor, temporary_name = tempfile.mkstemp(prefix=f".{manifest.name}.", dir=manifest.parent)
    temporary = Path(temporary_name)
    try:
        with os.fdopen(descriptor, "w", encoding="utf-8", newline="\n") as output:
            output.write("\n".join(lines) + "\n")
            output.flush()
            os.fsync(output.fileno())
        temporary.replace(manifest)
    finally:
        temporary.unlink(missing_ok=True)
    return lines


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("directory", type=Path, help="directory containing release artifacts")
    parser.add_argument(
        "--output",
        type=Path,
        default=None,
        help="manifest path; relative paths are resolved inside the artifact directory",
    )
    return parser.parse_args()


def main() -> int:
    arguments = parse_args()
    directory = arguments.directory.resolve()
    if not directory.is_dir():
        print(f"error: artifact directory does not exist: {directory}", file=sys.stderr)
        return 2

    output = arguments.output or Path(DEFAULT_MANIFEST)
    manifest = output if output.is_absolute() else directory / output
    manifest = manifest.resolve(strict=False)
    try:
        manifest.relative_to(directory)
    except ValueError:
        print("error: checksum manifest must be inside the artifact directory", file=sys.stderr)
        return 2

    try:
        lines = write_manifest(directory, manifest)
    except (OSError, ValueError) as error:
        print(f"error: {error}", file=sys.stderr)
        return 1

    print(f"wrote {manifest} with {len(lines)} entries")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
