# AirBattery Glass Phase 1 Design

## Goal

Introduce a premium, adjustable glass material system for the desktop main window, desktop widget, and future connection popup without changing Bluetooth collection, battery truthfulness, or native status logic.

## Product decisions

- Keep the existing Vue 3, Pinia, and authored CSS stack; add no UI framework or runtime dependency.
- Use five named presets: Off, Subtle, Balanced, Deep, and Crystal, plus Custom after manual changes.
- Provide a 0–100 master intensity and independent Main window, Desktop widget, and Connection popup controls.
- Persist glass-only preferences locally under `airbattery.glass.v1`; native application settings remain backward-compatible.
- Preserve exact/approximate/stale/unavailable semantics and never manufacture device or battery values.
- Use one primary glass material layer with restrained nested surfaces, specular edge highlights, blur, saturation, and theme-aware tint.
- Respect Reduce transparency, `prefers-reduced-transparency`, reduced motion, unsupported backdrop filters, dark/light/system themes, and keyboard navigation.

## Architecture

`src/domain/glass.ts` owns pure normalization, presets, CSS-variable calculation, and safe local persistence. The Pinia settings store owns runtime application and exposes mutation methods. `styles/glass.css` consumes CSS variables and assigns the correct material to main and widget window roles. `SettingsView.vue` exposes presets, sliders, accessibility switches, and a live material preview. `BatteryView.vue` receives the approved Overview hierarchy while keeping existing device presentation components and data flow.

## Scope boundary

Phase 1 does not add the actual connection-popup window, device-specific control protocols, new artwork, Bluetooth behavior, alert logic, or GNOME/Windows native-shell redesign. It establishes the popup token and setting so those later surfaces use the same design system.
