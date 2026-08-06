import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

test('GlassSelect popup matches the trigger and centers an SVG check', () => {
  const component = readFileSync(
    new URL('../src/components/GlassSelect.vue', import.meta.url),
    'utf8',
  );
  const css = readFileSync(
    new URL('../src/styles/glass.css', import.meta.url),
    'utf8',
  );

  assert.match(component, /class="glass-select__check-icon"/);
  assert.doesNotMatch(component, /class="glass-select__check"[\s\S]*?>\s*✓/);
  assert.match(css, /\.glass-select__listbox\s*\{[\s\S]*?width:\s*100%/);
  assert.match(css, /\.glass-select__listbox\s*\{[\s\S]*?min-width:\s*100%/);
  assert.match(css, /\.glass-select__check\s*\{[\s\S]*?place-items:\s*center/);
});

test('DevicesView separates active and known devices with the official mark', () => {
  const view = readFileSync(
    new URL('../src/views/DevicesView.vue', import.meta.url),
    'utf8',
  );
  const css = readFileSync(
    new URL('../src/styles/glass.css', import.meta.url),
    'utf8',
  );

  assert.match(view, /import BrandMark from '\.\.\/components\/BrandMark\.vue'/);
  assert.match(view, /Active devices/);
  assert.match(view, /Known devices/);
  assert.match(view, /showKnownDevices/);
  assert.match(view, /<BrandMark decorative \/>/);
  assert.match(view, /deviceBatteryTone\(device\)/);

  for (const tone of [
    'charging',
    'healthy',
    'medium',
    'critical',
    'unknown',
    'disconnected',
  ]) {
    assert.match(css, new RegExp(`\\.device-avatar--${tone}`));
  }
});

test('selected GlassSelect indicator stays centered independent of option copy height', () => {
  const css = readFileSync(
    new URL('../src/styles/glass.css', import.meta.url),
    'utf8',
  );

  assert.match(
    css,
    /\.glass-select__option\s*\{[\s\S]*?position:\s*relative/,
  );

  assert.match(
    css,
    /\.glass-select__check\s*\{[\s\S]*?position:\s*absolute/,
  );

  assert.match(
    css,
    /\.glass-select__check\s*\{[\s\S]*?top:\s*50%/,
  );

  assert.match(
    css,
    /\.glass-select__check\s*\{[\s\S]*?inset-inline-end:\s*12px/,
  );

  assert.match(
    css,
    /\.glass-select__check\s*\{[\s\S]*?transform:\s*translateY\(-50%\)/,
  );
});
