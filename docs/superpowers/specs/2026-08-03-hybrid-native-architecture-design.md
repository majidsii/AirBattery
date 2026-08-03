# AirBattery Hybrid Native Architecture

## Goal

Make AirBattery behave like a mature accessory application: one long-lived native service owns Bluetooth and protocol state, every UI surface consumes normalized snapshots, and product artwork is exact or intentionally generic—never guessed.

## Core architecture

AirBattery uses a service-first design. The Tauri process owns the application service on desktop platforms. Linux Bluetooth collection feeds provider-specific observations into one normalized registry. The desktop UI, GNOME extension, tray, widget, and diagnostics read the same cached snapshots and do not open competing Bluetooth sessions.

## Battery contract

- Valid percentages are integers from 0 through 100.
- Protocol sentinel values such as 127 and 255 are unavailable, not battery percentages.
- A temporarily unavailable component never erases a previous valid component.
- Previous valid values remain visible as last-known until the freshness policy expires them.
- Connection state and battery evidence are independent. Fresh verified accessory evidence may keep an AirPods device active while BlueZ connection state catches up.
- Case battery is optional. The UI displays an em dash when the service has never observed a valid case value.

## Provider priority

1. Active vendor protocol over the native transport.
2. Standard GATT Battery Service.
3. Platform battery properties from BlueZ, UPower, Windows, or macOS.
4. Passive manufacturer advertisements.
5. Cached last-known normalized values.

A lower-quality observation cannot overwrite a newer verified component with an unavailable value.

## Linux monitor lifecycle

The Apple accessory monitor is keyed by normalized Bluetooth address. Connected Apple audio candidates start or retain one background L2CAP monitor. Disconnected or removed candidates stop their monitor and release resources. The monitor retains the latest exact packet with its original observation time; freshness is decided by the registry rather than by deleting the packet in the transport layer.

## Native surfaces

- GNOME with integration enabled: GNOME extension only.
- Other Linux desktops: AppIndicator fallback.
- Windows: native Windows tray.
- macOS: native menu bar when implemented.

The GNOME indicator is hidden when the published snapshot has no active supported device. It reappears from a D-Bus change signal without running its own Bluetooth scan.

## Artwork contract

- Exact assets are allowed only when the exact model and license are known.
- Pre-rendered offline 3D is the preferred final format.
- Runtime WebGL/Three.js rendering is not required.
- AI-generated product approximations are prohibited as final assets.
- An asset for a similar model is never substituted for an exact model.
- When exact artwork is missing, the UI displays a neutral category silhouette.
- Left and right assets are independent when physical geometry is asymmetric.

## Delivery and Git history

Work is implemented on `feature/hybrid-native` in reviewable commits. Each behavior change receives regression tests first. The final Git bundle contains the original `main` history plus all new commits and can be fetched into the user's existing repository without replacing history or force-pushing.
