# AirPods exact battery validation on Ubuntu

This build has two intentionally different battery paths:

- **Passive BLE advertisement:** available when the case wakes and advertises. AirPods encode these values in 10% steps, so the UI prefixes them with `≈`, for example `≈70%`.
- **Active Apple accessory channel:** available while the paired AirPods are connected to Ubuntu. This path preserves one-percent values, for example `71%`, `68%`, and `12%`.

The UI never presents passive decile values as exact percentages.

## Build and install

```bash
cargo fmt --all
CARGO_INCREMENTAL=0 \
CARGO_PROFILE_DEV_DEBUG=0 \
CARGO_PROFILE_TEST_DEBUG=0 \
CARGO_BUILD_JOBS=2 \
./RUN_UBUNTU_VALIDATION.sh
```

## Run with exact-battery diagnostics

Close every running AirBattery process, connect `My AirPods` to Ubuntu, and run:

```bash
pkill -f '/airbattery($| )' 2>/dev/null || true

RUST_LOG='bluetooth_linux=debug,airbattery_desktop=info' \
./target/release/airbattery 2>&1 | tee airpods-exact-battery.log
```

Keep both earbuds connected for at least 20 seconds.

A successful active packet produces this debug message:

```text
received exact Apple accessory battery packet
```

Expected display behavior:

- plain `71%`, `68%`, `12%`: exact active accessory packet
- `≈70%`, `≈70%`, `≈10%`: passive BLE advertisement only
- no repeated `payload is truncated: expected at least 27 bytes, got 19` warning

When the case is closed or the AirPods are not connected to Ubuntu, exact active values might be unavailable; the app then falls back to clearly marked approximate BLE values instead of inventing precision.
