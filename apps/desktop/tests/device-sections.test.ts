import assert from 'node:assert/strict';
import test from 'node:test';

import {
  deviceBatteryTone,
  isActiveDevice,
  splitDeviceSections,
} from '../src/domain/device-sections.ts';
import type {
  BatteryComponent,
  BluetoothAudioDevice,
} from '../src/domain/types.ts';

const observedAt = '2026-08-06T10:00:00.000Z';

function component(
  percentage: number | null,
  chargingState: BatteryComponent['chargingState'] = 'notCharging',
  stale = false,
): BatteryComponent {
  return {
    componentType: 'aggregate',
    percentage,
    chargingState,
    updatedAt: observedAt,
    source: 'bluezBattery',
    confidence: 'high',
    stale,
  };
}

function device(
  overrides: Partial<BluetoothAudioDevice> = {},
): BluetoothAudioDevice {
  return {
    id: 'device',
    displayName: 'Device',
    systemName: null,
    manufacturer: null,
    model: null,
    deviceFamily: 'unknown',
    visual: { key: 'bluetooth', confidence: 'fallback' },
    transport: 'unknown',
    connectionState: 'disconnected',
    lastSeenAt: observedAt,
    lastUpdatedAt: observedAt,
    capabilities: [],
    components: [],
    ...overrides,
  };
}

test('only connected and connecting devices are active', () => {
  assert.equal(isActiveDevice(device({ connectionState: 'connected' })), true);
  assert.equal(isActiveDevice(device({ connectionState: 'connecting' })), true);
  assert.equal(isActiveDevice(device({ connectionState: 'disconnected' })), false);
  assert.equal(isActiveDevice(device({ connectionState: 'unknown' })), false);
});

test('device sections keep active hardware above known hardware', () => {
  const active = device({ id: 'active', connectionState: 'connected' });
  const known = device({ id: 'known', connectionState: 'disconnected' });
  const sections = splitDeviceSections([known, active]);

  assert.deepEqual(sections.active.map((item) => item.id), ['active']);
  assert.deepEqual(sections.known.map((item) => item.id), ['known']);
});

test('battery tone uses the lowest fresh component percentage', () => {
  assert.equal(deviceBatteryTone(device({
    connectionState: 'connected',
    components: [component(80, 'charging')],
  })), 'charging');

  assert.equal(deviceBatteryTone(device({
    connectionState: 'connected',
    components: [component(80), component(55)],
  })), 'healthy');

  assert.equal(deviceBatteryTone(device({
    connectionState: 'connected',
    components: [component(48), component(30)],
  })), 'medium');

  assert.equal(deviceBatteryTone(device({
    connectionState: 'connected',
    components: [component(82), component(15)],
  })), 'critical');

  assert.equal(deviceBatteryTone(device({
    connectionState: 'connected',
    components: [component(null)],
  })), 'unknown');

  assert.equal(deviceBatteryTone(device({
    connectionState: 'disconnected',
    components: [component(90)],
  })), 'disconnected');
});
