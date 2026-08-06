#!/usr/bin/env bash
set -euo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd -P)"
APP_ID="io.github.airbattery.airbattery"
DESKTOP_SOURCE="$ROOT/packaging/linux/$APP_ID.desktop"
ICON_SOURCE="$ROOT/packaging/linux/$APP_ID.svg"
DEBUG_BINARY="${AIRBATTERY_BIN:-$ROOT/target/debug/airbattery}"

APPLICATIONS_DIR="${XDG_DATA_HOME:-$HOME/.local/share}/applications"
ICONS_ROOT="${XDG_DATA_HOME:-$HOME/.local/share}/icons/hicolor"
DESKTOP_TARGET="$APPLICATIONS_DIR/$APP_ID.desktop"

for required in "$DESKTOP_SOURCE" "$ICON_SOURCE"; do
  if [[ ! -f "$required" ]]; then
    printf 'Missing required file: %s\n' "$required" >&2
    exit 1
  fi
done

install -d "$APPLICATIONS_DIR"
install -d "$ICONS_ROOT/scalable/apps"
install -d "$ICONS_ROOT/128x128/apps"
install -d "$ICONS_ROOT/256x256/apps"
install -d "$ICONS_ROOT/512x512/apps"

sed \
  -e "s|^Exec=.*$|Exec=\"$DEBUG_BINARY\"|" \
  -e "s|^Icon=.*$|Icon=$APP_ID|" \
  -e "s|^StartupWMClass=.*$|StartupWMClass=$APP_ID|" \
  "$DESKTOP_SOURCE" > "$DESKTOP_TARGET"
chmod 0644 "$DESKTOP_TARGET"

# Compatibility aliases override stale system launchers without adding duplicate
# menu entries. The canonical GTK app id remains the primary association.
for alias in AirBattery airbattery; do
  alias_target="$APPLICATIONS_DIR/$alias.desktop"
  sed \
    -e "s|^StartupWMClass=.*$|StartupWMClass=$alias|" \
    -e '/^X-GNOME-UsesNotifications=/a NoDisplay=true' \
    "$DESKTOP_TARGET" > "$alias_target"
  chmod 0644 "$alias_target"
done

install -m 0644 "$ICON_SOURCE" "$ICONS_ROOT/scalable/apps/$APP_ID.svg"
install -m 0644 "$ROOT/apps/desktop/src-tauri/icons/128x128.png" \
  "$ICONS_ROOT/128x128/apps/$APP_ID.png"
install -m 0644 "$ROOT/apps/desktop/src-tauri/icons/128x128@2x.png" \
  "$ICONS_ROOT/256x256/apps/$APP_ID.png"
install -m 0644 "$ROOT/apps/desktop/src-tauri/icons/icon.png" \
  "$ICONS_ROOT/512x512/apps/$APP_ID.png"

if command -v update-desktop-database >/dev/null 2>&1; then
  update-desktop-database "$APPLICATIONS_DIR" >/dev/null
fi

if command -v gtk-update-icon-cache >/dev/null 2>&1; then
  gtk-update-icon-cache -f -t "$ICONS_ROOT" >/dev/null 2>&1 || true
fi

printf 'Installed Linux desktop integration:\n'
printf '  Desktop ID: %s\n' "$APP_ID"
printf '  Launcher:   %s\n' "$DESKTOP_TARGET"
printf '  Icon:       %s\n' "$ICONS_ROOT/scalable/apps/$APP_ID.svg"
printf '\nClose every AirBattery window, rebuild, and start the application again.\n'
