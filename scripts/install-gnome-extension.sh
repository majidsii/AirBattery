#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
SOURCE="$ROOT/apps/gnome-extension"
UUID="airbattery@airbattery.github.io"
DESTINATION="${XDG_DATA_HOME:-$HOME/.local/share}/gnome-shell/extensions/$UUID"
STAGING="$(mktemp -d)"
trap 'rm -rf "$STAGING"' EXIT

for entry in extension.js prefs.js metadata.json service.js snapshot.js refresh-controller.js stylesheet.css icons schemas; do
  cp -a "$SOURCE/$entry" "$STAGING/"
done

if ! command -v glib-compile-schemas >/dev/null 2>&1; then
  echo "glib-compile-schemas is required to install the AirBattery GNOME extension." >&2
  exit 1
fi
glib-compile-schemas "$STAGING/schemas"

rm -rf "$DESTINATION"
mkdir -p "$(dirname "$DESTINATION")"
cp -a "$STAGING" "$DESTINATION"

echo "Installed AirBattery GNOME extension to: $DESTINATION"
if command -v gnome-extensions >/dev/null 2>&1; then
  gnome-extensions enable "$UUID" || {
    echo "The extension was installed but could not be enabled in this session." >&2
    echo "Log out and back in, then run: gnome-extensions enable '$UUID'" >&2
  }
else
  echo "Enable it after signing in to GNOME: gnome-extensions enable '$UUID'"
fi
