import assert from 'node:assert/strict';
import test from 'node:test';

import {
  applyGlassPreset,
  defaultGlassPreferences,
  glassCssVariables,
  normalizeGlassPreferences,
} from '../src/domain/glass.ts';

test('invalid glass preferences fall back to the balanced system-aware defaults', () => {
  assert.deepEqual(normalizeGlassPreferences(null), defaultGlassPreferences);
  assert.deepEqual(normalizeGlassPreferences('broken'), defaultGlassPreferences);
});

test('glass preferences clamp master and per-surface intensity independently', () => {
  const normalized = normalizeGlassPreferences({
    preset: 'custom',
    intensity: 140,
    adaptive: false,
    reduceTransparency: true,
    surfaces: {
      main: -5,
      widget: 67.6,
      popup: 500,
    },
  });

  assert.equal(normalized.preset, 'custom');
  assert.equal(normalized.intensity, 100);
  assert.deepEqual(normalized.surfaces, { main: 0, widget: 68, popup: 100 });
  assert.equal(normalized.adaptive, false);
  assert.equal(normalized.reduceTransparency, true);
});

test('named presets update all surfaces with intentional information hierarchy', () => {
  const crystal = applyGlassPreset(defaultGlassPreferences, 'crystal');

  assert.equal(crystal.preset, 'crystal');
  assert.equal(crystal.intensity, 96);
  assert.ok(crystal.surfaces.main < crystal.surfaces.widget);
  assert.ok(crystal.surfaces.widget < crystal.surfaces.popup);

  const off = applyGlassPreset(crystal, 'off');
  assert.deepEqual(off.surfaces, { main: 0, widget: 0, popup: 0 });
});

test('CSS variables expose bounded blur, saturation, alpha, and surface strength', () => {
  const variables = glassCssVariables(applyGlassPreset(defaultGlassPreferences, 'deep'));

  assert.equal(variables['--glass-intensity'], '0.82');
  assert.match(variables['--glass-main-blur'], /^\d+px$/);
  assert.match(variables['--glass-widget-blur'], /^\d+px$/);
  assert.match(variables['--glass-popup-blur'], /^\d+px$/);
  assert.match(variables['--glass-saturation'], /^\d+%$/);
  assert.match(variables['--glass-main-alpha'], /^0\.\d+$/);
});

test('stronger glass becomes clearer while increasing blur and edge depth', () => {
  const balanced = glassCssVariables(applyGlassPreset(defaultGlassPreferences, 'balanced'));
  const crystal = glassCssVariables(applyGlassPreset(defaultGlassPreferences, 'crystal'));

  assert.ok(Number.parseFloat(crystal['--glass-main-alpha']) < Number.parseFloat(balanced['--glass-main-alpha']));
  assert.ok(Number.parseInt(crystal['--glass-main-blur'], 10) > Number.parseInt(balanced['--glass-main-blur'], 10));
  assert.ok(Number.parseFloat(crystal['--glass-main-stroke']) > Number.parseFloat(balanced['--glass-main-stroke']));
});
