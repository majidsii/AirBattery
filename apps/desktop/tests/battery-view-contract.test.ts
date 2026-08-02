import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

const source = readFileSync(new URL('../src/views/BatteryView.vue', import.meta.url), 'utf8');

test('desktop dashboard renders every automatically selected connected device', () => {
  assert.match(source, /v-for="device in displayedDevices"/);
  assert.match(source, /<DeviceBatteryPanel\s+:device="device"/);
  assert.doesNotMatch(source, /<DeviceSwitcher/);
});

test('compact widget keeps one automatically selected device', () => {
  assert.match(source, /<template v-else-if="compactDevice"/);
  assert.match(source, /<DeviceBatteryPanel :device="compactDevice" compact/);
});
