#!/usr/bin/env bash
set -Eeuo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
LOG="$ROOT/airbattery-validation.log"
cd "$ROOT"

exec > >(tee "$LOG") 2>&1
trap 'status=$?; echo; echo "VALIDATION FAILED (exit $status) at line $LINENO: $BASH_COMMAND"; echo "Log saved to: $LOG"; exit $status' ERR

echo "== AirBattery Ubuntu validation =="
echo "Date: $(date --iso-8601=seconds)"
echo "Kernel: $(uname -a)"
if [[ -r /etc/os-release ]]; then
  cat /etc/os-release
fi

echo
echo "This script installs build/runtime prerequisites, Rust in your user account, npm dependencies, and the AirBattery GNOME extension for the current user."
echo "It does not unpair devices, change Bluetooth configuration, or install the desktop package system-wide."

echo "== Installing Ubuntu prerequisites =="
sudo apt-get update
sudo apt-get install -y \
  build-essential curl wget file pkg-config libssl-dev \
  libwebkit2gtk-4.1-dev libxdo-dev libayatana-appindicator3-dev librsvg2-dev \
  libbluetooth-dev bluez dbus-user-session libglib2.0-bin gjs unzip

echo "== Checking Node.js =="
node --version
npm --version
node -e 'const [a,b]=process.versions.node.split(".").map(Number); if (a<22 || (a===22 && b<12)) { console.error("Node.js 22.12 or newer is required."); process.exit(1); }'

echo "== Installing Rust =="
chmod +x scripts/*.sh apps/gnome-extension/scripts/*.sh
./scripts/bootstrap-rust.sh
# shellcheck disable=SC1090
source "${CARGO_HOME:-$HOME/.cargo}/env"
rustc --version
cargo --version

echo "== Installing frontend dependencies =="
npm --prefix apps/desktop install


echo "== Running universal runtime verification =="
./scripts/validate-universal-runtime.sh

echo "== Building Linux desktop bundles =="
cd "$ROOT/apps/desktop"
npm exec -- tauri build
cd "$ROOT"

echo "== Installing the GNOME extension for the current user =="
./scripts/install-gnome-extension.sh
GNOME_EXTENSION_UUID="airbattery@airbattery.github.io"
if command -v gnome-extensions >/dev/null 2>&1; then
  gnome-extensions info "$GNOME_EXTENSION_UUID" || true
fi

echo
echo "VALIDATION, BUILD, AND GNOME EXTENSION INSTALL COMPLETED"
echo "Linux bundles:"
find "$ROOT/target/release/bundle" -type f \
  \( -name "*.deb" -o -name "*.AppImage" -o -name "*.rpm" \) \
  -print 2>/dev/null || true
echo "Log saved to: $LOG"
