#!/usr/bin/env bash
set -euo pipefail

python3 scripts/verify_reference_models.py
python3 scripts/verify-desktop-source.py
python3 scripts/verify-tauri-source.py
python3 scripts/verify-dbus-source.py
python3 scripts/verify-gnome-source.py
python3 scripts/verify-packaging-source.py
python3 -m unittest tests/release_artifacts_test.py

node --experimental-strip-types --test apps/desktop/tests/*.test.ts
TERM=dumb npm --prefix apps/desktop exec -- tsc --noEmit \
  --target ES2022 \
  --module ESNext \
  --moduleResolution Bundler \
  --strict \
  --allowImportingTsExtensions \
  apps/desktop/src/types/native-runtime.d.ts \
  apps/desktop/src/domain/*.ts \
  apps/desktop/src/api/backend.ts

node --test apps/gnome-extension/tests/*.test.mjs
node --check apps/gnome-extension/extension.js
node --check apps/gnome-extension/refresh-controller.js
node --check apps/gnome-extension/service.js
node --check apps/gnome-extension/prefs.js
bash -n scripts/install-gnome-extension.sh
bash -n scripts/package-gnome-extension.sh

if command -v rg >/dev/null 2>&1; then
  markers="$(rg -n 'TODO|FIXME|MOCK|PLACEHOLDER|unimplemented!|todo!|#\[ignore\]' \
    crates apps scripts README.md docs \
    --glob '!**/node_modules/**' \
    --glob '!**/target/**' \
    --glob '!**/dist/**' \
    --glob '!docs/superpowers/plans/**' \
    --glob '!scripts/verify-foundation.sh' \
    --glob '!scripts/verify-*.py' || true)"
else
  markers="$(grep -RInE 'TODO|FIXME|MOCK|PLACEHOLDER|unimplemented!|todo!|#\[ignore\]' \
    crates apps scripts README.md docs \
    --exclude-dir=node_modules \
    --exclude-dir=target \
    --exclude-dir=dist \
    --exclude-dir=plans \
    --exclude=verify-foundation.sh \
    --exclude='verify-*.py' || true)"
fi

if [[ -n "$markers" ]]; then
  echo "$markers" >&2
  echo "error: unfinished marker scan failed" >&2
  exit 1
fi

if ! command -v cargo >/dev/null 2>&1; then
  echo "error: Cargo is not available. Run ./scripts/bootstrap-rust.sh first." >&2
  exit 127
fi

cargo fmt --all -- --check
cargo clippy --workspace --all-targets -- -D warnings
cargo test --workspace
cargo build --workspace --release

echo "foundation verification complete"
