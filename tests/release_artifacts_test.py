#!/usr/bin/env python3
"""Behavior tests for AirBattery release artifact tooling."""

from __future__ import annotations

import hashlib
import json
import subprocess
import sys
import tempfile
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[1]
CHECKSUMS = ROOT / "scripts" / "checksum-artifacts.py"
VERIFY = ROOT / "scripts" / "verify-release-artifacts.py"


def run_script(script: Path, *arguments: str) -> subprocess.CompletedProcess[str]:
    return subprocess.run(
        [sys.executable, str(script), *arguments],
        cwd=ROOT,
        check=False,
        capture_output=True,
        text=True,
    )


class ChecksumArtifactsTests(unittest.TestCase):
    def test_manifest_is_sorted_and_reproducible(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            (directory / "z.AppImage").write_bytes(b"zeta")
            (directory / "a.deb").write_bytes(b"alpha")

            first = run_script(CHECKSUMS, str(directory))
            self.assertEqual(first.returncode, 0, first.stderr)
            manifest = directory / "SHA256SUMS"
            first_bytes = manifest.read_bytes()

            second = run_script(CHECKSUMS, str(directory))
            self.assertEqual(second.returncode, 0, second.stderr)
            self.assertEqual(manifest.read_bytes(), first_bytes)

            expected = [
                f"{hashlib.sha256(b'alpha').hexdigest()}  a.deb",
                f"{hashlib.sha256(b'zeta').hexdigest()}  z.AppImage",
            ]
            self.assertEqual(manifest.read_text(encoding="utf-8").splitlines(), expected)

    def test_empty_directory_is_rejected_without_manifest(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            result = run_script(CHECKSUMS, str(directory))

            self.assertNotEqual(result.returncode, 0)
            self.assertIn("no release artifacts", result.stderr.lower())
            self.assertFalse((directory / "SHA256SUMS").exists())

    def test_symlink_is_rejected_without_hashing_target(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            target = directory / "real.deb"
            target.write_bytes(b"artifact")
            (directory / "linked.deb").symlink_to(target)

            result = run_script(CHECKSUMS, str(directory))

            self.assertNotEqual(result.returncode, 0)
            self.assertIn("symbolic link", result.stderr.lower())
            self.assertFalse((directory / "SHA256SUMS").exists())


class VerifyReleaseArtifactsTests(unittest.TestCase):
    def test_linux_validation_reports_every_missing_bundle_as_json(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            result = run_script(VERIFY, temporary, "--platform", "linux", "--json")

            self.assertNotEqual(result.returncode, 0)
            payload = json.loads(result.stdout)
            self.assertEqual(payload["status"], "error")
            self.assertEqual(
                set(payload["missing"]),
                {"debian-package", "appimage", "gnome-extension"},
            )

    def test_linux_validation_ignores_unrelated_native_packages(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            (directory / "unrelated.deb").write_bytes(b"deb")
            (directory / "unrelated.AppImage").write_bytes(b"appimage")

            result = run_script(VERIFY, temporary, "--platform", "linux", "--json")

            self.assertNotEqual(result.returncode, 0)
            payload = json.loads(result.stdout)
            self.assertEqual(
                set(payload["missing"]),
                {"debian-package", "appimage", "gnome-extension"},
            )

    def test_linux_validation_accepts_concrete_required_artifacts(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            (directory / "AirBattery_0.1.0-alpha.1_amd64.deb").write_bytes(b"deb")
            (directory / "AirBattery_0.1.0-alpha.1_amd64.AppImage").write_bytes(b"appimage")
            (directory / "airbattery@airbattery.github.io.zip").write_bytes(b"gnome")

            result = run_script(VERIFY, temporary, "--platform", "linux", "--json")

            self.assertEqual(result.returncode, 0, result.stderr)
            payload = json.loads(result.stdout)
            self.assertEqual(payload["status"], "ok")
            self.assertEqual(payload["platform"], "linux")
            self.assertEqual(len(payload["artifacts"]), 3)

    def test_windows_validation_requires_a_real_installer_executable(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            (directory / "notes.txt").write_text("not an installer", encoding="utf-8")
            (directory / "unrelated-setup.exe").write_bytes(b"not-airbattery")

            missing = run_script(VERIFY, temporary, "--platform", "windows", "--json")
            self.assertNotEqual(missing.returncode, 0)
            self.assertEqual(json.loads(missing.stdout)["missing"], ["windows-installer"])

            (directory / "AirBattery_0.1.0-alpha.1_x64-setup.exe").write_bytes(b"nsis")
            present = run_script(VERIFY, temporary, "--platform", "windows", "--json")
            self.assertEqual(present.returncode, 0, present.stderr)

    def test_checksum_manifest_must_cover_every_file_in_release_directory(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            installer = directory / "AirBattery_0.1.0-alpha.1_x64-setup.exe"
            installer.write_bytes(b"nsis")
            (directory / "release-notes.txt").write_text("notes", encoding="utf-8")
            (directory / "SHA256SUMS").write_text(
                f"{hashlib.sha256(b'nsis').hexdigest()}  {installer.name}\n",
                encoding="utf-8",
            )

            result = run_script(
                VERIFY,
                temporary,
                "--platform",
                "windows",
                "--checksums",
                "SHA256SUMS",
                "--json",
            )

            self.assertNotEqual(result.returncode, 0)
            payload = json.loads(result.stdout)
            self.assertIn("checksum-entry-missing", payload["errors"])

    def test_checksum_manifest_must_match_every_selected_artifact(self) -> None:
        with tempfile.TemporaryDirectory() as temporary:
            directory = Path(temporary)
            (directory / "AirBattery_0.1.0-alpha.1_x64-setup.exe").write_bytes(b"nsis")
            (directory / "SHA256SUMS").write_text(
                f"{'0' * 64}  AirBattery_0.1.0-alpha.1_x64-setup.exe\n",
                encoding="utf-8",
            )

            result = run_script(
                VERIFY,
                temporary,
                "--platform",
                "windows",
                "--checksums",
                "SHA256SUMS",
                "--json",
            )

            self.assertNotEqual(result.returncode, 0)
            payload = json.loads(result.stdout)
            self.assertIn("checksum-mismatch", payload["errors"])


if __name__ == "__main__":
    unittest.main()
