# GNOME Shell Integration

## Support status

The companion extension targets GNOME Shell 50, the primary desktop target for Ubuntu 26.04. Its source, preferences schema, installation scripts, JavaScript tests, and static contract checks are present. Runtime loading in GNOME Shell 50 is not verified in the current container because `gjs`, `gnome-shell`, `gnome-extensions`, and `glib-compile-schemas` are unavailable.

The extension UUID is:

```text
airbattery@airbattery.github.io
```

## Responsibility boundary

The extension is presentation-only. It does not scan Bluetooth, parse advertisements, store raw addresses, or implement AirPods protocol logic. The single-instance Tauri process owns the Bluetooth backends, normalization engine, settings repository, and freshness rules.

The extension receives privacy-safe, versioned JSON snapshots over the user session D-Bus.

## D-Bus contract

| Item | Value |
|---|---|
| Bus name | `io.github.airbattery.Service` |
| Object path | `/io/github/airbattery/Service` |
| Interface | `io.github.airbattery.Service1` |
| Snapshot schema | `1` |

Methods:

```text
GetSnapshot() -> snapshot_json: string
Refresh() -> snapshot_json: string
OpenSettings()
ShowMainWindow()
```

Signal:

```text
SnapshotChanged(snapshot_json: string)
```

`Refresh` performs one bounded native refresh in the desktop backend and returns the resulting complete snapshot. It is not a request acknowledgement. `SnapshotChanged` is emitted only by the desktop service after a complete snapshot is available.

Action requests use a bounded non-blocking queue and one-shot responses. A call fails explicitly when the queue is unavailable and is capped at ten seconds so a stalled backend cannot hold GNOME Shell indefinitely.

The canonical introspection contract is stored at:

```text
apps/gnome-extension/dbus/io.github.airbattery.Service1.xml
```

## Snapshot behavior

The JSON envelope contains:

- `schemaVersion`;
- RFC 3339 `generatedAt` when available;
- sanitized backend status;
- normalized privacy-safe devices;
- the preferred privacy-safe device id.

The extension rejects unsupported schema versions, malformed devices, blank ids/names, and percentages outside `0..=100`. Invalid input degrades to an unavailable state instead of crashing GNOME Shell.

The top bar shows one aggregate/headset percentage for single-battery devices. For split earbuds it shows `L`, `R`, and, when known or expected for AirPods, `C` without inventing missing values. The compact icon percentage logic still prefers the aggregate value or the lower fresh left/right earbud value; case battery is never substituted for the listening-device percentage. The popup lists each component and charging state separately.

## Event lifecycle

1. The extension creates a session-bus proxy when enabled.
2. It calls `GetSnapshot` once.
3. It subscribes to `SnapshotChanged`.
4. It watches the D-Bus owner and reloads after service restart.
5. It performs no interval polling.
6. Disabling the extension disconnects all signal handlers and destroys the indicator.

## Preferences

The local GNOME schema stores only:

- whether battery text is shown in the top bar;
- an optional AirBattery-generated preferred device id.

The desktop application remains the authority for Bluetooth data and global application settings.

## Development verification

Dependency-free checks:

```bash
node --test apps/gnome-extension/tests/snapshot.test.mjs
node --check apps/gnome-extension/extension.js
node --check apps/gnome-extension/service.js
node --check apps/gnome-extension/prefs.js
python3 scripts/verify-gnome-source.py
python3 scripts/verify-dbus-source.py
```

`RUN_UBUNTU_VALIDATION.sh` installs this extension into the current user's GNOME extension directory after validation and bundle creation. Manual runtime checks on GNOME 50:

```bash
./scripts/install-gnome-extension.sh
gnome-extensions info airbattery@airbattery.github.io
journalctl --user -f -o cat | grep -i airbattery
```

The installation script operates in the current user’s XDG data directory and must not be run with `sudo`.

## Packaging

Create the extension archive and checksum with:

```bash
./scripts/package-gnome-extension.sh
```

Expected output paths after a successful run:

```text
target/gnome-extension/airbattery@airbattery.github.io.zip
target/gnome-extension/airbattery@airbattery.github.io.zip.sha256
```

`glib-compile-schemas` is required. No archive is claimed to exist until the command succeeds.

## Runtime validation matrix

The following remain unverified until executed in a real GNOME 50 Wayland session:

- indicator load and unload;
- menu layout and live updates;
- service restart recovery;
- preferred-device selection;
- settings and main-window actions;
- extension install, enable, disable, upgrade, and removal;
- Shell lock/unlock and suspend/resume behavior.
