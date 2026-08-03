# Hardware Validation

## Evidence rule

Every hardware result must name the exact Git commit, operating system, desktop session, device model/firmware, physical state, timestamp, and sanitized diagnostics file. A result from an older commit does not validate a newer battery or lifecycle change.

## Build and start on Ubuntu

```bash
cd ~/projects/personal/AirBattery

CARGO_INCREMENTAL=0 \
CARGO_PROFILE_DEV_DEBUG=0 \
CARGO_PROFILE_TEST_DEBUG=0 \
CARGO_BUILD_JOBS=2 \
./RUN_UBUNTU_VALIDATION.sh
```

For interactive development:

```bash
npm --prefix apps/desktop run tauri dev
```

For the release-mode executable:

```bash
npm --prefix apps/desktop run tauri build -- --no-bundle
./target/release/airbattery
```

Keep the application running as the normal desktop user. Never start it with `sudo`.

## AirPods Pro first-generation matrix

| Scenario | Expected truthful result |
|---|---|
| Bluetooth off | backend reports unavailable; exact accessory monitors stop; GNOME indicator hides |
| Bluetooth on, AirPods disconnected | no active-device indicator; last values may remain in history but are not live |
| Connect renamed AirPods such as `SHABIN` | passive Apple advertisement can establish identity; exact accessory monitor starts for the paired address |
| Keep both earbuds connected for one minute | exact left/right values continue updating without reopening the desktop window |
| Open case with earbuds inside | case appears only when a valid case record is received |
| Remove only left | left may update independently; right/case remain verified, stale, or unavailable |
| Remove only right | right may update independently; left/case remain verified, stale, or unavailable |
| Remove both | left/right remain separate; case may become unavailable without becoming `0%` |
| Return one earbud | only protocol-supported component and charging changes are shown |
| Receive byte `127` or `255` | value is treated as unavailable, never displayed as a percentage |
| BlueZ restart | old monitor loops stop, service recovers, one monitor is recreated per connected candidate |
| Suspend/resume | snapshots recover without duplicate monitors or duplicate status icons |
| Close/reopen window | background service, GNOME extension, and main window converge on one cached snapshot |

### Passive versus exact values

Apple proximity advertisements can be approximate and often use coarse steps. AirBattery marks those values as approximate. The Apple accessory channel can provide exact component percentages and is labelled verified only after a valid packet is parsed.

A temporarily unavailable case record must not erase a previous verified case value. The retained value keeps its original timestamp and eventually becomes stale according to the registry policy.

## Renamed-device correlation

AirPods can have a user alias that contains no Apple product name. AirBattery therefore does not rely only on strings such as `AirPods` or `Beats`.

The safe flow is:

1. collect known paired/connected BlueZ devices;
2. observe a valid Apple proximity-pairing advertisement;
3. correlate it only when there is one unambiguous paired Apple identity or one unambiguous connected audio candidate;
4. merge the advertisement evidence into that known device;
5. start the exact accessory monitor for the paired address;
6. retain that monitor while the address remains connected, even if a later short scan misses the rotating advertisement.

When multiple candidates are plausible, AirBattery refuses to guess.

## Diagnostics capture

```bash
cargo run -p airbattery-cli -- scan --seconds 30 --json \
  | tee "hardware-airpods-$(date +%Y%m%d-%H%M%S).json"
```

Create separate captures for connected, left-only, right-only, both-out, charging, Bluetooth toggle, and resume states. Exported diagnostics must not contain raw Bluetooth addresses or raw manufacturer payload bytes.

## GNOME Shell 50

```bash
./scripts/install-gnome-extension.sh
gnome-extensions enable airbattery@airbattery.github.io
gnome-extensions info airbattery@airbattery.github.io
journalctl --user -f -o cat | grep -i airbattery
```

Verify:

- no active supported Bluetooth device: no AirBattery panel indicator;
- one active device: one AirBattery panel indicator;
- GNOME integration enabled: no duplicate blue AppIndicator icon;
- AirPods exact data: separate `L`, `R`, and `C` slots;
- unavailable case: `C —`, never `C 0%`;
- menu actions open the Linux desktop application, not a Windows-specific surface.

## Generic-device matrix

Repeat connect, update, disconnect, Bluetooth toggle, suspend/resume, and stale-value checks for available devices:

- TWS earbuds with one aggregate value;
- TWS earbuds with split left/right or case data;
- over-ear headset;
- speaker;
- mouse;
- keyboard;
- controller;
- stylus.

Acceptance rules:

- one aggregate value is shown once;
- missing components show `—`;
- an exact pre-rendered asset is used only for an exact model-key match;
- otherwise the neutral category/family fallback is used;
- no AI lookalike or photo from a similar model is shown.

## Windows 10/11

When Windows support is released, verify the native NSIS build on both Windows 10 and Windows 11:

- native notification-area icon and tooltip;
- no GNOME/Linux surface code path;
- left click, Settings, Restart, and Quit;
- background mode after closing the main window;
- startup, Bluetooth toggle, sleep/resume, upgrade, and uninstall;
- standard Battery Service behavior without invented component values.
