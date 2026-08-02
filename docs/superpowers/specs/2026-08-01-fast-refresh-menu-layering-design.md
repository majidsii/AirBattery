# Fast Bluetooth Refresh and Menu Layering Design

## Goal
Reduce perceived Bluetooth battery update latency from the current twelve-second discovery window to the shortest practical serialized cadence, while fixing navigation labels that render beneath the main content surface.

## Refresh design
Automatic refreshes use a two-second bounded BLE discovery window on a three-second scheduler. Refreshes remain serialized through the existing command path, so a new scan never overlaps an active scan. Diagnostic discovery remains thirty seconds because it is an explicit troubleshooting mode rather than the normal runtime path. Windows follows the same two-second normal discovery window for consistent behavior.

The scheduler interval is measured from completion-safe interval ticks. With a two-second scan and a three-second tick, the application can publish fresh observations roughly every three seconds when advertisements are available. If BlueZ takes longer than expected, missed ticks are skipped rather than queued.

## Menu layering design
The navigation rail becomes an explicit isolated stacking context above the content panel. The content panel receives a lower explicit layer, while tooltip labels receive a higher layer inside the navigation context. This avoids relying on DOM paint order, which currently lets the later `.app-content` backdrop-filter stacking context cover labels extending from the rail.

## Tests
Source-contract tests assert the exact refresh constants and required stacking rules. Existing Rust and frontend tests continue to cover data mapping and presentation behavior. Build/typecheck verifies the CSS remains accepted by the frontend toolchain.
