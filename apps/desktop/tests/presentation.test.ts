import assert from 'node:assert/strict';
import test from 'node:test';

import {
  effectiveConnectionState,
  presentBatteryComponent,
  selectPreferredDevice,
  selectDashboardDevices,
  summarizeDevice,
} from '../src/domain/presentation.ts';
import type { BatteryComponent, BluetoothAudioDevice } from '../src/domain/types.ts';

const observedAt = '2026-07-29T12:00:00Z';

function component(overrides: Partial<BatteryComponent> = {}): BatteryComponent {
  return {
    componentType: 'left',
    percentage: 80,
    chargingState: 'notCharging',
    updatedAt: observedAt,
    source: 'airPodsAdvertisement',
    confidence: 'high',
    stale: false,
    ...overrides,
  };
}

function device(overrides: Partial<BluetoothAudioDevice> = {}): BluetoothAudioDevice {
  return {
    id: 'device-1',
    displayName: 'AirPods Pro',
    systemName: null,
    manufacturer: 'Apple',
    model: 'AirPods Pro',
    deviceFamily: 'airPods',
    transport: 'dual',
    connectionState: 'connected',
    lastSeenAt: observedAt,
    lastUpdatedAt: observedAt,
    capabilities: ['earbudBattery', 'caseBattery', 'chargingState'],
    components: [component()],
    ...overrides,
  };
}

test('zero is displayed as a known value rather than unavailable', () => {
  const result = presentBatteryComponent(component({ percentage: 0 }), 'connected');
  assert.equal(result.valueText, '0%');
  assert.equal(result.available, true);
});

test('missing percentage stays unavailable and has an accessible label', () => {
  const result = presentBatteryComponent(component({ percentage: null }), 'connected');
  assert.equal(result.valueText, '—');
  assert.equal(result.available, false);
  assert.match(result.ariaLabel, /unavailable/i);
});

test('stale and disconnected values remain visible but are labelled last known', () => {
  const result = presentBatteryComponent(
    component({ percentage: 50, stale: true }),
    'disconnected',
  );
  assert.equal(result.valueText, '50%');
  assert.equal(result.tone, 'stale');
  assert.match(result.statusText, /last known/i);
});

test('charging state is exposed with text not color alone', () => {
  const result = presentBatteryComponent(
    component({ chargingState: 'charging' }),
    'connected',
  );
  assert.equal(result.statusText, 'Charging');
  assert.match(result.ariaLabel, /charging/i);
});

test('connected devices win over a disconnected preference', () => {
  const disconnected = device({ id: 'old', connectionState: 'disconnected' });
  const connected = device({ id: 'live', connectionState: 'connected' });
  assert.equal(selectPreferredDevice([disconnected, connected], 'old')?.id, 'live');
  assert.equal(selectPreferredDevice([disconnected, connected], null)?.id, 'live');
});


test('freshest connected battery sender wins over an older connected preference', () => {
  const oldPreferred = device({
    id: 'old',
    lastUpdatedAt: '2026-08-01T17:00:00Z',
    components: [component({ stale: true })],
  });
  const current = device({
    id: 'current',
    lastUpdatedAt: '2026-08-01T17:45:00Z',
    components: [component({ percentage: 68, stale: false })],
  });

  assert.equal(selectPreferredDevice([oldPreferred, current], 'old')?.id, 'current');
});

test('device summary preserves component identity and unavailable case', () => {
  const summary = summarizeDevice(
    device({
      components: [
        component({ componentType: 'left', percentage: 70 }),
        component({ componentType: 'right', percentage: 80 }),
        component({ componentType: 'case', percentage: null }),
      ],
    }),
  );
  assert.equal(summary, 'L 70% · R 80% · C —');
});


test('passive AirPods advertisements are labelled approximate rather than exact', () => {
  const result = presentBatteryComponent(
    component({ percentage: 70, source: 'airPodsAdvertisement', confidence: 'medium' }),
    'connected',
  );
  assert.equal(result.valueText, '≈70%');
  assert.match(result.statusText, /approximate/i);
  assert.match(result.ariaLabel, /approximate/i);
});

test('active vendor protocol values preserve exact one-percent readings', () => {
  const result = presentBatteryComponent(
    component({ percentage: 71, source: 'vendorProtocol', confidence: 'verified' }),
    'connected',
  );
  assert.equal(result.valueText, '71%');
  assert.doesNotMatch(result.statusText, /approximate/i);
});


test('dashboard shows every active device without presenting a disconnected fallback as active', () => {
  const disconnected = device({ id: 'off', connectionState: 'disconnected' });
  const first = device({ id: 'one', connectionState: 'connected' });
  const second = device({ id: 'two', connectionState: 'connected' });

  assert.deepEqual(
    selectDashboardDevices([disconnected, first, second], null).map((item) => item.id),
    ['one', 'two'],
  );
  assert.deepEqual(
    selectDashboardDevices([
      device({ id: 'old', lastUpdatedAt: '2026-08-01T17:00:00Z', components: [component({ stale: true })] }),
      device({ id: 'current', lastUpdatedAt: '2026-08-01T17:45:00Z' }),
    ], 'old').map((item) => item.id),
    ['current', 'old'],
  );
  assert.deepEqual(
    selectDashboardDevices([disconnected], 'off').map((item) => item.id),
    [],
  );
});


test('fresh exact accessory evidence promotes AirPods when BlueZ connection state lags', () => {
  const oldPreferred = device({
    id: 'old-headset',
    displayName: 'ACEFAST N2',
    connectionState: 'disconnected',
    components: [],
    lastUpdatedAt: null,
  });
  const currentAirPods = device({
    id: 'current-airpods',
    displayName: 'SHABIN',
    connectionState: 'disconnected',
    lastUpdatedAt: '2026-08-02T06:50:00Z',
    components: [component({
      percentage: 71,
      source: 'vendorProtocol',
      confidence: 'verified',
      stale: false,
      updatedAt: '2026-08-02T06:50:00Z',
    })],
  });

  assert.deepEqual(
    selectDashboardDevices([oldPreferred, currentAirPods], 'old-headset').map((item) => item.id),
    ['current-airpods'],
  );
  assert.equal(selectPreferredDevice([oldPreferred, currentAirPods], 'old-headset')?.id, 'current-airpods');
  assert.equal(effectiveConnectionState(currentAirPods), 'connected');
});
