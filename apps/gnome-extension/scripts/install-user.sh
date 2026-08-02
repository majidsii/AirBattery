#!/usr/bin/env bash
set -euo pipefail

EXTENSION_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
REPOSITORY_ROOT="$(cd "$EXTENSION_ROOT/../.." && pwd)"
exec "$REPOSITORY_ROOT/scripts/install-gnome-extension.sh" "$@"
