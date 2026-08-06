# AirBattery Liquid Glass Main Window Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Replace the current saturated gradient/glass prototype with the approved Option 2 main-window design: monochrome black waves in dark mode, white/pearl waves in light mode, clear layered Liquid Glass surfaces, compact truthful device cards, and a discreet theme-aware scrollbar.

**Architecture:** Keep battery data, artwork selection, and persistence unchanged. Split visual responsibilities into semantic theme tokens (`tokens.css`), decorative monochrome canvas layers (`ambient.css`), reusable material/scrollbar rules (`glass.css`), and route/layout styling (`base.css`). Vue components provide semantic structure only; all glass formulas remain centralized in `glass.css` and all appearance values remain driven by `domain/glass.ts` plus the settings store.

**Tech Stack:** Vue 3.5, Pinia 4, TypeScript 6, Vite 8, Tauri 2, CSS custom properties, Node built-in test runner, Python source verifiers.

## Global Constraints

- Dark canvas is predominantly `#07090D` to `#0A0D12`; decorative depth is black/graphite only.
- Light canvas is predominantly `#F5F6F8` to `#FAFBFC`; decorative depth is white/pearl only.
- No saturated blue, green, purple, orange, or teal ambient background blobs.
- Green remains reserved for truthful battery and connection state.
- The active navigation accent may use the operating-system accent color; blue is the fallback.
- `System` theme follows operating-system appearance through CSS media queries without requiring an application restart.
- Glass presets remain `Off`, `Subtle`, `Balanced`, `Deep`, and `Crystal`; `Balanced` stays the default.
- Stronger glass increases blur, edge depth, and refraction while making the tint clearer rather than more opaque.
- Missing battery data displays `Unavailable`; no battery value may be invented.
- Exact reviewed artwork is preferred; otherwise use product-family or honest generic category artwork.
- The main content remains scrollable by keyboard, wheel, touchpad, and thumb dragging.
- Reduced Transparency, reduced motion, unsupported backdrop blur, and high-contrast modes must remain fully usable.
- No new runtime or development dependencies are added.
- GNOME quick panel, Windows tray flyout, desktop widget final layout, connection popup final layout, protocol work, and new exact-model artwork are out of scope.

---

## File Structure

### New files

- `apps/desktop/src/styles/ambient.css` — monochrome canvas and decorative wave layers only.
- `apps/desktop/src/components/NavIcon.vue` — accessible, route-keyed inline SVG navigation icons.
- `apps/desktop/tests/theme-visual-contract.test.ts` — source-level contracts for dark/light palette, wave inversion, and absence of saturated ambient colors.
- `docs/superpowers/verification/2026-08-04-liquid-glass-main-window.md` — manual screenshot matrix and final visual acceptance record.

### Modified files

- `apps/desktop/src/main.ts` — imports `ambient.css` between base tokens and glass material.
- `apps/desktop/src/styles/tokens.css` — semantic dark/light palette, scrollbar, focus, and surface tokens.
- `apps/desktop/src/styles/base.css` — shell, navigation, route layout, device metrics, and responsive rules.
- `apps/desktop/src/styles/glass.css` — reusable glass material, fallback, and scrollbar implementation; no ambient artwork.
- `apps/desktop/src/domain/glass.ts` — preset-to-CSS calculations with clearer high-intensity glass.
- `apps/desktop/src/App.vue` — named non-interactive ambient wave elements.
- `apps/desktop/src/components/AppNav.vue` — uses `NavIcon` instead of text glyphs.
- `apps/desktop/src/components/DeviceBatteryPanel.vue` — one truthful artwork presentation with compact battery metrics and progress bars.
- `apps/desktop/src/views/BatteryView.vue` — approved header hierarchy and compact device-card composition.
- `apps/desktop/src/views/DevicesView.vue` — shared page/surface hierarchy.
- `apps/desktop/src/views/SettingsView.vue` — shared page/surface hierarchy without changing settings behavior.
- `apps/desktop/src/views/DiagnosticsView.vue` — shared page/surface hierarchy and horizontal table overflow.
- `apps/desktop/src/views/AboutView.vue` — shared page/surface hierarchy and approved brand mark.
- `apps/desktop/tests/glass-ui-contract.test.ts` — updated structural/material/scrollbar contracts.
- `apps/desktop/tests/glass.test.ts` — updated glass-variable expectations.
- `apps/desktop/tests/battery-view-contract.test.ts` — compact card and truthful metric contracts.
- `apps/desktop/tests/visual-priority.test.ts` — battery values remain stronger than decoration.
- `scripts/verify-desktop-source.py` — verifies new files, palette rules, fallbacks, and no legacy ambient blobs.

---

### Task 1: Lock the Approved Theme and File-Boundary Contracts

**Files:**
- Create: `apps/desktop/tests/theme-visual-contract.test.ts`
- Modify: `apps/desktop/tests/glass-ui-contract.test.ts`
- Modify: `scripts/verify-desktop-source.py`

**Interfaces:**
- Consumes: existing `App.vue`, `main.ts`, `tokens.css`, `glass.css` source files.
- Produces: enforceable contracts for `ambient.css`, semantic theme variables, named wave elements, material separation, and overlay scrollbar behavior.

- [ ] **Step 1: Write the failing theme visual contract test**

```ts
import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

const main = readFileSync(new URL('../src/main.ts', import.meta.url), 'utf8');
const app = readFileSync(new URL('../src/App.vue', import.meta.url), 'utf8');
const tokens = readFileSync(new URL('../src/styles/tokens.css', import.meta.url), 'utf8');
const ambient = readFileSync(new URL('../src/styles/ambient.css', import.meta.url), 'utf8');
const glass = readFileSync(new URL('../src/styles/glass.css', import.meta.url), 'utf8');

test('approved Option 2 uses a dedicated monochrome ambient layer', () => {
  assert.match(main, /\.\/styles\/ambient\.css/);
  assert.match(app, /ambient-background__wave--top/);
  assert.match(app, /ambient-background__wave--bottom/);
  assert.match(ambient, /--canvas:\s*#f5f6f8/i);
  assert.match(ambient, /--canvas:\s*#07090d/i);
  assert.match(ambient, /--wave-fill:\s*255,\s*255,\s*255/);
  assert.match(ambient, /--wave-fill:\s*0,\s*0,\s*0/);
});

test('ambient artwork contains no saturated color blobs', () => {
  assert.doesNotMatch(ambient, /rgba\(\s*(?:56|65|69|80|86|92|93|94|95|96|99|119|181)\s*,/i);
  assert.doesNotMatch(ambient, /(?:blue|green|purple|teal|violet|orange)/i);
  assert.doesNotMatch(glass, /ambient-background|radial-gradient\([^)]*var\(--accent\)/s);
});

test('theme-aware scrollbar has no permanently reserved gutter', () => {
  assert.match(tokens, /--scroll-thumb-idle:/);
  assert.match(tokens, /--scroll-thumb-active:/);
  assert.doesNotMatch(glass, /scrollbar-gutter:\s*stable/);
  assert.match(glass, /scrollbar-color:\s*transparent transparent/);
  assert.match(glass, /\.app-content:hover::-webkit-scrollbar-thumb/);
});
```

- [ ] **Step 2: Update the legacy scrollbar assertion**

Replace the old `scrollbar-gutter: stable` expectation in `glass-ui-contract.test.ts` with:

```ts
test('glass content stays scrollable without a permanent browser gutter', () => {
  assert.doesNotMatch(glassCss, /\.glass-shell \.app-content\s*\{[^}]*overflow:\s*clip/s);
  assert.match(glassCss, /\.glass-shell \.app-content\s*\{[^}]*overflow-y:\s*auto/s);
  assert.doesNotMatch(glassCss, /scrollbar-gutter:\s*stable/);
  assert.match(glassCss, /scrollbar-color:\s*transparent transparent/);
});
```

- [ ] **Step 3: Add failing verifier checks**

Add `ambient.css` to the required file list and add these checks after loading style sources:

```python
ambient_css = (SRC / "styles" / "ambient.css").read_text(encoding="utf-8")
check("--canvas: #07090d" in ambient_css.lower(), "dark Option 2 canvas token is missing")
check("--canvas: #f5f6f8" in ambient_css.lower(), "light Option 2 canvas token is missing")
check("--wave-fill: 0, 0, 0" in ambient_css, "dark waves are not black/graphite")
check("--wave-fill: 255, 255, 255" in ambient_css, "light waves are not white/pearl")
check("ambient-background" not in glass_css, "ambient artwork must not live in glass.css")
check("scrollbar-gutter: stable" not in glass_css, "scrollbar reserves a permanent gutter")
```

- [ ] **Step 4: Run the targeted tests and confirm failure**

Run:

```bash
npm --prefix apps/desktop test -- --test-name-pattern='Option 2|ambient|scrollbar'
python3 scripts/verify-desktop-source.py
```

Expected: failures report missing `ambient.css`, missing named waves, old saturated ambient CSS, and/or the old stable scrollbar gutter.

- [ ] **Step 5: Commit the red tests**

```bash
git add \
  apps/desktop/tests/theme-visual-contract.test.ts \
  apps/desktop/tests/glass-ui-contract.test.ts \
  scripts/verify-desktop-source.py
git commit -m "test(ui): lock approved liquid glass direction"
```

---

### Task 2: Implement the Monochrome Dark/Light Canvas and Waves

**Files:**
- Create: `apps/desktop/src/styles/ambient.css`
- Modify: `apps/desktop/src/main.ts`
- Modify: `apps/desktop/src/App.vue`
- Modify: `apps/desktop/src/styles/tokens.css`
- Modify: `apps/desktop/src/styles/base.css`
- Test: `apps/desktop/tests/theme-visual-contract.test.ts`

**Interfaces:**
- Consumes: `data-theme="light|dark|system"` already written by `useSettingsStore.applyAppearance()`.
- Produces: `--canvas`, `--canvas-deep`, `--wave-fill`, `--wave-edge`, `--surface-rgb`, `--surface-solid-rgb`, `--scroll-thumb-idle`, and `--scroll-thumb-active` CSS variables.

- [ ] **Step 1: Import the dedicated ambient stylesheet**

Update `main.ts` imports to this exact order:

```ts
import './styles/tokens.css';
import './styles/base.css';
import './styles/ambient.css';
import './styles/glass.css';
```

- [ ] **Step 2: Replace anonymous ambient spans with named layers**

Replace the ambient element in `App.vue` with:

```vue
<div class="ambient-background" aria-hidden="true">
  <span class="ambient-background__wave ambient-background__wave--top" />
  <span class="ambient-background__wave ambient-background__wave--bottom" />
</div>
```

- [ ] **Step 3: Add semantic theme variables to `tokens.css`**

Use these variables in the light root:

```css
:root {
  --canvas: #f5f6f8;
  --canvas-deep: #fafbfc;
  --surface-rgb: 255, 255, 255;
  --surface-solid-rgb: 247, 248, 250;
  --wave-fill: 255, 255, 255;
  --wave-edge: 110, 120, 136;
  --scroll-thumb-idle: rgba(13, 18, 26, 0);
  --scroll-thumb-active: rgba(13, 18, 26, 0.24);
  --scroll-thumb-drag: rgba(13, 18, 26, 0.42);
  --text: #11161e;
  --text-secondary: #596271;
  --text-tertiary: #7a8493;
  --stroke: rgba(28, 36, 48, 0.12);
  --stroke-muted: rgba(28, 36, 48, 0.08);
}
```

Use these variables in both the explicit dark selector and the system-dark media selector:

```css
--canvas: #07090d;
--canvas-deep: #0a0d12;
--surface-rgb: 18, 22, 29;
--surface-solid-rgb: 19, 23, 30;
--wave-fill: 0, 0, 0;
--wave-edge: 255, 255, 255;
--scroll-thumb-idle: rgba(255, 255, 255, 0);
--scroll-thumb-active: rgba(255, 255, 255, 0.26);
--scroll-thumb-drag: rgba(255, 255, 255, 0.44);
--text: #f2f4f7;
--text-secondary: #a6aeb9;
--text-tertiary: #7e8794;
--stroke: rgba(255, 255, 255, 0.11);
--stroke-muted: rgba(255, 255, 255, 0.07);
```

Keep `--positive`, `--warning`, `--critical`, and `--stale` status colors unchanged.

- [ ] **Step 4: Create `ambient.css` with the approved inversion**

```css
body {
  background: linear-gradient(145deg, var(--canvas), var(--canvas-deep));
}

.ambient-background {
  position: fixed;
  z-index: 0;
  inset: 0;
  overflow: hidden;
  pointer-events: none;
  background: var(--canvas);
}

.ambient-background__wave {
  position: absolute;
  display: block;
  border-radius: 48% 52% 62% 38% / 42% 46% 54% 58%;
  background: rgba(var(--wave-fill), 0.72);
  border: 1px solid rgba(var(--wave-edge), 0.035);
  box-shadow:
    inset 0 1px 0 rgba(var(--wave-edge), 0.035),
    0 34px 90px rgba(var(--wave-fill), 0.42);
  transform: rotate(-13deg);
  filter: blur(0.2px);
  opacity: calc(0.34 + var(--glass-intensity) * 0.18);
}

.ambient-background__wave--top {
  width: min(76vw, 1120px);
  height: min(44vw, 620px);
  top: -31vw;
  right: -17vw;
}

.ambient-background__wave--bottom {
  width: min(84vw, 1240px);
  height: min(38vw, 540px);
  left: -27vw;
  bottom: -29vw;
  transform: rotate(11deg);
  opacity: calc(0.24 + var(--glass-intensity) * 0.14);
}

@media (max-width: 780px) {
  .ambient-background__wave--top {
    width: 118vw;
    height: 72vw;
    top: -45vw;
    right: -50vw;
  }

  .ambient-background__wave--bottom {
    width: 132vw;
    height: 70vw;
    left: -63vw;
    bottom: -43vw;
  }
}

@media (prefers-contrast: more), (prefers-reduced-transparency: reduce) {
  .ambient-background__wave { display: none; }
}

:root[data-glass-reduced="true"] .ambient-background__wave {
  display: none;
}
```

- [ ] **Step 5: Remove all ambient artwork from `base.css` and `glass.css`**

Delete the old `.ambient-background span`, `nth-child`, colored radial gradients, and `body` ambient gradients. Keep only shell/layout rules in `base.css` and material rules in `glass.css`.

- [ ] **Step 6: Run the targeted contracts**

```bash
npm --prefix apps/desktop test -- --test-name-pattern='Option 2|ambient'
python3 scripts/verify-desktop-source.py
```

Expected: PASS.

- [ ] **Step 7: Commit the canvas implementation**

```bash
git add \
  apps/desktop/src/main.ts \
  apps/desktop/src/App.vue \
  apps/desktop/src/styles/tokens.css \
  apps/desktop/src/styles/base.css \
  apps/desktop/src/styles/ambient.css \
  apps/desktop/src/styles/glass.css
git commit -m "feat(ui): add monochrome adaptive canvas"
```

---

### Task 3: Make the Glass Material Clear, Layered, and Accessible

**Files:**
- Modify: `apps/desktop/src/domain/glass.ts`
- Modify: `apps/desktop/src/styles/glass.css`
- Modify: `apps/desktop/tests/glass.test.ts`
- Modify: `apps/desktop/tests/glass-ui-contract.test.ts`

**Interfaces:**
- Consumes: `GlassPreferences`, `glassCssVariables(preferences)`, semantic `--surface-rgb` and `--surface-solid-rgb` theme tokens.
- Produces: unchanged public TypeScript API with corrected optical behavior and reusable classes `.glass-card`, `.glass-control`, `.glass-chip`, `.glass-preview`, `.glass-subtle`, and `.liquid-surface`.

- [ ] **Step 1: Add failing optical-behavior assertions**

Add to `glass.test.ts`:

```ts
test('crystal stays clearer than subtle while increasing depth', () => {
  const subtle = glassCssVariables(applyGlassPreset(defaultGlassPreferences, 'subtle'));
  const crystal = glassCssVariables(applyGlassPreset(defaultGlassPreferences, 'crystal'));

  assert.ok(Number.parseFloat(crystal['--glass-main-alpha']) < Number.parseFloat(subtle['--glass-main-alpha']));
  assert.ok(Number.parseFloat(crystal['--glass-main-blur']) > Number.parseFloat(subtle['--glass-main-blur']));
  assert.ok(Number.parseFloat(crystal['--glass-main-stroke']) > Number.parseFloat(subtle['--glass-main-stroke']));
  assert.ok(Number.parseFloat(crystal['--glass-main-refraction']) > Number.parseFloat(subtle['--glass-main-refraction']));
});
```

Add to `glass-ui-contract.test.ts`:

```ts
test('glass surfaces use monochrome material without accent tint', () => {
  assert.match(glassCss, /rgba\(var\(--surface-rgb\),\s*var\(--glass-active-alpha\)\)/);
  assert.doesNotMatch(glassCss, /color-mix\([^)]*var\(--accent\)/s);
  assert.match(glassCss, /\.liquid-surface::after/);
  assert.match(glassCss, /prefers-contrast:\s*more/);
});
```

- [ ] **Step 2: Run the tests and confirm failure**

```bash
npm --prefix apps/desktop test -- --test-name-pattern='crystal|monochrome material'
```

Expected: FAIL until the updated formulas and CSS selectors exist.

- [ ] **Step 3: Correct `materialVariables()`**

Replace the calculations with:

```ts
function materialVariables(strength: number, prefix: string): Record<string, string> {
  const ratio = strength / 100;
  const blur = Math.round(10 + ratio * 34);
  const alpha = 0.48 - ratio * 0.24;
  const stroke = 0.12 + ratio * 0.24;
  const refraction = 0.08 + ratio * 0.24;
  const highlight = 0.08 + ratio * 0.20;

  return {
    [`--glass-${prefix}-strength`]: ratio.toFixed(2),
    [`--glass-${prefix}-blur`]: `${blur}px`,
    [`--glass-${prefix}-alpha`]: alpha.toFixed(2),
    [`--glass-${prefix}-stroke`]: stroke.toFixed(2),
    [`--glass-${prefix}-refraction`]: refraction.toFixed(2),
    [`--glass-${prefix}-highlight`]: highlight.toFixed(2),
  };
}
```

Keep normalization, storage, presets, and per-surface APIs unchanged.

- [ ] **Step 4: Replace glass surface formulas**

Use one shared material body:

```css
.glass-card,
.glass-control,
.glass-chip,
.glass-preview,
.glass-subtle,
.liquid-surface {
  position: relative;
  isolation: isolate;
  border: 1px solid rgba(var(--wave-edge), var(--glass-active-stroke));
  background:
    linear-gradient(
      145deg,
      rgba(var(--wave-edge), calc(var(--glass-active-refraction) * 0.42)),
      transparent 28%
    ),
    rgba(var(--surface-rgb), var(--glass-active-alpha));
  box-shadow:
    0 18px 52px rgba(0, 0, 0, calc(0.08 + var(--glass-intensity) * 0.16)),
    inset 0 1px 0 rgba(var(--wave-edge), var(--glass-active-highlight));
  backdrop-filter: blur(var(--glass-active-blur)) saturate(var(--glass-saturation));
  -webkit-backdrop-filter: blur(var(--glass-active-blur)) saturate(var(--glass-saturation));
}

.glass-card::before,
.glass-control::before,
.glass-chip::before,
.glass-preview::before,
.glass-subtle::before,
.liquid-surface::before {
  content: "";
  position: absolute;
  z-index: -1;
  inset: 0;
  border-radius: inherit;
  pointer-events: none;
  background: linear-gradient(
    115deg,
    rgba(var(--wave-edge), calc(var(--glass-active-highlight) * 0.78)),
    transparent 18%,
    transparent 76%,
    rgba(var(--wave-edge), calc(var(--glass-active-highlight) * 0.18))
  );
  opacity: 0.44;
}

.liquid-surface::after {
  content: "";
  position: absolute;
  z-index: -1;
  inset: 1px;
  border-radius: inherit;
  pointer-events: none;
  background: linear-gradient(165deg, transparent 56%, rgba(var(--wave-edge), 0.035));
}
```

Nested surfaces must use lower depth:

```css
.glass-card .glass-card,
.glass-card .glass-control,
.glass-card .glass-subtle {
  box-shadow: inset 0 1px 0 rgba(var(--wave-edge), 0.08);
  border-color: rgba(var(--wave-edge), calc(var(--glass-active-stroke) * 0.62));
}
```

- [ ] **Step 5: Implement complete fallbacks**

```css
:root[data-glass-reduced="true"] .glass-card,
:root[data-glass-reduced="true"] .glass-control,
:root[data-glass-reduced="true"] .glass-chip,
:root[data-glass-reduced="true"] .glass-preview,
:root[data-glass-reduced="true"] .glass-subtle,
:root[data-glass-reduced="true"] .liquid-surface,
@media (prefers-reduced-transparency: reduce) {
  /* keep the same selector list in an inner rule */
}
```

Because CSS cannot place an `@media` token inside a selector list, implement the media rule separately with the same selector list and this body:

```css
background: rgb(var(--surface-solid-rgb));
backdrop-filter: none;
-webkit-backdrop-filter: none;
box-shadow: 0 10px 28px rgba(0, 0, 0, 0.12);
```

Add:

```css
@supports not (backdrop-filter: blur(1px)) {
  .glass-card,
  .glass-control,
  .glass-chip,
  .glass-preview,
  .glass-subtle,
  .liquid-surface {
    background: rgba(var(--surface-solid-rgb), 0.96);
  }
}

@media (prefers-contrast: more) {
  .glass-card,
  .glass-control,
  .glass-chip,
  .glass-preview,
  .glass-subtle,
  .liquid-surface {
    border-width: 2px;
    border-color: currentColor;
    background: rgb(var(--surface-solid-rgb));
    backdrop-filter: none;
    -webkit-backdrop-filter: none;
  }
}
```

- [ ] **Step 6: Run glass tests**

```bash
npm --prefix apps/desktop test -- --test-name-pattern='glass|crystal|monochrome material'
python3 scripts/verify-desktop-source.py
```

Expected: PASS.

- [ ] **Step 7: Commit the material system**

```bash
git add \
  apps/desktop/src/domain/glass.ts \
  apps/desktop/src/styles/glass.css \
  apps/desktop/tests/glass.test.ts \
  apps/desktop/tests/glass-ui-contract.test.ts
git commit -m "feat(ui): refine liquid glass material"
```

---

### Task 4: Refine the Window Shell, Navigation, and Overlay Scrollbar

**Files:**
- Create: `apps/desktop/src/components/NavIcon.vue`
- Modify: `apps/desktop/src/components/AppNav.vue`
- Modify: `apps/desktop/src/styles/base.css`
- Modify: `apps/desktop/src/styles/glass.css`
- Modify: `apps/desktop/tests/glass-ui-contract.test.ts`

**Interfaces:**
- Consumes: `AppRoute` values `battery`, `devices`, `settings`, `diagnostics`, and `about`.
- Produces: `<NavIcon :name="route" />` and the existing `navigate` event with no route/API changes.

- [ ] **Step 1: Add failing navigation contracts**

```ts
const navIcon = readFileSync(new URL('../src/components/NavIcon.vue', import.meta.url), 'utf8');

test('navigation uses semantic SVG icons and keeps tooltips above glass', () => {
  assert.match(appNav, /import NavIcon/);
  assert.match(appNav, /<NavIcon :name="item\.route"/);
  assert.doesNotMatch(appNav, /glyph:/);
  assert.match(navIcon, /battery|devices|settings|diagnostics|about/);
  assert.match(glassCss, /\.nav-button__label\s*\{[^}]*z-index:/s);
});
```

- [ ] **Step 2: Run the contract and confirm failure**

```bash
npm --prefix apps/desktop test -- --test-name-pattern='navigation uses semantic'
```

Expected: FAIL because `NavIcon.vue` does not exist and `AppNav` still uses text glyphs.

- [ ] **Step 3: Create `NavIcon.vue`**

```vue
<script setup lang="ts">
import type { AppRoute } from './AppNav.vue';

defineProps<{ name: AppRoute }>();
</script>

<template>
  <svg class="nav-icon" viewBox="0 0 24 24" fill="none" aria-hidden="true">
    <template v-if="name === 'battery'">
      <circle cx="12" cy="12" r="7.25" />
      <circle cx="12" cy="12" r="2.25" class="nav-icon__fill" />
    </template>
    <template v-else-if="name === 'devices'">
      <path d="M6.5 9.5 10 13l4-4 3.5 3.5" />
      <path d="M8 6.5 16 17.5" />
    </template>
    <template v-else-if="name === 'settings'">
      <circle cx="12" cy="12" r="2.75" />
      <path d="M12 3.75v2M12 18.25v2M3.75 12h2M18.25 12h2M6.17 6.17l1.42 1.42M16.41 16.41l1.42 1.42M17.83 6.17l-1.42 1.42M7.59 16.41l-1.42 1.42" />
    </template>
    <template v-else-if="name === 'diagnostics'">
      <path d="M5 15h3l2-6 3.2 9 2.1-6H19" />
    </template>
    <template v-else>
      <circle cx="12" cy="12" r="8" />
      <path d="M12 10.5v5M12 7.5h.01" />
    </template>
  </svg>
</template>
```

- [ ] **Step 4: Replace glyph data in `AppNav.vue`**

Use route/label-only arrays:

```ts
import NavIcon from './NavIcon.vue';

const primaryItems: Array<{ route: AppRoute; label: string }> = [
  { route: 'battery', label: 'Battery' },
  { route: 'devices', label: 'Devices' },
  { route: 'settings', label: 'Settings' },
  { route: 'diagnostics', label: 'Diagnostics' },
];

const footerItems: Array<{ route: AppRoute; label: string }> = [
  { route: 'about', label: 'About' },
];
```

Replace each glyph span with:

```vue
<NavIcon :name="item.route" />
```

- [ ] **Step 5: Apply compact shell geometry in `base.css`**

```css
.app-shell {
  position: relative;
  z-index: 1;
  display: grid;
  grid-template-columns: 72px minmax(0, 1fr);
  height: 100dvh;
  padding: 14px;
  gap: 14px;
}

.app-content {
  min-width: 0;
  overflow: auto;
  border-radius: 28px;
}

.app-nav {
  display: flex;
  flex-direction: column;
  align-items: center;
  gap: 8px;
  padding: 12px 8px;
  border-radius: 26px;
}

.nav-button {
  position: relative;
  display: grid;
  place-items: center;
  width: 46px;
  min-height: 44px;
  border-radius: 14px;
}

.nav-icon {
  width: 21px;
  height: 21px;
  stroke: currentColor;
  stroke-width: 1.7;
  stroke-linecap: round;
  stroke-linejoin: round;
}

.nav-icon__fill {
  fill: currentColor;
  stroke: none;
}
```

Keep the existing brand mark and navigation event behavior.

- [ ] **Step 6: Implement the true overlay-style scrollbar**

```css
.glass-shell .app-content {
  overflow-x: hidden;
  overflow-y: auto;
  overscroll-behavior: contain;
  scrollbar-width: thin;
  scrollbar-color: transparent transparent;
}

.glass-shell .app-content:hover,
.glass-shell .app-content:focus-within {
  scrollbar-color: var(--scroll-thumb-active) transparent;
}

.glass-shell .app-content::-webkit-scrollbar {
  width: 7px;
  height: 7px;
}

.glass-shell .app-content::-webkit-scrollbar-track {
  background: transparent;
}

.glass-shell .app-content::-webkit-scrollbar-thumb {
  min-height: 44px;
  border: 2px solid transparent;
  border-radius: 999px;
  background: var(--scroll-thumb-idle);
  background-clip: padding-box;
}

.glass-shell .app-content:hover::-webkit-scrollbar-thumb,
.glass-shell .app-content:focus-within::-webkit-scrollbar-thumb {
  background: var(--scroll-thumb-active);
  background-clip: padding-box;
}

.glass-shell .app-content::-webkit-scrollbar-thumb:hover,
.glass-shell .app-content::-webkit-scrollbar-thumb:active {
  background: var(--scroll-thumb-drag);
  background-clip: padding-box;
}

.glass-shell .app-content::-webkit-scrollbar-button {
  display: none;
  width: 0;
  height: 0;
}
```

- [ ] **Step 7: Preserve narrow-window navigation behavior**

At `max-width: 780px`, keep the existing horizontal navigation conversion, but use `grid-template-columns: 1fr`, `grid-template-rows: auto minmax(0, 1fr)`, and ensure `.app-content { min-height: 0; }`.

- [ ] **Step 8: Run tests and typecheck**

```bash
npm --prefix apps/desktop test -- --test-name-pattern='navigation|scrollbar|scrollable'
npm --prefix apps/desktop run typecheck
```

Expected: PASS.

- [ ] **Step 9: Commit shell/navigation work**

```bash
git add \
  apps/desktop/src/components/NavIcon.vue \
  apps/desktop/src/components/AppNav.vue \
  apps/desktop/src/styles/base.css \
  apps/desktop/src/styles/glass.css \
  apps/desktop/tests/glass-ui-contract.test.ts
git commit -m "feat(ui): refine glass shell and navigation"
```

---

### Task 5: Rebuild the Overview Device Cards Around Truthful Metrics

**Files:**
- Modify: `apps/desktop/src/components/DeviceBatteryPanel.vue`
- Modify: `apps/desktop/src/views/BatteryView.vue`
- Modify: `apps/desktop/src/styles/base.css`
- Modify: `apps/desktop/src/styles/glass.css`
- Modify: `apps/desktop/tests/battery-view-contract.test.ts`
- Modify: `apps/desktop/tests/visual-priority.test.ts`
- Test: `apps/desktop/tests/device-visual.test.ts`

**Interfaces:**
- Consumes: `buildBatteryLayout(device)`, `selectDeviceArtwork(device)`, `effectiveConnectionState(device)`, and `presentBatteryComponent(component, state)` without changing their signatures.
- Produces: one artwork per device card and a `presentedSlots` array containing `componentType`, `label`, `percentage`, `valueText`, `statusText`, `tone`, and `ariaLabel`.

- [ ] **Step 1: Add failing component-structure contracts**

Add to `battery-view-contract.test.ts`:

```ts
test('overview device cards render one artwork and truthful battery metrics', () => {
  assert.match(panelSource, /const presentedSlots = computed/);
  assert.match(panelSource, /class="device-battery-panel__artwork"/);
  assert.match(panelSource, /class="battery-metric__track"/);
  assert.match(panelSource, /slot\.percentage !== null/);
  assert.match(panelSource, /width: `\$\{slot\.percentage\}%`/);
  assert.doesNotMatch(panelSource, /v-for="slot[^>]+<DeviceArtwork/s);
});
```

Update `visual-priority.test.ts` to assert:

```ts
assert.match(css, /\.battery-metric__value\s*\{[^}]*font-size:\s*clamp\(28px,\s*3vw,\s*40px\)/s);
assert.match(css, /\.device-battery-panel__artwork\s*\{[^}]*opacity:\s*0\.9/s);
```

- [ ] **Step 2: Run the targeted tests and confirm failure**

```bash
npm --prefix apps/desktop test -- --test-name-pattern='one artwork|battery metrics|visually stronger'
```

Expected: FAIL against the current repeated-artwork slot layout.

- [ ] **Step 3: Build `presentedSlots` in `DeviceBatteryPanel.vue`**

Add:

```ts
const presentedSlots = computed(() =>
  layout.value.slots.map((slot) => {
    const presented = presentBatteryComponent(slot.component, connectionState.value);
    return {
      componentType: slot.componentType,
      label: labels[slot.componentType],
      percentage: slot.component.percentage,
      charging: slot.component.chargingState === 'charging',
      ...presented,
    };
  }),
);

const artworkComponent = computed<ComponentType>(() => {
  if (layout.value.artworkMode === 'pairedEarbuds') return 'aggregate';
  return layout.value.slots[0]?.componentType ?? 'unknown';
});
```

- [ ] **Step 4: Replace the repeated-slot template**

```vue
<div
  class="device-battery-panel"
  :class="[`layout-${layout.mode}`, { 'is-compact': compact }]"
  aria-live="polite"
>
  <div class="device-battery-panel__artwork">
    <DeviceArtwork
      :artwork-key="artwork"
      :component-type="artworkComponent"
      :paired="layout.artworkMode === 'pairedEarbuds'"
      :label="device.displayName"
    />
  </div>

  <div class="device-battery-panel__metrics">
    <article
      v-for="slot in presentedSlots"
      :key="slot.componentType"
      class="battery-metric"
      :class="`tone-${slot.tone}`"
      :aria-label="slot.ariaLabel"
    >
      <div class="battery-metric__heading">
        <span>{{ slot.label }}</span>
        <span v-if="slot.charging" class="battery-metric__charging">Charging</span>
      </div>
      <div class="battery-metric__value">{{ slot.valueText }}</div>
      <div v-if="slot.percentage !== null" class="battery-metric__track" aria-hidden="true">
        <span :style="{ width: `${slot.percentage}%` }" />
      </div>
      <p class="battery-metric__status">{{ slot.statusText }}</p>
    </article>
  </div>
</div>
```

This template does not render a progress track for unknown percentages and therefore never converts missing data into a visual zero.

- [ ] **Step 5: Keep `BatteryView` hierarchy compact**

Use this main-window header copy:

```vue
<header v-if="!props.compact" class="overview-header">
  <div class="overview-header__copy">
    <p class="eyebrow">AirBattery</p>
    <h1 id="battery-title">Overview</h1>
    <p>Monitor your Bluetooth devices and their truthful battery state.</p>
  </div>
  <div class="overview-header__status glass-chip" aria-live="polite">
    <span aria-hidden="true" />
    <strong>{{ displayedDevices.length }}</strong>
    <small>{{ displayedDevices.length === 1 ? 'Device' : 'Devices' }}</small>
  </div>
</header>
```

Keep each card header with display name, optional model, textual `StatusPill`, and last-updated time. Keep the freshness note after the device list as a compact secondary surface; do not place it beside the first device card.

- [ ] **Step 6: Add compact device metric CSS**

```css
.connected-device-list {
  display: grid;
  gap: 14px;
}

.connected-device-card {
  padding: 18px;
  border-radius: 24px;
}

.device-battery-panel {
  display: grid;
  grid-template-columns: minmax(120px, 168px) minmax(0, 1fr);
  gap: 22px;
  align-items: center;
}

.device-battery-panel__artwork {
  display: grid;
  place-items: center;
  min-height: 132px;
  opacity: 0.9;
}

.device-battery-panel__artwork .device-artwork {
  width: min(100%, 160px);
  height: 126px;
}

.device-battery-panel__metrics {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(130px, 1fr));
  gap: 12px;
}

.battery-metric {
  min-width: 0;
  padding: 12px 14px;
  border-radius: 16px;
  background: rgba(var(--surface-rgb), 0.14);
}

.battery-metric__heading {
  display: flex;
  justify-content: space-between;
  gap: 8px;
  color: var(--text-secondary);
  font-size: 12px;
}

.battery-metric__value {
  margin-top: 6px;
  font-size: clamp(28px, 3vw, 40px);
  font-weight: 760;
  line-height: 1;
  letter-spacing: -0.045em;
}

.battery-metric__track {
  height: 5px;
  margin-top: 10px;
  overflow: hidden;
  border-radius: 999px;
  background: var(--stroke-muted);
}

.battery-metric__track span {
  display: block;
  height: 100%;
  border-radius: inherit;
  background: currentColor;
}

.battery-metric__status {
  margin: 7px 0 0;
  color: var(--text-secondary);
  font-size: 12px;
}

.battery-metric.tone-known,
.battery-metric.tone-charging { color: var(--positive); }
.battery-metric.tone-warning { color: var(--warning); }
.battery-metric.tone-critical { color: var(--critical); }
.battery-metric.tone-stale { color: var(--stale); }
.battery-metric.tone-unavailable { color: var(--text-tertiary); }
```

At `max-width: 620px`, switch `.device-battery-panel` to one column and keep metrics at `repeat(auto-fit, minmax(112px, 1fr))`.

- [ ] **Step 7: Run truthful-data and visual tests**

```bash
npm --prefix apps/desktop test -- --test-name-pattern='battery|artwork|truthful|visual'
npm --prefix apps/desktop run typecheck
```

Expected: PASS, including device category mapping and unavailable battery behavior.

- [ ] **Step 8: Commit the Overview redesign**

```bash
git add \
  apps/desktop/src/components/DeviceBatteryPanel.vue \
  apps/desktop/src/views/BatteryView.vue \
  apps/desktop/src/styles/base.css \
  apps/desktop/src/styles/glass.css \
  apps/desktop/tests/battery-view-contract.test.ts \
  apps/desktop/tests/visual-priority.test.ts
git commit -m "feat(ui): rebuild truthful device overview"
```

---

### Task 6: Apply the Same Design Family to Every Main-Window Route

**Files:**
- Modify: `apps/desktop/src/views/DevicesView.vue`
- Modify: `apps/desktop/src/views/SettingsView.vue`
- Modify: `apps/desktop/src/views/DiagnosticsView.vue`
- Modify: `apps/desktop/src/views/AboutView.vue`
- Modify: `apps/desktop/src/styles/base.css`
- Modify: `apps/desktop/tests/glass-ui-contract.test.ts`

**Interfaces:**
- Consumes: existing store actions, controls, diagnostic data, and About content unchanged.
- Produces: shared `.page-header`, `.page-stack`, `.section-surface`, and `.route-grid` structure; no data-flow or settings API changes.

- [ ] **Step 1: Add failing shared-route contracts**

```ts
const devicesView = readFileSync(new URL('../src/views/DevicesView.vue', import.meta.url), 'utf8');
const diagnosticsView = readFileSync(new URL('../src/views/DiagnosticsView.vue', import.meta.url), 'utf8');

test('all main routes share the approved header and surface hierarchy', () => {
  for (const source of [devicesView, settingsView, diagnosticsView, aboutView]) {
    assert.match(source, /class="page-header/);
    assert.match(source, /class="page-stack/);
  }
  assert.match(settingsView, /settings-group glass-card section-surface/);
  assert.match(diagnosticsView, /diagnostic-table-wrap glass-subtle/);
});
```

- [ ] **Step 2: Run the contract and confirm failure**

```bash
npm --prefix apps/desktop test -- --test-name-pattern='all main routes'
```

Expected: FAIL until route markup is normalized.

- [ ] **Step 3: Normalize route headers without changing copy or behavior**

For each route, use:

```vue
<header class="page-header">
  <div>
    <p class="eyebrow">...</p>
    <h1>...</h1>
    <p class="page-header__summary">...</p>
  </div>
  <!-- existing route actions remain here -->
</header>
```

Wrap route content in:

```vue
<div class="page-stack">
  <!-- existing sections -->
</div>
```

Add `section-surface` alongside existing `glass-card` on major sections. Keep all `v-model`, event handlers, stores, and conditional rendering unchanged.

- [ ] **Step 4: Normalize shared route spacing**

```css
.page {
  width: min(100%, 1120px);
  min-height: 100%;
  margin: 0 auto;
  padding: clamp(24px, 4vw, 48px);
}

.page-header {
  display: flex;
  align-items: flex-start;
  justify-content: space-between;
  gap: 24px;
  margin-bottom: 24px;
}

.page-header h1 {
  margin: 0;
  font-size: clamp(36px, 5vw, 58px);
  line-height: 0.98;
  letter-spacing: -0.052em;
}

.page-header__summary {
  max-width: 620px;
  margin: 14px 0 0;
  color: var(--text-secondary);
}

.page-stack {
  display: grid;
  gap: 16px;
}

.section-surface {
  padding: clamp(18px, 2.4vw, 26px);
  border-radius: 24px;
}
```

Do not add a giant glass panel around the whole route header.

- [ ] **Step 5: Preserve Settings scrolling and Diagnostics overflow**

Ensure:

```css
.settings-page,
.diagnostics-page { min-height: max-content; }

.diagnostic-table-wrap {
  max-width: 100%;
  overflow-x: auto;
  border-radius: 18px;
}
```

Keep the main `app-content` as the vertical scroll owner; no route may set `overflow: clip`.

- [ ] **Step 6: Run route contracts and typecheck**

```bash
npm --prefix apps/desktop test -- --test-name-pattern='main routes|settings|diagnostic|about'
npm --prefix apps/desktop run typecheck
```

Expected: PASS.

- [ ] **Step 7: Commit shared route styling**

```bash
git add \
  apps/desktop/src/views/DevicesView.vue \
  apps/desktop/src/views/SettingsView.vue \
  apps/desktop/src/views/DiagnosticsView.vue \
  apps/desktop/src/views/AboutView.vue \
  apps/desktop/src/styles/base.css \
  apps/desktop/tests/glass-ui-contract.test.ts
git commit -m "feat(ui): unify main window route surfaces"
```

---

### Task 7: Complete Motion, Contrast, and Source Verification

**Files:**
- Modify: `apps/desktop/src/styles/base.css`
- Modify: `apps/desktop/src/styles/glass.css`
- Modify: `apps/desktop/tests/glass-ui-contract.test.ts`
- Modify: `apps/desktop/tests/theme-visual-contract.test.ts`
- Modify: `scripts/verify-desktop-source.py`

**Interfaces:**
- Consumes: `data-motion="reduced|full"`, `data-glass-reduced="true|false"`, OS media queries.
- Produces: deterministic no-animation/no-transparency/high-contrast behavior.

- [ ] **Step 1: Add failing accessibility contracts**

```ts
test('motion and contrast fallbacks remove decorative effects without breaking layout', () => {
  assert.match(glassCss, /:root\[data-glass-reduced="true"\]/);
  assert.match(glassCss, /prefers-reduced-transparency:\s*reduce/);
  assert.match(glassCss, /prefers-contrast:\s*more/);
  assert.match(baseCss, /:root\[data-motion="reduced"\]/);
  assert.match(baseCss, /prefers-reduced-motion:\s*reduce/);
  assert.doesNotMatch(ambient, /animation:/);
});
```

- [ ] **Step 2: Run and confirm any missing fallback fails**

```bash
npm --prefix apps/desktop test -- --test-name-pattern='motion and contrast'
```

Expected: FAIL for any absent selector.

- [ ] **Step 3: Add restrained route motion and complete disable rules**

```css
.page-enter-active,
.page-leave-active {
  transition: opacity 150ms ease, transform 150ms ease;
}

.page-enter-from,
.page-leave-to {
  opacity: 0;
  transform: translateY(4px);
}

:root[data-motion="reduced"] *,
:root[data-motion="reduced"] *::before,
:root[data-motion="reduced"] *::after {
  scroll-behavior: auto !important;
  transition-duration: 0.01ms !important;
  animation-duration: 0.01ms !important;
  animation-iteration-count: 1 !important;
}

@media (prefers-reduced-motion: reduce) {
  *, *::before, *::after {
    scroll-behavior: auto !important;
    transition-duration: 0.01ms !important;
    animation-duration: 0.01ms !important;
    animation-iteration-count: 1 !important;
  }
}
```

Do not animate ambient waves or continuously animate glass highlights.

- [ ] **Step 4: Extend source verification**

Add checks that:

```python
check("prefers-contrast: more" in glass_css, "high-contrast glass fallback is missing")
check(":root[data-motion=\"reduced\"]" in css, "explicit reduced-motion selector is missing")
check("animation:" not in ambient_css, "ambient waves must not animate")
check("color-mix" not in glass_css or "var(--accent)" not in glass_css,
      "glass material must not tint surfaces with the accent color")
```

- [ ] **Step 5: Run complete desktop verification**

```bash
npm --prefix apps/desktop test
npm --prefix apps/desktop run typecheck
npm --prefix apps/desktop run build
python3 scripts/verify-desktop-source.py
python3 scripts/verify-tauri-source.py
cargo fmt --all
cargo fmt --all -- --check
git diff --check
```

Expected: every command exits `0`.

- [ ] **Step 6: Commit accessibility and verification**

```bash
git add \
  apps/desktop/src/styles/base.css \
  apps/desktop/src/styles/glass.css \
  apps/desktop/tests/glass-ui-contract.test.ts \
  apps/desktop/tests/theme-visual-contract.test.ts \
  scripts/verify-desktop-source.py
git commit -m "test(ui): verify liquid glass accessibility"
```

---

### Task 8: Perform Visual Acceptance and Record Evidence

**Files:**
- Create: `docs/superpowers/verification/2026-08-04-liquid-glass-main-window.md`
- No production code changes unless a screenshot exposes a failed acceptance criterion.

**Interfaces:**
- Consumes: built Tauri application and approved Option 2 design spec.
- Produces: a checked visual matrix with screenshot filenames and explicit pass/fail notes.

- [ ] **Step 1: Build and install the latest development binary**

```bash
npm --prefix apps/desktop run build
cargo build -p airbattery-desktop --bin airbattery
./scripts/install-linux-desktop-integration.sh
```

Expected: all commands exit `0` and the canonical launcher remains `io.github.airbattery.airbattery.desktop`.

- [ ] **Step 2: Start Vite and launch through the canonical desktop ID**

Terminal 1:

```bash
npm --prefix apps/desktop run dev
```

Terminal 2:

```bash
gtk-launch io.github.airbattery.airbattery
```

Expected: the Dock shows the approved AirBattery logo rather than a gear icon.

- [ ] **Step 3: Capture the required screenshot matrix**

Create `docs/superpowers/verification/liquid-glass-main-window/` and save screenshots with these exact names:

```text
dark-balanced-overview.png
light-balanced-overview.png
dark-crystal-overview.png
light-crystal-overview.png
settings-glass-controls.png
multiple-device-types.png
unavailable-battery.png
narrow-window.png
reduced-transparency.png
scrollbar-hover.png
scrollbar-dragging.png
```

Each screenshot must show only the application window and enough surrounding desktop to verify the window boundary and Dock icon where relevant.

- [ ] **Step 4: Write the visual verification record**

Use this exact document structure:

```markdown
# Liquid Glass Main Window Visual Verification

- [ ] Dark mode is predominantly black with black/graphite waves.
- [ ] Light mode is predominantly off-white with white/pearl waves.
- [ ] No saturated ambient blobs remain.
- [ ] Balanced glass is translucent and readable.
- [ ] Crystal is clearer, blurrier, and more edge-defined than Balanced.
- [ ] Overview header is not enclosed by a giant card.
- [ ] Each device card uses truthful category/model artwork.
- [ ] Missing battery data reads `Unavailable` without a fake zero bar.
- [ ] Settings is fully scrollable.
- [ ] Scrollbar is hidden when idle and visible on hover/drag.
- [ ] Reduced Transparency is fully solid and readable.
- [ ] Narrow layout preserves all controls and battery data.
- [ ] Dock uses the approved AirBattery icon when launched through the canonical launcher.

## Evidence

| State | Screenshot | Result | Notes |
|---|---|---|---|
| Dark Balanced | `liquid-glass-main-window/dark-balanced-overview.png` | PASS | Black canvas and graphite waves only. |
| Light Balanced | `liquid-glass-main-window/light-balanced-overview.png` | PASS | Off-white canvas and pearl waves only. |
| Dark Crystal | `liquid-glass-main-window/dark-crystal-overview.png` | PASS | Clearer tint with stronger depth. |
| Light Crystal | `liquid-glass-main-window/light-crystal-overview.png` | PASS | Clearer tint with stronger depth. |
| Settings | `liquid-glass-main-window/settings-glass-controls.png` | PASS | Glass controls reachable by scrolling. |
| Multiple devices | `liquid-glass-main-window/multiple-device-types.png` | PASS | Category artwork remains truthful. |
| Unavailable | `liquid-glass-main-window/unavailable-battery.png` | PASS | No invented percentage. |
| Narrow window | `liquid-glass-main-window/narrow-window.png` | PASS | Navigation and metrics remain usable. |
| Reduced transparency | `liquid-glass-main-window/reduced-transparency.png` | PASS | Solid surfaces preserve hierarchy. |
| Scroll hover | `liquid-glass-main-window/scrollbar-hover.png` | PASS | Thin theme-aware thumb. |
| Scroll drag | `liquid-glass-main-window/scrollbar-dragging.png` | PASS | More visible active thumb. |
```

Change `PASS` to `FAIL` only when the corresponding screenshot fails the criterion, and describe the visible mismatch precisely.

- [ ] **Step 5: Re-run the complete verification after any visual correction**

```bash
npm --prefix apps/desktop test
npm --prefix apps/desktop run typecheck
npm --prefix apps/desktop run build
python3 scripts/verify-desktop-source.py
python3 scripts/verify-tauri-source.py
cargo fmt --all -- --check
git diff --check
```

Expected: every command exits `0`.

- [ ] **Step 6: Commit visual evidence**

```bash
git add \
  docs/superpowers/verification/2026-08-04-liquid-glass-main-window.md \
  docs/superpowers/verification/liquid-glass-main-window
git commit -m "docs(ui): verify liquid glass main window"
```

---

## Final Acceptance Gate

Before opening a pull request, confirm all ten specification criteria:

1. Dark mode is black with only black/graphite depth.
2. Light mode is off-white with only white/pearl depth.
3. Saturated ambient blobs are absent.
4. Glass is translucent and layered rather than an opaque gray gradient.
5. The main window scrolls normally with a discreet overlay-style thumb.
6. Device artwork and battery information remain truthful.
7. Dark and light are the same design family with inverted monochrome depth.
8. The approved AirBattery mark appears in the shell and canonical Dock launcher.
9. All automated commands pass.
10. Screenshots match approved Option 2 without unrelated layout changes.

Run one final status check:

```bash
git status --short
git log -8 --oneline --decorate
```

Expected: only intentionally committed work exists and the working tree is clean.
