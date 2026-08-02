# Building AirBattery

## Required toolchains

- Rust 1.97 or newer compatible with the workspace `rust-version`;
- Node.js 22.12 or newer;
- npm;
- platform build dependencies required by Tauri 2;
- Linux: BlueZ development/runtime libraries, WebKitGTK 4.1, GTK 3, and AppIndicator support;
- GNOME extension packaging: `glib-compile-schemas`;
- Windows: a supported Visual Studio Build Tools installation and Windows SDK.

Use the normal desktop user. Do not build or run AirBattery as root.

## Bootstrap

```bash
cd /path/to/airbattery
./scripts/bootstrap-rust.sh
npm --prefix apps/desktop install
```

The repository does not vendor package-manager caches. A reachable crates.io/npm-compatible registry is required for the first dependency resolution.

## Verify

```bash
./scripts/verify-foundation.sh
npm --prefix apps/desktop run typecheck
npm --prefix apps/desktop run test:ui
```

Every non-zero result must be investigated. Do not treat an unavailable toolchain as a passed gate.

## Development application

```bash
npm --prefix apps/desktop run tauri dev
```

## Release application

Linux:

```bash
npm --prefix apps/desktop run tauri build -- --bundles deb,appimage
```

Windows, from a Windows runner:

```powershell
npm --prefix apps/desktop run tauri build -- --bundles nsis
```

Expected artifact directories are controlled by Tauri under `apps/desktop/src-tauri/target/release/bundle/`. Exact artifacts must be listed and checksummed after a successful build; no artifact is assumed to exist from configuration alone.

## GNOME companion

```bash
./scripts/package-gnome-extension.sh
```

See `docs/GNOME_EXTENSION.md` for the D-Bus contract and runtime validation.
