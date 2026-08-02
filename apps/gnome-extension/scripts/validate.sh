#!/usr/bin/env bash
set -euo pipefail

EXTENSION_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPOSITORY_ROOT="$(cd "$EXTENSION_ROOT/../.." && pwd)"
STAGING="$(mktemp -d)"
trap 'rm -rf "$STAGING"' EXIT

node --check "$EXTENSION_ROOT/extension.js"
node --check "$EXTENSION_ROOT/prefs.js"
node --check "$EXTENSION_ROOT/service.js"
node --check "$EXTENSION_ROOT/snapshot.js"
node --check "$EXTENSION_ROOT/refresh-controller.js"
node --test "$EXTENSION_ROOT/tests"/*.test.mjs
python3 "$REPOSITORY_ROOT/scripts/verify-dbus-source.py"
python3 "$REPOSITORY_ROOT/scripts/verify-gnome-source.py"

if ! command -v glib-compile-schemas >/dev/null 2>&1; then
  echo "glib-compile-schemas is required for the GNOME schema validation stage." >&2
  exit 127
fi
cp -a "$EXTENSION_ROOT/schemas" "$STAGING/schemas"
glib-compile-schemas --strict "$STAGING/schemas"
echo "GNOME extension validation completed."
