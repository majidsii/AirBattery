# Exact product photographs

AirBattery uses real model-specific photographs only when the file has an auditable redistribution license. Exact photographs are build-time assets; the packaged application performs no artwork network requests.

## Source of truth

`assets/exact-artwork/sources.json` records the exact model key, component mode, Wikimedia Commons file title, source page, author, expected Creative Commons license, crop, and mirroring permission. `assets/exact-artwork/audit.json` records approved and pending coverage by brand.

The importer accepts only:

- CC0 1.0
- CC BY 2.0, 3.0, or 4.0
- CC BY-SA 3.0 or 4.0

The importer rechecks live Wikimedia metadata before installing a file. A missing file, changed license, unsupported license, unknown artwork key, duplicate model/mode slot, or malformed crop stops the import without replacing the last valid generated map.

## Build workflow

```bash
python3 scripts/fetch-exact-artwork.py
python3 scripts/verify-exact-artwork.py
npm --prefix apps/desktop run build
```

The importer:

1. Resolves the original file through the Wikimedia Commons API.
2. Verifies the current license against the checked-in expectation.
3. Downloads the original to `.cache/exact-artwork/`.
4. Applies only orientation correction, audited crop, proportional resize, and WebP conversion.
5. Writes local assets under `apps/desktop/src/assets/device-artwork/exact/`.
6. Generates `exact-artwork.generated.ts` and `ATTRIBUTION.generated.json`.

## Runtime selection

The desktop selects an exact photo only for the exact model key and requested mode. It never substitutes a nearby model. Missing modes and unapproved models use the existing original SVG family artwork.

## Licensing notice

The project code license does not replace the license of a photograph. Each generated photograph remains under the license listed in `ATTRIBUTION.generated.json`. Attribution includes author, source page, license URL, and the processing steps performed by AirBattery.
