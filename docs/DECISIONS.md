# Architecture Decision Records

## ADR-001 — Tauri 2 + Rust + Vue 3

- Date: 2026-07-29
- Decision: Use Tauri 2, Rust, Vue 3, TypeScript, Vite, and Pinia.
- Alternatives: Electron; GTK-only Rust UI; Qt.
- Reason: Small runtime footprint, strong Rust integration, cross-platform packaging, accessible web UI, and maintainable native adapters.
- Tradeoffs: Linux WebKit and packaging dependencies; tray behavior differs across desktop environments.
- Affected components: Desktop application, packaging, CI.

## ADR-002 — Modular Rust workspace

- Date: 2026-07-29
- Decision: Isolate domain, protocols, platform adapters, diagnostics, and application shell into crates.
- Alternatives: One Tauri backend crate; standalone daemon from day one.
- Reason: Testability, platform isolation, and future macOS support.
- Tradeoffs: More manifests and explicit dependency management.
- Affected components: Entire repository.

## ADR-003 — BlueZ through `bluer`; D-Bus service through `zbus`

- Date: 2026-07-29
- Decision: Use `bluer` for Linux Bluetooth behavior and `zbus` for the app-owned session service.
- Alternatives: Raw D-Bus everywhere; deprecated command-line parsing; custom C bindings.
- Reason: Maintained typed BlueZ interface while retaining full control of the stable GNOME IPC contract.
- Tradeoffs: Two related D-Bus libraries in Linux builds.
- Affected components: `bluetooth-linux`, desktop backend, GNOME extension.

## ADR-004 — Component-wise deterministic provider resolution

- Date: 2026-07-29
- Decision: Merge reports per battery component using validity, non-stale freshness, confidence, provider priority, and timestamp.
- Alternatives: Last-write-wins; one provider per device.
- Reason: Devices expose partial and conflicting sources; component-level resolution prevents loss of case or aggregate information.
- Tradeoffs: More metadata and tests.
- Affected components: `device-protocols`, `airbattery-core`.

## ADR-005 — MIT license and clean protocol reimplementation

- Date: 2026-07-29
- Decision: License AirBattery under MIT and reimplement protocol facts without copying GPL code.
- Alternatives: GPL-3.0 project; dual license.
- Reason: Broad adoption and compatibility. The MIT `apple-ble` parser is an allowed research reference; GPL projects are cited only as research.
- Tradeoffs: Any future GPL code contribution must be clean-room rewritten or the licensing decision revisited.
- Affected components: Repository, protocol crates, attribution.

## ADR-006 — Persistent normalization engine across native refreshes

- Date: 2026-07-29
- Decision: Keep one `ApplicationEngine` in Tauri-managed state and apply each platform collection transactionally.
- Alternatives: Rebuild the provider engine on every scan; store only the latest UI list.
- Reason: AirPods case advertisements are intermittent. Rebuilding on every scan would discard valid last-known case data before the freshness policy can mark it stale.
- Tradeoffs: Runtime state requires async synchronization and explicit lifecycle tests.
- Affected components: Tauri state, Linux/Windows collection adapters, diagnostics, GNOME D-Bus service.

## ADR-007 — Narrow Tauri frontend capability

- Date: 2026-07-29
- Decision: Grant bundled webviews only core event listen/unlisten permissions and expose Bluetooth, settings, diagnostics, autostart, and window actions through validated Rust commands.
- Alternatives: `core:default`; filesystem/store/shell plugins directly available to the frontend.
- Reason: A compromised webview should not gain arbitrary filesystem or process access. The frontend does not need direct native plugin permissions.
- Tradeoffs: More Rust command code and explicit event contracts.
- Affected components: Tauri capabilities, backend bridge, security documentation.

## ADR-008 — GNOME 50 is the primary Shell extension target

- Date: 2026-07-29
- Decision: Author the first extension release for GNOME Shell 50, the desktop shipped by Ubuntu 26.04 LTS, using ESM imports and a session D-Bus proxy.
- Alternatives: Legacy imports; broad multi-version metadata without validation; tray-only integration.
- Reason: The real target environment is Ubuntu 26.04. Declaring untested older/future Shell versions would create unsupported compatibility claims.
- Tradeoffs: Additional branches may be needed later for older GNOME releases.
- Affected components: GNOME extension metadata, extension code, installation and test documentation.

## ADR-009 — Versioned JSON snapshots over a bounded session D-Bus bridge

- Date: 2026-07-29
- Decision: Own `io.github.airbattery.Service` at `/io/github/airbattery/Service` with interface `io.github.airbattery.Service1`; transport schema-versioned JSON snapshots; route actions through a bounded Tokio queue using non-blocking enqueue, one-shot responses, and a ten-second response timeout.
- Alternatives: Strongly typed nested D-Bus structs; continuous polling from GJS; parsing BlueZ/AirPods data in the GNOME extension; filesystem polling; a second system-wide daemon.
- Reason: The desktop process already owns normalized state and settings. JSON permits controlled model evolution while a schema version enables strict rejection of incompatible payloads. Bounded non-blocking requests prevent Shell actions from accumulating, and the response timeout prevents an unavailable backend from holding a D-Bus call indefinitely.
- Tradeoffs: JSON loses compile-time field typing on the JavaScript side and therefore requires strict runtime validation. The desktop application must be running for live data, and callers receive an explicit error when the queue is full or a request exceeds ten seconds.
- Affected components: `airbattery-dbus`, Tauri lifecycle, GNOME extension, diagnostics, compatibility tests.
