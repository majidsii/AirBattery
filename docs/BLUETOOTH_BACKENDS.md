# Bluetooth Backends

## Linux / BlueZ

The Linux adapter uses `bluer` 0.17 over BlueZ. It reads:

- default-adapter availability;
- powered and discovering state;
- known device identifiers;
- device name;
- connection state;
- battery percentage exposed by BlueZ;
- manufacturer data keyed by Bluetooth SIG company id.

`scan_window` uses a bounded discovery stream with a fixed deadline. Dropping the stream releases the discovery session. AirBattery does not call `set_powered(true)` and does not modify system-wide adapter settings.

The mapping layer emits connection, platform battery, and manufacturer observations without manufacturer-specific inference. Protocol crates decide how to interpret payloads.

Implemented failure states include:

- BlueZ/system D-Bus unavailable;
- no default adapter;
- powered-off adapter;
- per-device property failure without aborting the complete catalog.

Linux work still required:

- long-lived adapter and device signal processing for normal background operation;
- adapter hot-plug and BlueZ service restart recovery;
- suspend/resume recovery;
- explicit GATT Battery Service reads when BlueZ does not expose a platform battery;
- runtime validation on Ubuntu 26.04, GNOME 50, and Wayland.

## Windows / WinRT

The Windows crate isolates supported Windows Runtime APIs behind the same `PlatformCollection` and `RawObservation` boundaries used by Linux. Authored source includes:

- Bluetooth radio availability and state;
- paired BLE device enumeration;
- connection-state mapping;
- standard Battery Service `0x180F` and Battery Level characteristic `0x2A19` reads;
- typed per-device failure handling.

Manufacturer-specific AirPods component data is not claimed on Windows unless the native APIs expose reliable advertisement data and the implementation is validated. The Windows crate and installer configuration are not compiled or runtime-tested in the current environment.

## Source selection

BlueZ platform battery, standard GATT battery, AirPods advertisements, Windows platform battery, and generic aggregate values remain distinct through `DataSource`. Resolution happens per component so an aggregate value does not erase fresher left, right, or case evidence.

When source quality is equal, fresher evidence wins. Older stale reports cannot overwrite fresh component data. A value that becomes unavailable remains unavailable; it is never replaced with zero.
