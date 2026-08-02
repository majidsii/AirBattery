#!/usr/bin/env python3
"""Generate deterministic Tauri raster icons from AirBattery's original SVG."""

from __future__ import annotations

import io
from pathlib import Path

import cairosvg
from PIL import Image

ROOT = Path(__file__).resolve().parents[1]
ICON_DIR = ROOT / "apps" / "desktop" / "src-tauri" / "icons"
SOURCE = ICON_DIR / "icon.svg"
SIZES = {
    "32x32.png": 32,
    "128x128.png": 128,
    "128x128@2x.png": 256,
    "icon.png": 512,
}


def render_png(size: int) -> Image.Image:
    data = cairosvg.svg2png(
        bytestring=SOURCE.read_bytes(),
        output_width=size,
        output_height=size,
    )
    with Image.open(io.BytesIO(data)) as image:
        return image.convert("RGBA")


def main() -> None:
    if not SOURCE.is_file():
        raise SystemExit(f"missing icon source: {SOURCE}")

    ICON_DIR.mkdir(parents=True, exist_ok=True)
    generated: list[Path] = []
    for filename, size in SIZES.items():
        output = ICON_DIR / filename
        render_png(size).save(output, format="PNG", optimize=True)
        generated.append(output)

    ico = ICON_DIR / "icon.ico"
    source = render_png(256)
    source.save(ico, format="ICO", sizes=[(16, 16), (24, 24), (32, 32), (48, 48), (64, 64), (128, 128), (256, 256)])
    generated.append(ico)

    for path in generated:
        print(path.relative_to(ROOT))


if __name__ == "__main__":
    main()
