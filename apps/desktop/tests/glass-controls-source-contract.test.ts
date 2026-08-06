import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import test from 'node:test';

const selectUrl = new URL(
  '../src/components/GlassSelect.vue',
  import.meta.url,
);

function readSource(): string {
  return existsSync(selectUrl)
    ? readFileSync(selectUrl, 'utf8')
    : '';
}

test('GlassSelect exposes one accessible shared listbox contract', () => {
  const source = readSource();

  assert.equal(
    existsSync(selectUrl),
    true,
    'GlassSelect.vue must exist',
  );

  assert.match(source, /aria-haspopup="listbox"/);
  assert.match(source, /:aria-expanded=/);
  assert.match(source, /role="listbox"/);
  assert.match(source, /role="option"/);
  assert.match(source, /:aria-selected=/);
  assert.match(source, /@keydown="handleKeydown"/);
  assert.match(source, /pointerdown/);
  assert.match(source, /update:modelValue/);
  assert.match(source, /change:\s*\[value:\s*string\]/);
});

const checkboxUrl = new URL(
  '../src/components/GlassCheckbox.vue',
  import.meta.url,
);

test('GlassCheckbox keeps a semantic native checkbox behind a custom surface', () => {
  const source = existsSync(checkboxUrl)
    ? readFileSync(checkboxUrl, 'utf8')
    : '';

  assert.equal(
    existsSync(checkboxUrl),
    true,
    'GlassCheckbox.vue must exist',
  );

  assert.match(source, /type="checkbox"/);
  assert.match(source, /class="sr-only"/);
  assert.match(source, /glass-checkbox__box/);
  assert.match(source, /glass-checkbox__copy/);
  assert.match(source, /update:modelValue/);
  assert.match(source, /change:\s*\[value:\s*boolean\]/);
  assert.match(source, /:disabled="props\.disabled"/);
});

test('all interactive controls share one glass material contract', () => {
  const glass = readFileSync(
    new URL('../src/styles/glass.css', import.meta.url),
    'utf8',
  );

  const base = readFileSync(
    new URL('../src/styles/base.css', import.meta.url),
    'utf8',
  );

  const styles = `${glass}\n${base}`;

  assert.match(glass, /--control-height:\s*42px/);
  assert.match(glass, /--control-radius:\s*14px/);
  assert.match(glass, /--control-border:/);
  assert.match(glass, /--control-surface:/);
  assert.match(glass, /--control-shadow:/);
  assert.match(glass, /--control-motion:/);

  assert.match(styles, /\.glass-select__trigger/);
  assert.match(styles, /\.glass-select__listbox/);
  assert.match(styles, /\.glass-checkbox\s*\{/);
  assert.match(styles, /\.glass-checkbox__box/);

  assert.match(
    styles,
    /\.glass-select__trigger[\s\S]*?min-height:\s*var\(--control-height\)/,
  );

  assert.match(
    styles,
    /\.glass-preset[\s\S]*?min-height:\s*var\(--control-height\)/,
  );

  assert.match(
    styles,
    /\.number-input[\s\S]*?min-height:\s*var\(--control-height\)/,
  );

  assert.match(
    styles,
    /\.toggle[\s\S]*?var\(--control-surface\)/,
  );

  assert.match(
    styles,
    /\.toggle[\s\S]*?var\(--control-motion\)/,
  );
});

test('Settings uses shared glass controls and preserves existing store behavior', () => {
  const settings = readFileSync(
    new URL('../src/views/SettingsView.vue', import.meta.url),
    'utf8',
  );

  assert.match(
    settings,
    /import GlassSelect from '\.\.\/components\/GlassSelect\.vue'/,
  );

  assert.match(
    settings,
    /import GlassCheckbox from '\.\.\/components\/GlassCheckbox\.vue'/,
  );

  assert.match(
    settings,
    /import type \{ GlassSelectOption \} from '\.\.\/domain\/glass-controls\.ts'/,
  );

  assert.match(settings, /const themeOptions:/);
  assert.match(settings, /function updateTheme\(value: string\): void/);

  assert.match(
    settings,
    /settings\.value\.appearance\.theme = value/,
  );

  assert.match(settings, /settings\.applyAppearance\(\)/);

  assert.match(
    settings,
    /settings\.setGlassAdaptive\(value\)/,
  );

  assert.match(
    settings,
    /settings\.setReduceTransparency\(value\)/,
  );

  assert.match(settings, /<GlassSelect\b/);

  assert.equal(
    (settings.match(/<GlassCheckbox\b/g) ?? []).length,
    2,
    'Settings must render exactly two GlassCheckbox controls',
  );

  assert.doesNotMatch(
    settings,
    /<select\b/,
    'Settings must not retain a native select',
  );

  assert.doesNotMatch(
    settings,
    /<input[^>]*type="checkbox"[^>]*@change="updateAdaptive"/s,
  );

  assert.doesNotMatch(
    settings,
    /<input[^>]*type="checkbox"[^>]*@change="updateReduceTransparency"/s,
  );

  assert.doesNotMatch(
    settings,
    /function updateAdaptive\(event: Event\)/,
  );

  assert.doesNotMatch(
    settings,
    /function updateReduceTransparency\(event: Event\)/,
  );
});

test('DeviceSwitcher uses the shared GlassSelect contract', () => {
  const switcher = readFileSync(
    new URL('../src/components/DeviceSwitcher.vue', import.meta.url),
    'utf8',
  );

  assert.match(
    switcher,
    /import GlassSelect from '\.\/GlassSelect\.vue'/,
  );

  assert.match(
    switcher,
    /import type \{ GlassSelectOption \} from '\.\.\/domain\/glass-controls\.ts'/,
  );

  assert.match(
    switcher,
    /const options = computed<readonly GlassSelectOption\[\]>/,
  );

  assert.match(
    switcher,
    /function selectDevice\(deviceId: string\): void/,
  );

  assert.match(
    switcher,
    /emit\('select', deviceId\)/,
  );

  assert.match(switcher, /<GlassSelect\b/);

  assert.match(
    switcher,
    /:model-value="props\.selectedId \?\? ''"/,
  );

  assert.match(
    switcher,
    /:options="options"/,
  );

  assert.match(
    switcher,
    /:disabled="props\.devices\.length < 2"/,
  );

  assert.match(
    switcher,
    /@update:model-value="selectDevice"/,
  );

  assert.doesNotMatch(
    switcher,
    /<select\b/,
    'DeviceSwitcher must not retain its native select',
  );

  assert.doesNotMatch(
    switcher,
    /device-switcher__chevron/,
    'The shared GlassSelect owns its chevron',
  );
});

test('legacy native select styles are removed after GlassSelect migration', () => {
  const base = readFileSync(
    new URL('../src/styles/base.css', import.meta.url),
    'utf8',
  );

  assert.doesNotMatch(
    base,
    /\.device-switcher\s+select/,
    'DeviceSwitcher must not retain native select styles',
  );

  assert.doesNotMatch(
    base,
    /\.setting-row\s+select/,
    'Settings must not retain native select styles',
  );

  assert.doesNotMatch(
    base,
    /\.device-switcher__chevron/,
    'The removed duplicate chevron must not retain styles',
  );

  assert.match(
    base,
    /\.device-switcher\s+\.glass-select\s*\{/,
  );

  assert.match(
    base,
    /@media \(max-width: 780px\)[\s\S]*?\.device-switcher\s+\.glass-select\s*\{[\s\S]*?width:\s*100%/,
  );

  assert.match(
    base,
    /@media \(max-width: 480px\)[\s\S]*?\.device-switcher\s*\{[\s\S]*?flex:\s*1/,
  );
});

test('legacy accessibility checkbox selectors do not override GlassCheckbox', () => {
  const glass = readFileSync(
    new URL('../src/styles/glass.css', import.meta.url),
    'utf8',
  );

  assert.match(
    glass,
    /\.glass-accessibility\s*\{[\s\S]*?display:\s*grid/,
  );

  assert.doesNotMatch(
    glass,
    /\.glass-accessibility\s+label/,
    'Legacy label styling must not override GlassCheckbox',
  );

  assert.doesNotMatch(
    glass,
    /\.glass-accessibility\s+input/,
    'Legacy input sizing must not override the hidden native checkbox',
  );

  assert.doesNotMatch(
    glass,
    /\.glass-accessibility\s+span/,
    'Legacy span layout must not affect the checkbox box or check glyph',
  );

  assert.doesNotMatch(
    glass,
    /\.glass-accessibility\s+small/,
    'Legacy copy styling must be owned by GlassCheckbox',
  );

  assert.match(glass, /\.glass-checkbox__copy/);
  assert.match(glass, /\.glass-checkbox__box/);
});

test('GlassCheckbox uses a centered SVG indicator instead of a font glyph', () => {
  const component = readFileSync(
    new URL('../src/components/GlassCheckbox.vue', import.meta.url),
    'utf8',
  );

  const css = readFileSync(
    new URL('../src/styles/glass.css', import.meta.url),
    'utf8',
  );

  assert.match(
    component,
    /class="glass-checkbox__check-icon"/,
  );

  assert.doesNotMatch(
    component,
    />\s*✓\s*</,
    'Checkbox must not rely on a font glyph for its checkmark',
  );

  assert.match(
    css,
    /\.glass-checkbox__box\s*\{[\s\S]*?place-items:\s*center/,
  );

  assert.match(
    css,
    /\.glass-checkbox__box\s*\{[\s\S]*?line-height:\s*0/,
  );

  assert.match(
    css,
    /\.glass-checkbox__check-icon\s*\{[\s\S]*?width:\s*14px/,
  );

  assert.match(
    css,
    /\.glass-checkbox__check-icon\s*\{[\s\S]*?height:\s*14px/,
  );

  assert.match(
    css,
    /\.glass-checkbox__check-icon\s*\{[\s\S]*?transform-origin:\s*50%\s+50%/,
  );
});
