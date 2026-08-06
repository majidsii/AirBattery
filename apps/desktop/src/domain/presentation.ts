import { hasArtworkKey } from './artwork-keys.ts';
import type {
  BatteryComponent,
  BluetoothAudioDevice,
  ComponentType,
  ConnectionState,
} from './types.ts';

export type BatteryLayoutMode = 'splitThree' | 'splitTwo' | 'single';
export type ArtworkMode = 'component' | 'pairedEarbuds' | 'device';


export type DeviceArtworkVariant =
  | 'airpodsPro'
  | 'airpodsClassic'
  | 'galaxyBuds'
  | 'pixelBuds'
  | 'stemEarbuds'
  | 'genericEarbuds'
  | 'headset'
  | 'speaker'
  | 'mouse'
  | 'keyboard'
  | 'controller'
  | 'stylus'
  | 'bluetooth';

/** Maps stable backend artwork keys to deliberately distinct silhouettes. */
export function resolveArtworkVariant(artworkKey: string): DeviceArtworkVariant {
  const key = artworkKey.toLowerCase();
  if (key === 'airpods-pro-1' || key === 'airpods-pro-2' || key === 'airpods-pro-3') {
    return 'airpodsPro';
  }
  if (key.startsWith('airpods-') && key !== 'airpods-max') return 'airpodsClassic';
  if (key === 'galaxy-buds') return 'galaxyBuds';
  if (key === 'pixel-buds') return 'pixelBuds';
  if (key === 'soundcore-earbuds') return 'stemEarbuds';
  if (hasArtworkKey(key)) {
    return key.includes('max') || key.includes('headset') || key.includes('h-series')
      ? 'headset'
      : 'genericEarbuds';
  }
  if (key.includes('earbud') || key.includes('buds')) return 'genericEarbuds';
  if (key.includes('headset') || key.includes('headphone') || key.includes('airpods-max')) {
    return 'headset';
  }
  if (key.includes('speaker')) return 'speaker';
  if (key.includes('mouse')) return 'mouse';
  if (key.includes('keyboard')) return 'keyboard';
  if (key.includes('controller') || key.includes('gamepad')) return 'controller';
  if (key.includes('stylus') || key.includes('pencil')) return 'stylus';
  return 'bluetooth';
}

export interface BatteryLayoutSlot {
  componentType: ComponentType;
  component: BatteryComponent;
}

export interface BatteryLayout {
  mode: BatteryLayoutMode;
  artworkMode: ArtworkMode;
  slots: BatteryLayoutSlot[];
}

export type BatteryTone = 'normal' | 'charging' | 'warning' | 'critical' | 'stale' | 'unavailable';

export interface PresentedBatteryComponent {
  valueText: string;
  statusText: string;
  ariaLabel: string;
  available: boolean;
  tone: BatteryTone;
}

const componentNames: Record<ComponentType, string> = {
  left: 'Left earbud',
  right: 'Right earbud',
  case: 'Charging case',
  headset: 'Headset',
  aggregate: 'Combined battery',
  unknown: 'Battery',
};

const compactLabels: Record<ComponentType, string> = {
  left: 'L',
  right: 'R',
  case: 'C',
  headset: 'H',
  aggregate: 'A',
  unknown: 'B',
};

const defaultArtworkByFamily: Record<BluetoothAudioDevice['deviceFamily'], string> = {
  airPods: 'airpods-generic',
  earbuds: 'earbuds-generic',
  headset: 'headset-generic',
  speaker: 'speaker-generic',
  mouse: 'mouse-generic',
  keyboard: 'keyboard-generic',
  gameController: 'game-controller-generic',
  stylus: 'stylus-generic',
  genericBle: 'bluetooth',
  unknown: 'bluetooth',
};

function inferredArtworkKey(identity: string): string | null {
  const matches = (...needles: string[]) => needles.some((needle) => identity.includes(needle));
  if (matches('airpods pro 3', 'airpods pro (3rd')) return 'airpods-pro-3';
  if (matches('airpods pro (1st', 'airpods pro 1')) return 'airpods-pro-1';
  if (identity.includes('airpods pro')) return 'airpods-pro-2';
  if (identity.includes('airpods max 2')) return 'airpods-max-2';
  if (identity.includes('airpods max')) return 'airpods-max';
  if (matches('airpods 4', 'airpods (4th')) return 'airpods-4';
  if (matches('airpods 3', 'airpods (3rd')) return 'airpods-3';
  if (matches('airpods 2', 'airpods (2nd')) return 'airpods-2';
  if (identity.includes('airpods')) return 'airpods-generic';
  if (identity.includes('qcy')) {
    if (matches('qcy sp7', 'qcy speaker')) return 'speaker-generic';
    if (matches('qcy watch', 'watch gs2', 'watch gt2', 'urban gs', 'active gt')) return 'bluetooth';
    if (matches('heroad', 'vt200', 'v200', 'gt2 tri-mode')) return 'qcy-heroad';

    const qcyModels: ReadonlyArray<readonly [readonly string[], string]> = [
      [['crossky c50'], 'qcy-crossky-c50'],
      [['crossky c30s'], 'qcy-crossky-c30s'],
      [['crossky c30'], 'qcy-crossky-c30'],
      [['crossky c10'], 'qcy-crossky-c10'],
      [['crossky r70'], 'qcy-crossky-r70'],
      [['crossky link'], 'qcy-crossky-link'],
      [['crossky gtr2'], 'qcy-crossky-gtr2'],
      [['melobuds n70'], 'qcy-melobuds-n70'],
      [['melobuds n60'], 'qcy-melobuds-n60'],
      [['melobuds a30'], 'qcy-melobuds-a30'],
      [['melobuds n20'], 'qcy-melobuds-n20'],
      [['melobuds n50'], 'qcy-melobuds-n50'],
      [['melobuds n65'], 'qcy-melobuds-n65'],
      [['melobuds neo'], 'qcy-melobuds-neo'],
      [['melobuds anc'], 'qcy-melobuds-anc'],
      [['melobuds pro'], 'qcy-melobuds-pro'],
      [['t13 anc 2'], 'qcy-t13-anc-2'],
      [['t13 pro'], 'qcy-t13-pro'],
      [['t13 anc'], 'qcy-t13-anc'],
      [['t13x'], 'qcy-t13x'],
      [['t17'], 'qcy-t17'],
      [['ailybuds pro+'], 'qcy-ailybuds-pro-plus'],
      [['ailybuds e10'], 'qcy-ailybuds-e10'],
      [['ailybuds clear'], 'qcy-ailybuds-clear'],
      [['qcy air'], 'qcy-air'],
      [['qcy buds anc'], 'qcy-buds-anc'],
      [['qcy buds'], 'qcy-buds'],
      [['arcbuds lite'], 'qcy-arcbuds-lite'],
      [['arcbuds'], 'qcy-arcbuds'],
      [['h3 pro'], 'qcy-h3-pro'],
      [['h3 lite'], 'qcy-h3-lite'],
      [['h3s'], 'qcy-h3s'],
      [['qcy h3'], 'qcy-h3'],
      [['h2 pro'], 'qcy-h2-pro'],
    ];
    for (const [needles, key] of qcyModels) {
      if (needles.every((needle) => identity.includes(needle))) return key;
    }
    if (identity.includes('crossky')) return 'qcy-crossky';
    if (identity.includes('ailybuds')) return 'qcy-ailybuds';
    if (identity.includes('melobuds')) return 'qcy-melobuds';
    return 'qcy-t13';
  }
  if (identity.includes('xiaomi buds')) return identity.includes('buds 6') ? 'xiaomi-buds-6' : 'xiaomi-buds-5';
  if (identity.includes('redmi buds')) {
    for (const generation of ['8', '6', '5', '4', '3']) {
      if (identity.includes(`buds ${generation}`)) return `redmi-buds-${generation}`;
    }
    return 'redmi-buds';
  }
  if (identity.includes('soundcore') || identity.includes('liberty')) {
    if (matches('sport x20', 'sport-x20')) return 'soundcore-sport-x20';
    if (identity.includes('aerofit')) return 'soundcore-aerofit';
    if (identity.includes('space a40')) return 'soundcore-space-a40';
    if (identity.includes('sleep')) return 'soundcore-sleep';
    if (identity.includes('liberty')) return 'soundcore-liberty';
    return 'soundcore-earbuds';
  }
  if (identity.includes('galaxy buds')) {
    if (identity.includes('buds4')) return 'galaxy-buds4';
    if (identity.includes('buds3')) return 'galaxy-buds3';
    if (identity.includes('live')) return 'galaxy-buds-live';
    if (identity.includes('fe')) return 'galaxy-buds-fe';
    if (identity.includes('buds2')) return 'galaxy-buds2';
    return 'galaxy-buds';
  }
  if (matches('linkbuds open', 'wf-l910')) return 'sony-linkbuds-open';
  if (matches('linkbuds fit', 'wf-ls910')) return 'sony-linkbuds-fit';
  if (identity.includes('wf-1000xm5')) return 'sony-wf-1000xm5';
  if (identity.includes('wf-1000xm4')) return 'sony-wf-1000xm4';
  if (identity.includes('jbl')) {
    if (matches(
      'partybox',
      'boombox',
      'xtreme',
      'flip ',
      'charge ',
      'pulse ',
      'clip ',
      'jbl go',
      'encore',
      'authentics',
    )) return 'speaker-generic';
    if (matches('jbl free truly', 'jbl free')) return 'jbl-free';
    if (identity.includes('tour pro')) return 'jbl-tour-pro';
    if (identity.includes('live beam')) return 'jbl-live-beam';
    if (identity.includes('tune beam')) return 'jbl-tune-beam';
    if (identity.includes('live buds')) return 'jbl-live-buds';
    if (identity.includes('tune buds')) return 'jbl-tune-buds';
    return 'jbl-earbuds';
  }
  if (identity.includes('pixel buds')) return identity.includes('2a') ? 'pixel-buds-2a' : identity.includes('pro') ? 'pixel-buds-pro' : 'pixel-buds';
  if (identity.includes('nothing ear')) {
    if (identity.includes('open')) return 'nothing-ear-open';
    if (identity.includes('(a)')) return 'nothing-ear-a';
    if (matches('nothing ear (1)', 'nothing ear 1')) return 'nothing-ear-1';
    return 'nothing-ear';
  }
  if (identity.includes('oneplus') && identity.includes('buds')) return identity.includes('nord') ? 'oneplus-nord-buds' : identity.includes('pro') ? 'oneplus-buds-pro' : 'oneplus-buds';
  if (identity.includes('freeclip')) return 'huawei-freeclip';
  if (identity.includes('freebuds')) {
    if (identity.includes('freebuds 6')) return 'huawei-freebuds-6';
    return identity.includes('pro') ? 'huawei-freebuds-pro' : 'huawei-freebuds';
  }
  if (identity.includes('beats fit pro')) return 'beats-fit-pro';
  if (identity.includes('beats studio buds')) {
    if (matches('beats studio buds plus', 'beats studio buds +')) return 'beats-studio-buds-plus';
    return 'beats-studio-buds';
  }
  return null;
}

export function selectDeviceArtwork(device: BluetoothAudioDevice): string {
  if (device.visual?.key) return device.visual.key;

  const identity = [device.manufacturer, device.model, device.displayName, device.systemName]
    .filter((value): value is string => Boolean(value))
    .join(' ')
    .toLowerCase();

  return inferredArtworkKey(identity) ?? defaultArtworkByFamily[device.deviceFamily];
}

function unavailableComponent(
  componentType: ComponentType,
  template: BatteryComponent | undefined,
): BatteryComponent {
  return {
    componentType,
    percentage: null,
    chargingState: 'unknown',
    updatedAt: template?.updatedAt ?? new Date(0).toISOString(),
    source: template?.source ?? 'aggregatePlatform',
    confidence: 'unknown',
    stale: template?.stale ?? false,
  };
}

function slot(
  componentType: ComponentType,
  components: ReadonlyMap<ComponentType, BatteryComponent>,
  template: BatteryComponent | undefined,
): BatteryLayoutSlot {
  return {
    componentType,
    component: components.get(componentType) ?? unavailableComponent(componentType, template),
  };
}

/**
 * Converts raw battery components into a truthful visual layout.
 * Missing component percentages are represented as unavailable, never copied or inferred.
 */
export function buildBatteryLayout(device: BluetoothAudioDevice): BatteryLayout {
  const components = new Map(
    device.components.map((component) => [component.componentType, component] as const),
  );
  const template = device.components[0];

  const hasSplitComponent =
    components.has('left') || components.has('right') || components.has('case');

  if (device.deviceFamily === 'airPods' && (hasSplitComponent || components.size === 0)) {
    return {
      mode: 'splitThree',
      artworkMode: 'component',
      slots: [
        slot('left', components, template),
        slot('right', components, template),
        slot('case', components, template),
      ],
    };
  }

  if (components.has('left') && components.has('right')) {
    const componentTypes: ComponentType[] = components.has('case')
      ? ['left', 'right', 'case']
      : ['left', 'right'];
    return {
      mode: componentTypes.length === 3 ? 'splitThree' : 'splitTwo',
      artworkMode: 'component',
      slots: componentTypes.map((componentType) => slot(componentType, components, template)),
    };
  }

  const singleType: ComponentType = components.has('aggregate')
    ? 'aggregate'
    : components.has('headset')
      ? 'headset'
      : components.has('unknown')
        ? 'unknown'
        : device.components[0]?.componentType ?? 'unknown';

  return {
    mode: 'single',
    artworkMode:
      device.deviceFamily === 'airPods' || device.deviceFamily === 'earbuds'
        ? 'pairedEarbuds'
        : 'device',
    slots: [slot(singleType, components, template)],
  };
}

function isApproximateAirPodsValue(component: BatteryComponent): boolean {
  return component.source === 'airPodsAdvertisement' &&
    (component.confidence === 'medium' || component.confidence === 'low');
}

export function presentBatteryComponent(
  component: BatteryComponent,
  connectionState: ConnectionState,
): PresentedBatteryComponent {
  const name = componentNames[component.componentType];
  const available = component.percentage !== null;
  const approximate = available && isApproximateAirPodsValue(component);
  const valueText = available
    ? `${approximate ? '≈' : ''}${component.percentage}%`
    : '—';
  const disconnected = connectionState === 'disconnected';
  const stale = component.stale || disconnected;
  const approximationText = approximate ? ' · approximate BLE value' : '';
  const approximationAria = approximate ? ', approximate Bluetooth value' : '';

  if (!available) {
    return {
      valueText,
      statusText: disconnected ? 'Disconnected · unavailable' : 'Unavailable',
      ariaLabel: `${name} battery unavailable`,
      available: false,
      tone: 'unavailable',
    };
  }

  if (stale) {
    return {
      valueText,
      statusText: disconnected
        ? `Disconnected · last known${approximationText}`
        : `Last known value${approximationText}`,
      ariaLabel: `${name} ${valueText}, last known value${approximationAria}`,
      available: true,
      tone: 'stale',
    };
  }

  if (component.chargingState === 'charging') {
    return {
      valueText,
      statusText: `Charging${approximationText}`,
      ariaLabel: `${name} ${valueText}, charging${approximationAria}`,
      available: true,
      tone: 'charging',
    };
  }

  if (component.chargingState === 'full') {
    return {
      valueText,
      statusText: `Full${approximationText}`,
      ariaLabel: `${name} ${valueText}, full${approximationAria}`,
      available: true,
      tone: 'charging',
    };
  }

  const percentage = component.percentage ?? 0;
  const tone: BatteryTone = percentage <= 10 ? 'critical' : percentage <= 20 ? 'warning' : 'normal';
  const baseStatus = component.chargingState === 'notCharging' ? 'Not charging' : 'Battery level';
  return {
    valueText,
    statusText: `${baseStatus}${approximationText}`,
    ariaLabel: `${name} ${valueText}${approximationAria}`,
    available: true,
    tone,
  };
}

export function hasFreshBattery(device: BluetoothAudioDevice): boolean {
  return device.components.some(
    (component) => component.percentage !== null && component.stale !== true,
  );
}

/**
 * Treats a device with fresh normalized battery evidence as active even when
 * `BlueZ` briefly lags behind the accessory channel and reports disconnected.
 */
export function hasLiveAccessoryEvidence(device: BluetoothAudioDevice): boolean {
  return device.components.some(
    (component) =>
      component.percentage !== null &&
      component.stale !== true &&
      component.source === 'vendorProtocol' &&
      component.confidence === 'verified',
  );
}

export function effectiveConnectionState(device: BluetoothAudioDevice): ConnectionState {
  if (device.connectionState === 'connected' || device.connectionState === 'connecting') {
    return device.connectionState;
  }
  return hasLiveAccessoryEvidence(device) ? 'connected' : device.connectionState;
}

function activityTimestamp(device: BluetoothAudioDevice): number {
  const timestamp = Date.parse(device.lastUpdatedAt ?? '');
  return Number.isFinite(timestamp) ? timestamp : Number.NEGATIVE_INFINITY;
}

function sortByCurrentActivity(
  devices: readonly BluetoothAudioDevice[],
  preferredDeviceId: string | null,
): BluetoothAudioDevice[] {
  return [...devices].sort((left, right) => {
    const freshness = Number(hasFreshBattery(right)) - Number(hasFreshBattery(left));
    if (freshness !== 0) return freshness;

    const leftTimestamp = activityTimestamp(left);
    const rightTimestamp = activityTimestamp(right);
    if (leftTimestamp !== rightTimestamp) return rightTimestamp > leftTimestamp ? 1 : -1;

    return Number(right.id === preferredDeviceId) - Number(left.id === preferredDeviceId);
  });
}

export function selectPreferredDevice(
  devices: readonly BluetoothAudioDevice[],
  preferredDeviceId: string | null,
): BluetoothAudioDevice | null {
  const active = devices.filter((device) => {
    const state = effectiveConnectionState(device);
    return state === 'connected' || state === 'connecting';
  });
  if (active.length > 0) return sortByCurrentActivity(active, preferredDeviceId)[0] ?? null;

  const preferred = preferredDeviceId
    ? devices.find((device) => device.id === preferredDeviceId)
    : undefined;
  return preferred ?? devices[0] ?? null;
}

/** Returns every currently connected device for the battery dashboard. */
export function selectDashboardDevices(
  devices: readonly BluetoothAudioDevice[],
  preferredDeviceId: string | null,
): BluetoothAudioDevice[] {
  const active = devices.filter((device) => {
    const state = effectiveConnectionState(device);
    return state === 'connected' || state === 'connecting';
  });

  return sortByCurrentActivity(active, preferredDeviceId);
}

export function summarizeDevice(device: BluetoothAudioDevice): string {
  const order: ComponentType[] = ['left', 'right', 'case', 'headset', 'aggregate', 'unknown'];
  return order
    .map((kind) => device.components.find((component) => component.componentType === kind))
    .filter((component): component is BatteryComponent => component !== undefined)
    .map((component) => {
      const approximate = component.percentage !== null && isApproximateAirPodsValue(component);
      const value = component.percentage === null
        ? '—'
        : `${approximate ? '≈' : ''}${component.percentage}%`;
      return `${compactLabels[component.componentType]} ${value}`;
    })
    .join(' · ');
}
