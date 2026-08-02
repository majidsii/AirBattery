from __future__ import annotations

import importlib.util
import json
import tempfile
import unittest
from unittest.mock import patch
from pathlib import Path

MODULE_PATH = Path(__file__).resolve().parents[1] / "exact_artwork.py"
spec = importlib.util.spec_from_file_location("exact_artwork", MODULE_PATH)
assert spec and spec.loader
exact_artwork = importlib.util.module_from_spec(spec)
spec.loader.exec_module(exact_artwork)


class ExactArtworkManifestTests(unittest.TestCase):
    def valid_manifest(self) -> dict:
        return {
            "schemaVersion": 1,
            "sources": [
                {
                    "id": "airpods-pro-1-pair",
                    "artworkKey": "airpods-pro-1",
                    "mode": "pair",
                    "provider": "wikimedia-commons",
                    "fileTitle": "File:AirPods Pro 1.jpg",
                    "sourcePage": "https://commons.wikimedia.org/wiki/File:AirPods_Pro_1.jpg",
                    "author": "Aconcagua",
                    "license": "CC-BY-SA-4.0",
                    "licenseUrl": "https://creativecommons.org/licenses/by-sa/4.0/",
                    "mirrorSafe": False,
                    "crop": [0.0, 0.0, 1.0, 1.0],
                }
            ],
        }

    def test_valid_manifest_is_normalized(self) -> None:
        manifest = exact_artwork.validate_manifest(
            self.valid_manifest(), {"airpods-pro-1", "nothing-ear"}
        )
        self.assertEqual(manifest["sources"][0]["mode"], "pair")
        self.assertEqual(manifest["sources"][0]["outputName"], "airpods-pro-1-pair.webp")

    def test_unapproved_license_is_rejected(self) -> None:
        manifest = self.valid_manifest()
        manifest["sources"][0]["license"] = "ALL-RIGHTS-RESERVED"
        with self.assertRaisesRegex(ValueError, "approved Creative Commons"):
            exact_artwork.validate_manifest(manifest, {"airpods-pro-1"})

    def test_unknown_artwork_key_is_rejected(self) -> None:
        manifest = self.valid_manifest()
        manifest["sources"][0]["artworkKey"] = "invented-product"
        with self.assertRaisesRegex(ValueError, "unknown artwork key"):
            exact_artwork.validate_manifest(manifest, {"airpods-pro-1"})

    def test_duplicate_key_and_mode_is_rejected(self) -> None:
        manifest = self.valid_manifest()
        duplicate = dict(manifest["sources"][0])
        duplicate["id"] = "duplicate"
        manifest["sources"].append(duplicate)
        with self.assertRaisesRegex(ValueError, "duplicate exact artwork slot"):
            exact_artwork.validate_manifest(manifest, {"airpods-pro-1"})

    def test_typescript_generator_is_deterministic_and_per_mode(self) -> None:
        manifest = exact_artwork.validate_manifest(
            self.valid_manifest(), {"airpods-pro-1"}
        )
        generated = exact_artwork.generate_typescript(manifest["sources"])
        self.assertIn("ExactArtworkAssetMap", generated)
        self.assertIn("'airpods-pro-1'", generated)
        self.assertIn("pair:", generated)
        self.assertIn("mirrorSafe: false", generated)
        self.assertIn("airpods-pro-1-pair.webp", generated)

    def test_attribution_contains_modifications_and_source(self) -> None:
        source = exact_artwork.validate_manifest(
            self.valid_manifest(), {"airpods-pro-1"}
        )["sources"][0]
        record = exact_artwork.attribution_record(
            source,
            {
                "resolvedUrl": "https://upload.wikimedia.org/example.jpg",
                "resolvedAuthor": "Aconcagua",
                "resolvedLicense": "CC-BY-SA-4.0",
            },
        )
        self.assertEqual(record["sourcePage"], source["sourcePage"])
        self.assertIn("crop", record["modifications"])
        self.assertIn("WebP conversion", record["modifications"])

    def test_checked_in_manifest_is_valid(self) -> None:
        root = Path(__file__).resolve().parents[2]
        manifest_data = json.loads(
            (root / "assets/exact-artwork/sources.json").read_text()
        )
        keys = exact_artwork.read_registered_artwork_keys(
            root / "apps/desktop/src/domain/artwork-keys.ts"
        )
        normalized = exact_artwork.validate_manifest(manifest_data, keys)
        self.assertGreaterEqual(len(normalized["sources"]), 8)
        providers = {item["provider"] for item in normalized["sources"]}
        self.assertEqual(providers, {"wikimedia-commons"})


class FakeHttpResponse:
    def __init__(self, payload: dict) -> None:
        self.payload = payload

    def __enter__(self) -> "FakeHttpResponse":
        return self

    def __exit__(self, *_args: object) -> None:
        return None

    def read(self, _size: int = -1) -> bytes:
        return json.dumps(self.payload).encode("utf-8")


class ExactArtworkWikimediaTests(unittest.TestCase):
    def source(self) -> dict:
        return exact_artwork.validate_manifest(
            ExactArtworkManifestTests().valid_manifest(), {"airpods-pro-1"}
        )["sources"][0]

    def payload(self, license_name: str = "CC BY-SA 4.0") -> dict:
        return {
            "query": {
                "pages": [
                    {
                        "title": "File:AirPods Pro 1.jpg",
                        "imageinfo": [
                            {
                                "url": "https://upload.wikimedia.org/example.jpg",
                                "mime": "image/jpeg",
                                "width": 3233,
                                "height": 2426,
                                "sha1": "abc123",
                                "extmetadata": {
                                    "LicenseShortName": {"value": license_name},
                                    "Artist": {"value": "<a>Aconcagua</a>"},
                                },
                            }
                        ],
                    }
                ]
            }
        }

    def test_resolver_accepts_matching_live_license(self) -> None:
        with patch.object(
            exact_artwork.urllib.request,
            "urlopen",
            return_value=FakeHttpResponse(self.payload()),
        ):
            resolved = exact_artwork.resolve_wikimedia_source(self.source())

        self.assertEqual(resolved["resolvedLicense"], "CC-BY-SA-4.0")
        self.assertEqual(resolved["resolvedAuthor"], "Aconcagua")
        self.assertEqual(resolved["resolvedUrl"], "https://upload.wikimedia.org/example.jpg")

    def test_resolver_accepts_wikimedia_cc0_short_name(self) -> None:
        manifest = ExactArtworkManifestTests().valid_manifest()
        manifest["sources"][0].update(
            {
                "id": "jbl-free-pair",
                "artworkKey": "jbl-free",
                "fileTitle": "File:JBL Free Truly Earphones.jpg",
                "sourcePage": "https://commons.wikimedia.org/wiki/File:JBL_Free_Truly_Earphones.jpg",
                "author": "Mangadidoes134",
                "license": "CC0-1.0",
                "licenseUrl": "https://creativecommons.org/publicdomain/zero/1.0/",
            }
        )
        source = exact_artwork.validate_manifest(manifest, {"jbl-free"})["sources"][0]

        with patch.object(
            exact_artwork.urllib.request,
            "urlopen",
            return_value=FakeHttpResponse(self.payload("CC0")),
        ):
            resolved = exact_artwork.resolve_wikimedia_source(source)

        self.assertEqual(resolved["resolvedLicense"], "CC0-1.0")

    def test_resolver_rejects_a_changed_live_license(self) -> None:
        with patch.object(
            exact_artwork.urllib.request,
            "urlopen",
            return_value=FakeHttpResponse(self.payload("CC BY 4.0")),
        ):
            with self.assertRaisesRegex(ValueError, "license changed"):
                exact_artwork.resolve_wikimedia_source(self.source())


class ExactArtworkImageTests(unittest.TestCase):
    def test_normalize_photo_writes_square_webp_without_stretching(self) -> None:
        from PIL import Image

        with tempfile.TemporaryDirectory() as tmp:
            tmp_path = Path(tmp)
            source = tmp_path / "source.png"
            output = tmp_path / "result.webp"
            Image.new("RGB", (800, 400), (230, 230, 230)).save(source)

            exact_artwork.normalize_photo(
                source,
                output,
                crop=[0.25, 0.0, 0.75, 1.0],
                size=512,
            )

            with Image.open(output) as result:
                self.assertEqual(result.size, (512, 512))
                self.assertEqual(result.format, "WEBP")


if __name__ == "__main__":
    unittest.main()
