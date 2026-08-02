#!/usr/bin/env bash
set -Eeuo pipefail

ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
cd "$ROOT"

run() {
  printf '\n==> %s\n' "$*"
  "$@"
}

run node --version
run npm --prefix apps/desktop test
run npm --prefix apps/gnome-extension test
run node --check apps/gnome-extension/extension.js
run node --check apps/gnome-extension/refresh-controller.js
run node --check apps/gnome-extension/service.js
run node --check apps/gnome-extension/prefs.js
run python3 scripts/verify_reference_models.py
run python3 -m unittest scripts/tests/test_exact_artwork.py
run python3 scripts/verify-exact-artwork.py
run python3 scripts/verify-desktop-source.py
run python3 scripts/verify-tauri-source.py
run python3 scripts/verify-dbus-source.py
run python3 scripts/verify-gnome-source.py
run python3 scripts/verify-packaging-source.py
run python3 -m unittest tests/release_artifacts_test.py tests/ubuntu_validation_script_test.py

if [[ -x apps/desktop/node_modules/.bin/vue-tsc ]]; then
  run npm --prefix apps/desktop run typecheck
  run npm --prefix apps/desktop run build
else
  echo
  echo "SKIP: frontend dependencies are not installed. Run: npm --prefix apps/desktop install"
fi

if command -v cargo >/dev/null 2>&1; then
  run cargo fmt --all -- --check
  run cargo test --workspace
  run cargo clippy --workspace --all-targets -- -D warnings
else
  echo
  echo "SKIP: Rust toolchain is not installed. Run: ./scripts/bootstrap-rust.sh"
fi

echo
echo "Universal runtime source validation completed."
echo "Physical Bluetooth scenarios still require docs/HARDWARE_TESTS.md."
