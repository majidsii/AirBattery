#!/usr/bin/env python3
"""Verify that AirBattery version declarations and an optional release tag agree."""

from __future__ import annotations

import argparse
import json
import sys
import tomllib
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]


def parse_args() -> argparse.Namespace:
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument(
        "--expected-tag",
        help="optional semantic tag that must equal v<workspace version>",
    )
    return parser.parse_args()


def read_json(path: Path) -> dict[str, object]:
    return json.loads(path.read_text(encoding="utf-8"))


def main() -> int:
    arguments = parse_args()
    errors: list[str] = []

    cargo = tomllib.loads((ROOT / "Cargo.toml").read_text(encoding="utf-8"))
    workspace_version = cargo.get("workspace", {}).get("package", {}).get("version")
    if not isinstance(workspace_version, str) or not workspace_version:
        errors.append("Cargo.toml workspace package version is missing")
        workspace_version = ""

    declarations = {
        "apps/desktop/package.json": read_json(ROOT / "apps/desktop/package.json").get("version"),
        "apps/desktop/src-tauri/tauri.conf.json": read_json(
            ROOT / "apps/desktop/src-tauri/tauri.conf.json"
        ).get("version"),
    }
    for label, value in declarations.items():
        if value != workspace_version:
            errors.append(f"{label} version {value!r} does not match {workspace_version!r}")

    changelog = (ROOT / "CHANGELOG.md").read_text(encoding="utf-8")
    if workspace_version and workspace_version not in changelog:
        errors.append(f"CHANGELOG.md does not mention {workspace_version}")

    if arguments.expected_tag is not None:
        expected = f"v{workspace_version}"
        if arguments.expected_tag != expected:
            errors.append(f"release tag {arguments.expected_tag!r} must equal {expected!r}")

    if errors:
        print(f"version sync checks: {len(errors)} failed", file=sys.stderr)
        for error in errors:
            print(f"- {error}", file=sys.stderr)
        return 1

    print(f"version sync checks: 4 passed ({workspace_version})")
    return 0


if __name__ == "__main__":
    raise SystemExit(main())
