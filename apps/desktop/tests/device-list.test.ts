import assert from 'node:assert/strict';
import test from 'node:test';

import { filterVisibleDevices, toggleHiddenDevice } from '../src/domain/device-list.ts';
import type { BluetoothAudioDevice } from '../src/domain/types.ts';

function device(id: string, overrides: Partial<BluetoothAudioDevice> = {}): BluetoothAudioDevice {
  return {
    id,
    displayName: id,
    systemName: null,
    manufacturer: null,
    model: null,
    deviceFamily: 'unknown',
    transport: 'unknown',
    connectionState: 'disconnected',
    lastSeenAt: null,
    lastUpdatedAt: null,
    capabilities: [],
    components: [],
    ...overrides,
  };
}

test('hidden device ids are filtered without mutating the input list', () => {
  const input = [device('one'), device('two')];
  const result = filterVisibleDevices(input, ['one']);

  assert.deepEqual(result.map((entry) => entry.id), ['two']);
  assert.equal(input.length, 2);
});

test('toggle hidden device is idempotent in both directions', () => {
  assert.deepEqual(toggleHiddenDevice(['one'], 'one', true), ['one']);
  assert.deepEqual(toggleHiddenDevice(['one'], 'two', true), ['one', 'two']);
  assert.deepEqual(toggleHiddenDevice(['one', 'two'], 'one', false), ['two']);
});


test('hidden devices that are connected or actively reporting battery remain visible', () => {
  const hiddenDisconnected = device('hidden-off');
  const hiddenConnected = device('hidden-connected', { connectionState: 'connected' });
  const hiddenReporter = device('hidden-reporter', {
    connectionState: 'disconnected',
    lastUpdatedAt: '2026-08-02T06:50:00Z',
    components: [{
      componentType: 'left',
      percentage: 71,
      chargingState: 'notCharging',
      updatedAt: '2026-08-02T06:50:00Z',
      source: 'vendorProtocol',
      confidence: 'verified',
      stale: false,
    }],
  });

  const result = filterVisibleDevices(
    [hiddenDisconnected, hiddenConnected, hiddenReporter],
    ['hidden-off', 'hidden-connected', 'hidden-reporter'],
  );

  assert.deepEqual(result.map((entry) => entry.id), ['hidden-connected', 'hidden-reporter']);
});
