import assert from 'node:assert/strict';
import test from 'node:test';

import {
  buildBatteryLayout,
  resolveArtworkVariant,
  selectDeviceArtwork,
} from '../src/domain/presentation.ts';
import type { BatteryComponent, BluetoothAudioDevice } from '../src/domain/types.ts';

const observedAt = '2026-07-30T12:00:00Z';

function component(
  componentType: BatteryComponent['componentType'],
  percentage: number | null,
): BatteryComponent {
  return {
    componentType,
    percentage,
    chargingState: 'unknown',
    updatedAt: observedAt,
    source: componentType === 'aggregate' ? 'bluezBattery' : 'airPodsAdvertisement',
    confidence: componentType === 'aggregate' ? 'medium' : 'high',
    stale: false,
  };
}

function device(overrides: Partial<BluetoothAudioDevice> = {}): BluetoothAudioDevice {
  return {
    id: 'device-1',
    displayName: 'Bluetooth device',
    systemName: null,
    manufacturer: null,
    model: null,
    deviceFamily: 'unknown',
    visual: { key: 'bluetooth', confidence: 'fallback' },
    transport: 'unknown',
    connectionState: 'connected',
    lastSeenAt: observedAt,
    lastUpdatedAt: observedAt,
    capabilities: [],
    components: [],
    ...overrides,
  };
}

test('full AirPods layout always exposes left right and case slots', () => {
  const layout = buildBatteryLayout(
    device({
      deviceFamily: 'airPods',
      visual: { key: 'airpods-pro-1', confidence: 'exact' },
      components: [component('left', 82), component('right', 79), component('case', null)],
    }),
  );

  assert.equal(layout.mode, 'splitThree');
  assert.deepEqual(layout.slots.map((slot) => slot.componentType), ['left', 'right', 'case']);
  assert.equal(layout.slots[2]?.component.percentage, null);
});


test('AirPods with no received payload still expose unavailable component slots', () => {
  const layout = buildBatteryLayout(
    device({
      deviceFamily: 'airPods',
      visual: { key: 'airpods-pro-1', confidence: 'exact' },
      components: [],
    }),
  );

  assert.equal(layout.mode, 'splitThree');
  assert.deepEqual(layout.slots.map((slot) => slot.component.percentage), [null, null, null]);
});

test('AirPods with only an aggregate use paired AirPods artwork and one truthful value', () => {
  const layout = buildBatteryLayout(
    device({
      deviceFamily: 'airPods',
      visual: { key: 'airpods-pro-1', confidence: 'exact' },
      components: [component('aggregate', 88)],
    }),
  );

  assert.equal(layout.mode, 'single');
  assert.equal(layout.artworkMode, 'pairedEarbuds');
  assert.deepEqual(layout.slots.map((slot) => slot.componentType), ['aggregate']);
  assert.equal(layout.slots[0]?.component.percentage, 88);
});

test('generic earbuds with only an aggregate use paired artwork and one value', () => {
  const layout = buildBatteryLayout(
    device({
      deviceFamily: 'earbuds',
      visual: { key: 'earbuds-generic', confidence: 'category' },
      components: [component('aggregate', 100)],
    }),
  );

  assert.equal(layout.mode, 'single');
  assert.equal(layout.artworkMode, 'pairedEarbuds');
  assert.equal(layout.slots.length, 1);
  assert.equal(layout.slots[0]?.component.percentage, 100);
});

test('two independent earbuds stay split without inventing a case', () => {
  const layout = buildBatteryLayout(
    device({
      deviceFamily: 'earbuds',
      components: [component('left', 65), component('right', 70)],
    }),
  );

  assert.equal(layout.mode, 'splitTwo');
  assert.deepEqual(layout.slots.map((slot) => slot.componentType), ['left', 'right']);
});

test('non-audio devices retain one real aggregate value and category artwork', () => {
  const mouse = device({
    displayName: 'MX Master 3S',
    manufacturer: 'Logitech',
    deviceFamily: 'mouse',
    visual: { key: 'mouse-generic', confidence: 'category' },
    components: [component('aggregate', 75)],
  });

  const layout = buildBatteryLayout(mouse);
  assert.equal(layout.mode, 'single');
  assert.equal(layout.artworkMode, 'device');
  assert.equal(selectDeviceArtwork(mouse), 'mouse-generic');
});

test('unknown devices use the Bluetooth fallback', () => {
  assert.equal(selectDeviceArtwork(device()), 'bluetooth');
});

test('known model names preserve exact model keys when an older backend omits visual metadata', () => {
  assert.equal(selectDeviceArtwork(device({
    displayName: 'SHABIN',
    model: 'AirPods Pro (1st generation)',
    deviceFamily: 'airPods',
    visual: undefined,
  })), 'airpods-pro-1');
  assert.equal(selectDeviceArtwork(device({
    displayName: 'Galaxy Buds2 Pro',
    deviceFamily: 'earbuds',
    visual: undefined,
  })), 'galaxy-buds2');
});


test('exact model keys remain distinct across supported product families', () => {
  assert.equal(resolveArtworkVariant('airpods-pro-1'), 'airpodsPro');
  assert.equal(resolveArtworkVariant('airpods-pro-2'), 'airpodsPro');
  assert.equal(resolveArtworkVariant('airpods-generic'), 'airpodsClassic');
  assert.equal(resolveArtworkVariant('galaxy-buds'), 'galaxyBuds');
  assert.equal(resolveArtworkVariant('pixel-buds'), 'pixelBuds');
  assert.equal(resolveArtworkVariant('soundcore-earbuds'), 'stemEarbuds');
  assert.equal(resolveArtworkVariant('earbuds-generic'), 'genericEarbuds');
});


test('older backends infer model-aware artwork for QCY and other major brands', () => {
  const fixtures = [
    ['QCY T13 ANC 2', 'qcy-t13-anc-2'],
    ['QCY Crossky C30', 'qcy-crossky-c30'],
    ['QCY MeloBuds N70', 'qcy-melobuds-n70'],
    ['QCY AilyBuds Pro+', 'qcy-ailybuds-pro-plus'],
    ['QCY H3 Pro', 'qcy-h3-pro'],
    ['QCY ArcBuds Lite', 'qcy-arcbuds-lite'],
    ['QCY Heroad VT200', 'qcy-heroad'],
    ['QCY SP7 Speaker', 'speaker-generic'],
    ['QCY Watch GS2', 'bluetooth'],
    ['Redmi Buds 6', 'redmi-buds-6'],
    ['Galaxy Buds3 Pro', 'galaxy-buds3'],
    ['Sony LinkBuds Open', 'sony-linkbuds-open'],
    ['Sony WF-1000XM4', 'sony-wf-1000xm4'],
    ['soundcore Sport X20', 'soundcore-sport-x20'],
    ['JBL Free', 'jbl-free'],
    ['JBL Tour Pro 3', 'jbl-tour-pro'],
    ['Nothing Ear (a)', 'nothing-ear-a'],
    ['Nothing Ear (1)', 'nothing-ear-1'],
    ['HUAWEI FreeBuds 6', 'huawei-freebuds-6'],
    ['Beats Studio Buds +', 'beats-studio-buds-plus'],
  ] as const;

  for (const [displayName, expected] of fixtures) {
    assert.equal(selectDeviceArtwork(device({ displayName, deviceFamily: 'earbuds', visual: undefined })), expected);
  }
});


test('JBL PartyBox models use the speaker fallback rather than earbud artwork', () => {
  assert.equal(selectDeviceArtwork(device({
    displayName: 'JBL PartyBox Club 120',
    deviceFamily: 'unknown',
    visual: undefined,
  })), 'speaker-generic');
});
