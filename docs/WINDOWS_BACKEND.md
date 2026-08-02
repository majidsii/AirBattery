# Windows Bluetooth Backend and System Tray

## Architecture

The Windows backend combines two independent sources during one serialized refresh:

1. paired `BluetoothLEDevice` records and the standard GATT Battery Service;
2. a bounded active `BluetoothLEAdvertisementWatcher` window for manufacturer advertisements.

Normal automatic advertisement windows last 2 seconds and run on the serialized 3-second desktop cadence. A user-initiated Diagnostics scan lasts 30 seconds. Known-device refreshes do not start a watcher. The application resolver chooses each battery component independently, so an overall Windows/GATT percentage never fills left, right, or case.

## AirPods on Windows

Apple manufacturer data with company id `0x004C` is routed through the same clean-room AirPods proximity-pairing parser used on Linux. When a valid type-`0x07` payload is observed, it can provide:

- left earbud battery and charging state;
- right earbud battery and charging state;
- charging-case battery and charging state;
- a protocol-evidence model label.

Advertisements are unauthenticated and intermittent. The case may need to be opened near the computer after scanning starts. Exact Bluetooth-address matches are preferred. A rotating continuity address is associated with a paired device only when Windows reports one unambiguous connected candidate; otherwise the advertisement stays separate. Missing values remain `—`; AirBattery does not fabricate them.

## Generic devices

The backend also preserves:

- the paired device name and connection state;
- standard Battery Service values;
- advertised local name and service UUIDs;
- manufacturer data for registered protocol providers.

This supports aggregate batteries for headsets, earbuds, speakers, mice, keyboards, controllers, styluses, and other BLE devices when Windows or the device exposes a usable value. Vendor-specific split values require a reliable provider for that vendor protocol.

## System tray

The Tauri notification-area icon is synchronized after every normalized snapshot:

- one aggregate value: the icon renders that percentage;
- split earbuds: the icon renders the lower fresh left/right value;
- case battery is shown in the tooltip but does not replace the listening-device percentage;
- tooltip format supports `L 82%  R 79%  C 64%`;
- missing or disconnected data replaces an old percentage icon with an unavailable dash;
- left click opens AirBattery;
- the context menu provides Open, Settings, Restart, and Quit.

Windows may initially place the icon under hidden tray icons. Pinning is controlled by Windows, not by the application.

## Current verification status

The mapping and icon logic have target-independent tests. A native Windows build and physical Bluetooth validation are still required before release acceptance.

Run on Windows 10/11 with the MSVC toolchain:

```powershell
cargo fmt --all -- --check
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
cargo check -p bluetooth-windows --target x86_64-pc-windows-msvc
npm --prefix apps/desktop install
npm --prefix apps/desktop run typecheck
npm --prefix apps/desktop run build
npm --prefix apps/desktop run tauri build -- --bundles nsis
```

Then execute the Windows scenarios in `docs/HARDWARE_TESTS.md`.
