import assert from 'node:assert/strict';
import test from 'node:test';

import { defaultSettings, normalizeSettings } from '../src/domain/settings.ts';

test('invalid persisted settings fall back safely without throwing', () => {
  assert.deepEqual(normalizeSettings(null), defaultSettings);
  assert.deepEqual(normalizeSettings('corrupt'), defaultSettings);
});

test('settings normalization clamps thresholds and removes duplicate hidden ids', () => {
  const normalized = normalizeSettings({
    ...defaultSettings,
    hiddenDeviceIds: ['one', 'one', '', 'two'],
    appearance: {
      ...defaultSettings.appearance,
      transparency: 140,
    },
    notifications: {
      ...defaultSettings.notifications,
      lowThreshold: -2,
      criticalThreshold: 75,
      cooldownMinutes: 0,
    },
  });

  assert.equal(normalized.appearance.transparency, 100);
  assert.equal(normalized.notifications.lowThreshold, 0);
  assert.equal(normalized.notifications.criticalThreshold, 0);
  assert.equal(normalized.notifications.cooldownMinutes, 1);
  assert.deepEqual(normalized.hiddenDeviceIds, ['one', 'two']);
});

test('unknown appearance values use defaults and valid values are preserved', () => {
  const normalized = normalizeSettings({
    ...defaultSettings,
    appearance: {
      ...defaultSettings.appearance,
      theme: 'neon',
      reducedMotion: true,
      transparency: 42,
    },
  });

  assert.equal(normalized.appearance.theme, 'system');
  assert.equal(normalized.appearance.reducedMotion, true);
  assert.equal(normalized.appearance.transparency, 42);
});

test('update mode accepts supported values and rejects unknown modes', () => {
  assert.equal(normalizeSettings(defaultSettings).updateMode, 'notify');
  assert.equal(normalizeSettings({ ...defaultSettings, updateMode: 'manual' }).updateMode, 'manual');
  assert.equal(normalizeSettings({ ...defaultSettings, updateMode: 'silent' }).updateMode, 'notify');
});
