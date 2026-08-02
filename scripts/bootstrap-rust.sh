#!/usr/bin/env bash
set -euo pipefail

readonly TOOLCHAIN="1.97.1"

version_is_adequate() {
  command -v rustc >/dev/null 2>&1 \
    && rustc --version | awk -v minimum="1.97.0" '
      function normalized(value, parts) {
        split(value, parts, ".")
        return (parts[1] * 1000000) + (parts[2] * 1000) + parts[3]
      }
      $1 == "rustc" { exit normalized($2) < normalized(minimum) }
      END { if (NR == 0) exit 1 }
    '
}

if version_is_adequate && command -v cargo >/dev/null 2>&1; then
  echo "Rust toolchain already available: $(rustc --version)"
  exit 0
fi

if ! command -v rustup >/dev/null 2>&1; then
  tmp_dir="$(mktemp -d)"
  trap 'rm -rf "$tmp_dir"' EXIT
  installer="$tmp_dir/rustup-init.sh"
  echo "Downloading the official rustup installer..."
  curl --fail --location --proto '=https' --tlsv1.2 \
    https://sh.rustup.rs --output "$installer"
  sh "$installer" -y --profile minimal --default-toolchain "$TOOLCHAIN"
fi

# shellcheck disable=SC1091
source "${CARGO_HOME:-$HOME/.cargo}/env"
rustup toolchain install "$TOOLCHAIN" --profile minimal --component rustfmt --component clippy
rustup override set "$TOOLCHAIN"
rustc --version
cargo --version
