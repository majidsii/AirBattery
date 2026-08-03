#!/usr/bin/env python3
"""Contract tests for the one-command Ubuntu validation entry point."""

from __future__ import annotations

import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
SCRIPT = ROOT / "RUN_UBUNTU_VALIDATION.sh"


class UbuntuValidationScriptTests(unittest.TestCase):
    def test_entry_point_runs_the_universal_runtime_validator(self) -> None:
        text = SCRIPT.read_text(encoding="utf-8")

        self.assertIn("./scripts/validate-universal-runtime.sh", text)

    def test_entry_point_builds_the_tauri_bundle_after_validation(self) -> None:
        text = SCRIPT.read_text(encoding="utf-8")

        self.assertIn("npm exec -- tauri build", text)
        self.assertIn('cd "$ROOT/apps/desktop"', text)

    def test_entry_point_installs_the_gnome_extension_for_the_current_user(self) -> None:
        text = SCRIPT.read_text(encoding="utf-8")

        self.assertIn("./scripts/install-gnome-extension.sh", text)
        self.assertIn("airbattery@airbattery.github.io", text)

    def test_entry_point_does_not_download_product_artwork_during_build(self) -> None:
        text = SCRIPT.read_text(encoding="utf-8")

        self.assertNotIn("fetch-exact-artwork.py", text)
        self.assertNotIn("AIRBATTERY_SKIP_EXACT_ARTWORK_FETCH", text)
        self.assertNotIn("python3-pil", text)


if __name__ == "__main__":
    unittest.main()
