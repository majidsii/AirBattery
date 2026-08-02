# Hardware Validation

## Status and evidence rule

A raw AirPods Pro 2020 capture from 2026-07-31 now verifies that Apple `0x004C` proximity-pairing advertisements change across lid-open, left-out, both-out, and both-returned transitions. Those captured payloads are regression fixtures. This is protocol evidence, not yet proof that the rebuilt desktop UI updates correctly; the application still needs to be built and exercised on the physical Ubuntu host.

## Build and start on Ubuntu

```bash
cd /path/to/AirBattery
source "$HOME/.cargo/env"
./scripts/validate-universal-runtime.sh

cd apps/desktop
npm exec -- tauri build
```

The one-command script also installs and attempts to enable the GNOME extension for the current user. Install the generated `.deb`, start AirBattery as the normal desktop user, and keep Diagnostics open. Do not run the application with `sudo`.

## AirPods Pro 2020 matrix

For each row, press **Run 30-second device scan first**, then perform the physical action near the Bluetooth adapter.

| Scenario | Expected truthful result |
|---|---|
| Closed case for 30 seconds | Last-known data may remain stale; no invented live case value |
| Open case, both earbuds inside | Left/right/case appear when the Apple advertisement is received |
| Remove only left | Left can update independently; right/case remain real or `—` |
| Remove only right | Right can update independently; left/case remain real or `—` |
| Remove both and connect | Both earbuds update; case may become unavailable after its advertisement expires |
| Return left only | Charging/inside-case transition updates only when protocol evidence changes |
| Return right only | Same rule for right |
| Charge case and earbuds | Charging flags are shown separately when advertised |
| Toggle Bluetooth | Backend recovers and automatic refresh resumes |
| Suspend/resume | Values refresh without manual normal-page refresh |
| Close/reopen app | tray, GNOME, and main window converge on one snapshot |

Compare against an Apple device only as a reference at the same moment. AirPods advertisements use 10% steps, so exact single-percent agreement is not expected from that source.

## Ubuntu command-line capture

```bash
cargo run -p airbattery-cli -- scan --seconds 30 --json \
  | tee "hardware-airpods-$(date +%Y%m%d-%H%M%S).json"
```

Open the case after the command starts. The sanitized JSON includes the parsed AirPods model, left/right/case values, charging flags, RSSI, category hints, and manufacturer payload lengths without exporting Bluetooth addresses or raw manufacturer bytes. Capture separate files for left-only, right-only, both-out, and charging states.

When Apple uses a rotating continuity address, AirBattery first looks for exactly one paired Apple audio identity. It uses the paired device's BlueZ Device ID vendor (`0x004C`) or an explicit AirPods/Beats name, so a valid advertisement can still attach to `SHABIN` while the lid is open and the audio profile is disconnected. If no paired Apple identity exists, it falls back to exactly one connected audio candidate. Multiple plausible Apple identities remain separate; AirBattery deliberately refuses to guess.

## Generic-device matrix

Repeat connection, automatic update, disconnect, sleep/resume, and stale-value checks for available devices:

- TWS earbuds with one aggregate value;
- TWS earbuds with split left/right or case data;
- over-ear headset;
- Bluetooth speaker;
- mouse;
- keyboard;
- controller;
- stylus or digital pen.

Acceptance rules:

- exact model artwork only when reliable model evidence exists;
- otherwise category artwork is shown;
- completely unknown devices use the Bluetooth fallback;
- one aggregate percentage is shown once and is never duplicated into components;
- missing component percentages display `—`;
- changes appear automatically without the normal-page refresh button.

## GNOME Shell 50

```bash
./scripts/install-gnome-extension.sh
gnome-extensions enable airbattery@airbattery.github.io
gnome-extensions info airbattery@airbattery.github.io
journalctl --user -f -o cat | grep -i airbattery
```

Verify the top bar shows either one real percentage or separate `L/R/C` values. Click the indicator and verify device name, connection state, component percentages, charging labels, and Open AirBattery.

## Windows 10/11

Install the NSIS package and verify:

- the notification-area icon displays the selected listening-device percentage;
- the tooltip shows device name and separate `L/R/C` values when available;
- left click opens AirBattery;
- context-menu Settings, Restart, and Quit work;
- closing the main window leaves the tray process running when background mode is enabled;
- startup, sleep/resume, Bluetooth toggle, upgrade, and uninstall work;
- the 30-second Diagnostics scan captures AirPods manufacturer advertisements.

## Acceptance record

For every test record the exact build commit, operating system, device model/firmware, physical state, timestamp, sanitized diagnostic report, trusted comparison value if available, and Pass/Fail/Blocked result.
