import assert from 'node:assert/strict';
import test from 'node:test';

import {
  componentText,
  normalizeSnapshot,
  panelStatusTone,
  panelSummary,
  selectDisplayDevice,
  selectPanelDevice,
} from '../snapshot.js';

const device = (overrides = {}) => ({
  id: 'safe-id',
  displayName: 'AirPods Pro',
  connectionState: 'connected',
  components: [],
  ...overrides,
});

test('malformed or unsupported snapshots degrade to an unavailable state', () => {
  assert.deepEqual(normalizeSnapshot('{broken'), {
    schemaVersion: 1,
    generatedAt: null,
    backend: null,
    devices: [],
    preferredDeviceId: null,
  });
  assert.deepEqual(normalizeSnapshot(JSON.stringify({ schemaVersion: 99 })), {
    schemaVersion: 1,
    generatedAt: null,
    backend: null,
    devices: [],
    preferredDeviceId: null,
  });
});

test('connected devices win over a disconnected preference', () => {
  const first = device({ id: 'first', connectionState: 'disconnected' });
  const connected = device({ id: 'connected' });
  const preferred = device({ id: 'preferred', connectionState: 'disconnected' });

  assert.equal(selectDisplayDevice([first, connected, preferred], 'preferred')?.id, 'connected');
  assert.equal(selectDisplayDevice([first, connected, preferred], '')?.id, 'connected');
  assert.equal(selectDisplayDevice([first], 'first')?.id, 'first');
});

test('an active preferred device still wins when activity is otherwise equal', () => {
  const first = device({ id: 'first' });
  const preferred = device({ id: 'preferred' });

  assert.equal(selectDisplayDevice([first, preferred], 'preferred')?.id, 'preferred');
});

test('the freshest connected battery sender wins over a stale connected preference', () => {
  const oldPreferred = device({
    id: 'old',
    lastUpdatedAt: '2026-08-01T17:00:00Z',
    components: [{percentage: 70, stale: true}],
  });
  const current = device({
    id: 'current',
    lastUpdatedAt: '2026-08-01T17:45:00Z',
    components: [{percentage: 68, stale: false}],
  });

  assert.equal(selectDisplayDevice([oldPreferred, current], 'old')?.id, 'current');
});


test('fresh exact battery evidence wins when BlueZ temporarily reports disconnected', () => {
  const oldPreferred = device({
    id: 'old',
    displayName: 'ACEFAST N2',
    connectionState: 'disconnected',
    components: [],
    lastUpdatedAt: null,
  });
  const current = device({
    id: 'current',
    displayName: 'SHABIN',
    connectionState: 'disconnected',
    lastUpdatedAt: '2026-08-02T06:50:00Z',
    components: [{
      componentType: 'left',
      percentage: 71,
      stale: false,
      chargingState: 'notCharging',
      source: 'vendorProtocol',
      confidence: 'verified',
      updatedAt: '2026-08-02T06:50:00Z',
    }],
  });

  assert.equal(selectDisplayDevice([oldPreferred, current], 'old')?.id, 'current');
});



test('panel selection hides the extension when no active Bluetooth device exists', () => {
  const disconnected = device({id: 'old', connectionState: 'disconnected'});
  const unknown = device({id: 'unknown', connectionState: 'unknown'});

  assert.equal(selectPanelDevice([disconnected, unknown], 'old'), null);
});

test('panel selection becomes visible for a connected device or fresh exact accessory evidence', () => {
  const connected = device({id: 'connected', connectionState: 'connected'});
  const exact = device({
    id: 'exact',
    connectionState: 'disconnected',
    lastUpdatedAt: '2026-08-02T10:00:00Z',
    components: [{
      componentType: 'left',
      percentage: 68,
      stale: false,
      chargingState: 'notCharging',
      source: 'vendorProtocol',
      confidence: 'verified',
      updatedAt: '2026-08-02T10:00:00Z',
    }],
  });

  assert.equal(selectPanelDevice([connected], '')?.id, 'connected');
  assert.equal(selectPanelDevice([exact], '')?.id, 'exact');
});
test('component text preserves zero, unavailable, stale, and charging states', () => {
  assert.equal(componentText({ percentage: 0, stale: false, chargingState: 'notCharging' }), '0%');
  assert.equal(componentText({ percentage: null, stale: false, chargingState: 'unknown' }), 'Unavailable');
  assert.equal(componentText({ percentage: 70, stale: true, chargingState: 'unknown' }), '70% · stale');
  assert.equal(componentText({ percentage: 80, stale: false, chargingState: 'charging' }), '80% · charging');
});

test('compact percentage prefers aggregate/headset and otherwise uses the lowest fresh earbud', async () => {
  const {compactPercentage} = await import('../snapshot.js');
  const component = (componentType, percentage, stale = false) => ({
    componentType,
    percentage,
    stale,
    chargingState: 'unknown',
  });

  assert.equal(compactPercentage(device({
    components: [component('left', 65), component('right', 42), component('case', 90)],
  })), 42);
  assert.equal(compactPercentage(device({
    components: [component('left', 65), component('aggregate', 58)],
  })), 58);
  assert.equal(compactPercentage(device({
    components: [component('left', 65, true), component('case', 90)],
  })), null);
});

test('snapshot normalization preserves only a string preferred device identifier', () => {
  assert.equal(normalizeSnapshot({
    schemaVersion: 1,
    generatedAt: '2026-07-29T12:00:00Z',
    backend: {},
    devices: [],
    preferredDeviceId: 'safe-preferred',
  }).preferredDeviceId, 'safe-preferred');
  assert.equal(normalizeSnapshot({
    schemaVersion: 1,
    preferredDeviceId: 42,
  }).preferredDeviceId, null);
});

test('snapshot normalization rejects malformed devices and unsafe battery percentages', () => {
  const snapshot = normalizeSnapshot({
    schemaVersion: 1,
    devices: [
      null,
      {id: '', displayName: 'Missing identifier', connectionState: 'connected', components: []},
      {id: 'missing-name', displayName: '', connectionState: 'connected', components: []},
      {
        id: 'valid',
        displayName: 'Validated headset',
        connectionState: 'connected',
        components: [
          {componentType: 'aggregate', percentage: 150, chargingState: 'unknown', stale: false},
          {componentType: 'headset', percentage: 0, chargingState: 'notCharging', stale: false},
        ],
      },
    ],
  });

  assert.equal(snapshot.devices.length, 1);
  assert.equal(snapshot.devices[0].id, 'valid');
  assert.deepEqual(snapshot.devices[0].components.map(component => component.percentage), [0]);
});


test('panel summary exposes separate earbuds and case values when available', () => {
  const component = (componentType, percentage) => ({
    componentType, percentage, stale: false, chargingState: 'unknown',
  });

  assert.equal(panelSummary(device({
    deviceFamily: 'airPods',
    components: [component('left', 82), component('right', 79), component('case', 64)],
  })), 'L 82%  R 79%  C 64%');
});

test('panel summary keeps a missing AirPods case visible and uses one aggregate value otherwise', () => {
  const component = (componentType, percentage) => ({
    componentType, percentage, stale: false, chargingState: 'unknown',
  });

  assert.equal(panelSummary(device({
    deviceFamily: 'airPods',
    components: [component('left', 82), component('right', 79)],
  })), 'L 82%  R 79%  C —');
  assert.equal(panelSummary(device({
    deviceFamily: 'earbuds',
    components: [component('aggregate', 100)],
  })), '100%');
  assert.equal(panelSummary(device({
    deviceFamily: 'airPods',
    components: [component('aggregate', 88)],
  })), '88%');
});


test('passive AirPods BLE values are marked approximate in panel text', () => {
  const approximate = {
    componentType: 'left',
    percentage: 70,
    source: 'airPodsAdvertisement',
    confidence: 'medium',
    stale: false,
    chargingState: 'notCharging',
  };
  assert.equal(componentText(approximate), '≈70%');
  assert.equal(panelSummary(device({deviceFamily: 'airPods', components: [
    approximate,
    {...approximate, componentType: 'right'},
    {...approximate, componentType: 'case', percentage: 10},
  ]})), 'L ≈70%  R ≈70%  C ≈10%');
});


test('panel status colors the logo by the selected device battery state', () => {
  const component = percentage => ({
    componentType: 'aggregate',
    percentage,
    stale: false,
    chargingState: 'notCharging',
  });

  const healthy = device({id: 'healthy', components: [component(72)]});
  const low = device({id: 'low', components: [component(18)]});
  const critical = device({id: 'critical', components: [component(9)]});
  const unavailable = device({id: 'unknown', components: []});
  const disconnected = device({id: 'offline', connectionState: 'disconnected'});

  assert.equal(panelStatusTone([healthy], healthy), 'connected');
  assert.equal(panelStatusTone([low], low), 'low');
  assert.equal(panelStatusTone([critical], critical), 'critical');
  assert.equal(panelStatusTone([unavailable], unavailable), 'unavailable');
  assert.equal(panelStatusTone([disconnected], disconnected), 'disconnected');
});

test('a critical active device overrides the selected device status color', () => {
  const component = percentage => ({
    componentType: 'aggregate',
    percentage,
    stale: false,
    chargingState: 'notCharging',
  });
  const selected = device({id: 'selected', components: [component(82)]});
  const critical = device({id: 'critical', components: [component(7)]});

  assert.equal(panelStatusTone([selected, critical], selected), 'critical');
});

test('stale values never color the panel logo as low or critical', () => {
  const selected = device({
    id: 'selected',
    components: [{
      componentType: 'aggregate',
      percentage: 4,
      stale: true,
      chargingState: 'notCharging',
    }],
  });

  assert.equal(panelStatusTone([selected], selected), 'unavailable');
});
