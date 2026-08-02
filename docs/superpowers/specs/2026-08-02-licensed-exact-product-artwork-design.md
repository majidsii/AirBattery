# Licensed Exact Product Artwork Design

## Goal

Replace synthetic/stylized product renders with exact photographs of real devices only when the photograph has an auditable redistribution license. Models without an approved photograph must fall back to the existing original SVG family artwork rather than display an invented lookalike.

## Architecture

The desktop keeps the existing SVG family catalog as the always-available fallback. A generated exact-artwork module may override any individual artwork key and component mode (`single`, `pair`, or `case`) with a locally bundled WebP derived from a licensed source photograph.

A curated JSON source manifest records Wikimedia Commons file titles, expected license families, attribution, target artwork key, and mode. A Python importer queries the Wikimedia Commons API, validates license metadata against an allowlist, downloads the original image, normalizes orientation, crops it to a square photo card without altering product geometry, writes a high-quality WebP, writes a machine-readable attribution manifest, and generates TypeScript imports.

## Safety and licensing rules

- Never use AI-generated product imagery.
- Never scrape or bundle manufacturer product photographs unless an explicit redistribution license is recorded.
- Accept only CC0, CC BY 2.0/3.0/4.0, and CC BY-SA 3.0/4.0 sources in phase one.
- Preserve author, source page, source file, license URL, and modification notes.
- Treat each processed image as a derivative under the source image license.
- Do not claim exact coverage for a model unless the source manifest names that model.
- Missing exact assets use existing SVG artwork.

## UI behavior

- Exact photographs are rendered with `object-fit: contain`, rounded clipping, and no horizontal mirroring unless a mode-specific source explicitly permits it.
- A single-earbud exact photograph can be reused for left/right only when the manifest marks it mirror-safe; otherwise the fallback SVG is used for the missing side.
- Photos remain local after build; the packaged application does not contact Wikimedia at runtime.

## Initial curated coverage

Phase one includes auditable sources for selected current catalog keys where Wikimedia Commons has clear licenses, including AirPods Pro 1, Galaxy Buds Live, original Galaxy Buds, Nothing Ear (1), Pixel Buds Pro, Huawei FreeBuds 6, Xiaomi Buds 5, and Beats Studio Buds+.

Coverage for QCY, Soundcore earbuds, Sony WF, JBL earbuds, OnePlus Buds, AirPods Max, AirPods 4, and other uncovered models remains SVG until a redistributable exact photograph is approved.

## Testing

- Catalog tests prove SVG fallback remains present and no synthetic studio directory is referenced.
- Manifest tests prove every exact entry has source, author, license, mode, and model key.
- Generator tests use local fixture metadata and images; no network is required.
- Source verification rejects unapproved license IDs, missing attribution, or unrecognized artwork keys.
