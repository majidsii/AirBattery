# AirBattery Liquid Glass Main Window — Design Specification

**Status:** Approved visual direction, awaiting written-spec review
**Date:** 2026-08-04
**Scope:** Desktop main window only (Overview, Devices, Settings, Diagnostics, About)

## 1. Approved Direction

AirBattery will use the approved **minimal graphic black-and-white Liquid Glass direction**:

- The interface remains calm, dark, and professional rather than colorful or gaming-like.
- Graphic background halos are monochrome and reverse between themes.
- Dark mode uses a near-black base with subtly darker black graphic waves.
- Light mode uses a soft off-white base with white pearl-like graphic waves.
- Background graphics stay behind the content hierarchy and never compete with text, device artwork, or battery values.
- Green remains reserved for truthful connection and battery status.
- The active navigation/control accent may follow the operating-system accent color, with blue as the default fallback.

This direction replaces the current blue/green/purple ambient blobs and the overly opaque gray panels.

## 2. Theme System

### 2.1 Dark Mode

- Base canvas: `#07090D` to `#0A0D12`.
- Graphic waves/halos: black and graphite only, approximately `#000000` to `#11151C`.
- Halo opacity: low enough that the design remains visible only through tonal depth and edge sheen.
- No large saturated blue, green, or purple background glow.
- Glass surfaces use a restrained cool-white tint over the black canvas.
- Text remains high contrast without appearing pure-white everywhere.

### 2.2 Light Mode

- Base canvas: `#F5F6F8` to `#FAFBFC`.
- Graphic waves/halos: white and pearl, approximately `#FFFFFF` with subtle cool-gray edge separation.
- Background waves must remain visible through relief, shadow, and refraction rather than colored gradients.
- Glass surfaces use translucent white with restrained shadow and edge highlights.
- The layout and component hierarchy remain identical to dark mode.

### 2.3 Theme Resolution

- `System` follows the operating-system appearance in real time.
- `Dark` and `Light` force their respective palettes.
- The selected theme controls the canvas, glass tint, background waves, text, borders, shadows, and scrollbar.
- No component may keep a hard-coded dark-only background.

## 3. Background Composition

The background is decorative, non-interactive, and rendered as lightweight CSS layers.

- Use two or three broad monochrome wave shapes at most.
- One wave may enter from the upper-right area; another may sit near the lower-left or lower edge.
- Waves must not pass directly behind primary text at strong contrast.
- Avoid radial color blobs, neon bloom, stars, noise-heavy textures, or wallpaper photography.
- Keep the center reading area visually quiet.
- Background graphics must not cause horizontal scrolling or layout reflow.
- Reduced-transparency and high-contrast modes may simplify or remove the waves.

## 4. Main Window Layout

### 4.1 Window Shell

- Preserve native Tauri window controls and platform behavior.
- Use a compact, vertically aligned navigation rail on the left.
- The main content area fills the remaining width and scrolls independently.
- Window padding and radii remain consistent between all main-window routes.
- The main shell must not look like one large opaque card placed on another opaque card.

### 4.2 Navigation Rail

- Display the approved AirBattery battery/Bluetooth brand mark at the top.
- Active navigation uses a small glass selection surface with the system accent.
- Inactive icons remain neutral and readable.
- Tooltips stay above all content and glass layers.
- The About action remains anchored near the bottom.
- The rail itself may use a subtle glass tint but must remain visually lighter than a solid sidebar block.

### 4.3 Overview Header

- Present `Overview`, supporting copy, and active-device count as a clean header region.
- Do not place the entire header inside an oversized heavy glass card.
- The device-count control may use a compact glass pill.
- Typography remains the primary hierarchy; glow is not used to manufacture emphasis.

### 4.4 Device Presentation

- Connected devices appear as compact glass rows or cards.
- Exact reviewed artwork is used when available.
- Product-family artwork is used only when model identity is not exact but the family is known.
- Honest generic category artwork is used otherwise.
- Never show earbuds artwork for a speaker or any unrelated category.
- Battery values remain visually stronger than decorative artwork.
- Missing battery data displays `Unavailable`; no value is invented.
- AirPods and multi-component devices retain left/right/case identity.
- Connection, charging, stale, approximate, and exact states remain textually understandable without relying on color alone.

## 5. Liquid Glass Material

The material must read as layered glass, not as a gray gradient card.

Each glass surface combines:

1. A low-opacity theme-aware tint.
2. Backdrop blur where supported.
3. Mild saturation adjustment.
4. A fine inner highlight along the upper/leading edges.
5. A restrained outer border.
6. A soft depth shadow.
7. Optional subtle refraction/shine through a pseudo-element.

### 5.1 Material Rules

- No large diagonal gray highlight covering most of a card.
- No heavy white outline around every surface.
- Nested glass layers must reduce contrast rather than multiply borders.
- Main cards use stronger depth than secondary rows.
- Active controls may use a clearer edge highlight.
- Stronger intensity increases blur and edge depth while keeping surfaces clearer rather than more opaque.

### 5.2 Presets

Retain the existing presets:

- Off
- Subtle
- Balanced
- Deep
- Crystal

`Balanced` is the default. Presets change material strength, not the layout or background theme.

### 5.3 Per-Surface Controls

Keep independent settings for:

- Main window
- Desktop widget
- Connection popup

Only the main-window visual implementation is part of this phase. Widget and popup consume the same tokens in later phases.

## 6. Scroll Behavior

The current browser-like scrollbar is replaced with an overlay-style scrollbar.

- Idle: visually hidden while the page remains scrollable.
- Hover/focus: a narrow thumb fades in.
- Dragging: the thumb becomes slightly wider and more opaque.
- No permanent bright track.
- No large reserved gutter.
- The thumb color reverses with the theme: light translucent thumb in dark mode, dark translucent thumb in light mode.
- Keyboard, wheel, touchpad, and drag scrolling must all remain functional.
- `overflow: clip` must not be used on the scrolling content container.
- Firefox and WebKit scrollbar rules must both be covered where practical.

## 7. Motion

- Route changes use short opacity/translation transitions.
- Glass shine does not continuously animate.
- Device updates may use a restrained one-time transition.
- No pulsing background blobs.
- `prefers-reduced-motion` and the existing Reduce Motion setting disable nonessential motion.

## 8. Accessibility

- Maintain WCAG-conscious text contrast in both themes.
- Status is never communicated by color alone.
- Focus rings remain clearly visible over glass.
- Reduced Transparency replaces glass with solid theme-aware surfaces while preserving hierarchy.
- High-contrast mode removes decorative waves and increases border/text separation.
- All controls remain keyboard accessible.
- Scroll content remains reachable and the skip link remains functional.

## 9. Component and Token Boundaries

Implementation should preserve clear responsibilities:

- `glass.ts`: normalization, presets, bounded material values.
- `glass.css`: shared material tokens and reusable glass utilities.
- Theme/background tokens: monochrome canvas and wave definitions.
- `App.vue`: shell composition only.
- `AppNav.vue`: navigation behavior and brand presentation only.
- Route views: content structure, without duplicating global glass formulas.
- Device artwork/presentation domain: truthful model/category selection independent of CSS.

Large view-specific CSS blocks should be split when they mix shell, material, and content responsibilities.

## 10. Error and Fallback Behavior

- Unsupported backdrop blur falls back to translucent or solid surfaces with correct contrast.
- Reduced Transparency produces a complete usable interface, not partially transparent remnants.
- Missing artwork falls back to the honest generic category asset.
- Missing battery data never blocks rendering the device card.
- Theme detection failure falls back to dark mode only when no persisted explicit choice exists.

## 11. Testing Requirements

### 11.1 Contract Tests

- Approved monochrome theme tokens exist for dark and light modes.
- Saturated ambient background blobs are absent.
- Main content remains scrollable.
- Scrollbar idle/hover/drag states are defined.
- Brand mark remains present in navigation and About.
- Device artwork mapping does not regress across categories.
- Glass presets remain bounded and normalized.
- Reduced Transparency and reduced-motion fallbacks exist.

### 11.2 Visual Verification

Capture and review at minimum:

- Dark mode, normal window size.
- Light mode, normal window size.
- Settings page scrolled to Glass controls.
- Multiple connected device types.
- Unavailable battery state.
- Narrow window behavior.
- Balanced and Crystal presets.
- Reduced Transparency enabled.
- Scrollbar idle, hover, and dragging states.

### 11.3 Build Verification

- Desktop tests pass.
- Vue typecheck passes.
- Vite production build passes.
- Desktop source verification passes.
- Tauri source verification passes.
- Rust formatting passes.
- `git diff --check` passes.

## 12. Phase Boundaries

### Included

- Final visual system for the desktop main window.
- Dark/light monochrome backgrounds.
- Refined glass material.
- Navigation, Overview, device cards, shared route surfaces, and scrollbar.
- Accessibility fallbacks and visual contract tests.

### Excluded

- GNOME quick panel redesign.
- Windows tray flyout redesign.
- Desktop widget final layout.
- Connection popup final layout.
- New Bluetooth protocol or battery-detection work.
- New exact-model artwork production beyond assets already reviewed.

These excluded surfaces will adopt the approved tokens in later implementation phases.

## 13. Acceptance Criteria

The phase is accepted when:

1. Dark mode looks predominantly black, with only subtle black/graphite graphic depth.
2. Light mode looks predominantly off-white, with soft white/pearl graphic depth.
3. No saturated background blobs remain.
4. Glass reads as translucent layered material rather than opaque gray gradients.
5. The main window is fully scrollable with an unobtrusive overlay scrollbar.
6. Device artwork and battery data remain truthful.
7. Dark and light modes clearly belong to the same design family.
8. The approved AirBattery brand mark appears consistently in the app shell.
9. All automated verification commands pass.
10. Visual screenshots match the approved Option 2 direction without introducing unrelated layout changes.
