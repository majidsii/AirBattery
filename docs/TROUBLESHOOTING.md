# Troubleshooting

## Bluetooth backend unavailable

Check the adapter and BlueZ as the normal desktop user:

```bash
bluetoothctl show
systemctl status bluetooth --no-pager
cargo run -p airbattery-cli -- status
```

AirBattery does not enable Bluetooth automatically. A missing adapter, powered-off adapter, permission failure, and BlueZ failure are separate states.

## No AirPods component values

AirPods values are advertisement-driven and may be intermittent. Open the case near the computer and run one bounded capture:

```bash
cargo run -p airbattery-cli -- scan --seconds 20 --json
```

Do not assume the case battery is live while the case is closed. A previous value must become stale according to its original timestamp.

## GNOME indicator shows unavailable

Confirm the desktop process owns the session-bus name:

```bash
gdbus call --session \
  --dest io.github.airbattery.Service \
  --object-path /io/github/airbattery/Service \
  --method io.github.airbattery.Service1.GetSnapshot
```

Then inspect the extension:

```bash
gnome-extensions info airbattery@airbattery.github.io
journalctl --user -b -o cat | grep -i airbattery
```

The extension is presentation-only. It needs the desktop application running with GNOME integration enabled.

## GNOME extension cannot be installed

Install `glib-compile-schemas` through the distribution package manager, then run:

```bash
./scripts/install-gnome-extension.sh
```

Do not run the extension installer with `sudo`. A logout/login may be required after adding a new extension.

## Desktop closes instead of staying in the tray

Open Settings and enable background operation and the tray icon. Desktop environments without AppIndicator support may not show the fallback tray; GNOME users should use the companion extension.

## Settings file is corrupt

AirBattery normalizes versioned settings and falls back safely. The application-controlled configuration path is reported by the diagnostics view. Preserve the file for debugging, then move it aside and restart the application. Do not delete unrelated configuration directories.

## Windows device has no component detail

Many Windows devices expose only an aggregate standard Battery Service or operating-system value. Left/right/case values are not promised unless reliable native advertisement data is implemented and validated for that device.

## Diagnostic sharing

Use the application’s sanitized diagnostic export. Review the file before sharing. Do not post raw Bluetooth captures, user directory paths, secrets, or signing material publicly.
