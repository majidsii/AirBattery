#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SOURCE="$ROOT/apps/gnome-extension"
UUID="airbattery@airbattery.github.io"
OUT_DIR="${1:-$ROOT/target/gnome-extension}"
STAGING="$(mktemp -d)"
trap 'rm -rf "$STAGING"' EXIT

for entry in extension.js prefs.js metadata.json service.js snapshot.js refresh-controller.js stylesheet.css icons schemas; do
  cp -a "$SOURCE/$entry" "$STAGING/"
done

if ! command -v glib-compile-schemas >/dev/null 2>&1; then
  echo "glib-compile-schemas is required to package the AirBattery GNOME extension." >&2
  exit 1
fi
glib-compile-schemas "$STAGING/schemas"
mkdir -p "$OUT_DIR"
ARCHIVE="$OUT_DIR/$UUID.zip"

STAGING="$STAGING" ARCHIVE="$ARCHIVE" python3 - <<'PY'
import os
from pathlib import Path
from zipfile import ZIP_DEFLATED, ZipFile

staging = Path(os.environ["STAGING"])
archive = Path(os.environ["ARCHIVE"])
with ZipFile(archive, "w", ZIP_DEFLATED) as output:
    for path in sorted(staging.rglob("*")):
        if path.is_file():
            output.write(path, path.relative_to(staging))
PY
sha256sum "$ARCHIVE" > "$ARCHIVE.sha256"
echo "Created: $ARCHIVE"
echo "Checksum: $ARCHIVE.sha256"
