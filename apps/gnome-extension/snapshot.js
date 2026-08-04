
const COMPONENT_TYPES = new Set(['left', 'right', 'case', 'headset', 'aggregate', 'unknown']);
const CHARGING_STATES = new Set(['charging', 'notCharging', 'full', 'unknown']);
const CONNECTION_STATES = new Set(['connected', 'disconnected', 'connecting', 'unknown']);

function normalizeComponent(component) {
  if (!component || typeof component !== 'object' || !COMPONENT_TYPES.has(component.componentType)) {
    return null;
  }
  const percentage = component.percentage;
  if (percentage !== null &&
      (!Number.isInteger(percentage) || percentage < 0 || percentage > 100)) {
    return null;
  }
  return {
    ...component,
    percentage,
    chargingState: CHARGING_STATES.has(component.chargingState)
      ? component.chargingState
      : 'unknown',
    stale: component.stale === true,
  };
}

function normalizeDevice(device) {
  if (!device || typeof device !== 'object' ||
      typeof device.id !== 'string' || device.id.trim() === '' ||
      typeof device.displayName !== 'string' || device.displayName.trim() === '') {
    return null;
  }
  return {
    ...device,
    id: device.id,
    displayName: device.displayName,
    connectionState: CONNECTION_STATES.has(device.connectionState)
      ? device.connectionState
      : 'unknown',
    components: Array.isArray(device.components)
      ? device.components.map(normalizeComponent).filter(component => component !== null)
      : [],
  };
}

const EMPTY_SNAPSHOT = Object.freeze({
  schemaVersion: 1,
  generatedAt: null,
  backend: null,
  devices: [],
  preferredDeviceId: null,
});

function emptySnapshot() {
  return {
    schemaVersion: EMPTY_SNAPSHOT.schemaVersion,
    generatedAt: EMPTY_SNAPSHOT.generatedAt,
    backend: EMPTY_SNAPSHOT.backend,
    devices: [],
    preferredDeviceId: null,
  };
}

export function normalizeSnapshot(input) {
  let candidate;

  try {
    candidate = typeof input === 'string' ? JSON.parse(input) : input;
  } catch {
    return emptySnapshot();
  }

  if (!candidate || typeof candidate !== 'object' || candidate.schemaVersion !== 1) {
    return emptySnapshot();
  }

  return {
    schemaVersion: 1,
    generatedAt: typeof candidate.generatedAt === 'string' ? candidate.generatedAt : null,
    backend: candidate.backend && typeof candidate.backend === 'object'
      ? candidate.backend
      : null,
    devices: Array.isArray(candidate.devices)
      ? candidate.devices.map(normalizeDevice).filter(device => device !== null)
      : [],
    preferredDeviceId: typeof candidate.preferredDeviceId === 'string'
      ? candidate.preferredDeviceId
      : null,
  };
}

function activityTimestamp(device) {
  const parsed = Date.parse(device?.lastUpdatedAt ?? '');
  return Number.isFinite(parsed) ? parsed : Number.NEGATIVE_INFINITY;
}

function hasFreshBattery(device) {
  return Array.isArray(device?.components) && device.components.some(component =>
    component?.stale !== true && Number.isInteger(component?.percentage));
}

function hasLiveAccessoryEvidence(device) {
  return Array.isArray(device?.components) && device.components.some(component =>
    component?.stale !== true &&
    Number.isInteger(component?.percentage) &&
    component?.source === 'vendorProtocol' &&
    component?.confidence === 'verified');
}

export function effectiveConnectionState(device) {
  if (device?.connectionState === 'connected' || device?.connectionState === 'connecting') {
    return device.connectionState;
  }
  return hasLiveAccessoryEvidence(device) ? 'connected' : (device?.connectionState ?? 'unknown');
}

function selectFreshestDevice(devices, preferredDeviceId) {
  return [...devices].sort((left, right) => {
    const freshness = Number(hasFreshBattery(right)) - Number(hasFreshBattery(left));
    if (freshness !== 0) return freshness;

    const leftTimestamp = activityTimestamp(left);
    const rightTimestamp = activityTimestamp(right);
    if (leftTimestamp !== rightTimestamp) return rightTimestamp > leftTimestamp ? 1 : -1;

    return Number(right.id === preferredDeviceId) - Number(left.id === preferredDeviceId);
  })[0] ?? null;
}

export function selectPanelDevice(devices, preferredDeviceId) {
  if (!Array.isArray(devices) || devices.length === 0) {
    return null;
  }

  const active = devices.filter(device => {
    const state = effectiveConnectionState(device);
    return state === 'connected';
  });
  return active.length > 0 ? selectFreshestDevice(active, preferredDeviceId) : null;
}

export function selectDisplayDevice(devices, preferredDeviceId) {
  if (!Array.isArray(devices) || devices.length === 0) {
    return null;
  }

  const active = selectPanelDevice(devices, preferredDeviceId);
  if (active) return active;

  return devices.find(device => device.id === preferredDeviceId) ?? devices[0] ?? null;
}

function isApproximate(component) {
  return component?.source === 'airPodsAdvertisement' &&
    (component?.confidence === 'medium' || component?.confidence === 'low');
}

export function componentText(component) {
  if (!component || !Number.isInteger(component.percentage)) {
    return 'Unavailable';
  }

  const suffixes = [];
  if (component.stale === true) {
    suffixes.push('stale');
  }
  if (component.chargingState === 'charging') {
    suffixes.push('charging');
  } else if (component.chargingState === 'full') {
    suffixes.push('full');
  }

  const base = `${isApproximate(component) ? '≈' : ''}${component.percentage}%`;
  return suffixes.length > 0 ? `${base} · ${suffixes.join(' · ')}` : base;
}

export function compactPercentage(device) {
  const components = Array.isArray(device?.components) ? device.components : [];
  const freshKnown = components.filter(component =>
    component?.stale !== true && Number.isInteger(component?.percentage));

  const aggregate = freshKnown.find(component =>
    component.componentType === 'aggregate' || component.componentType === 'headset');
  if (aggregate) {
    return aggregate.percentage;
  }

  const earbuds = freshKnown
    .filter(component => component.componentType === 'left' || component.componentType === 'right')
    .map(component => component.percentage);

  return earbuds.length > 0 ? Math.min(...earbuds) : null;
}

function freshPercentages(device) {
  const components = Array.isArray(device?.components) ? device.components : [];
  return components
    .filter(component => component?.stale !== true && Number.isInteger(component?.percentage))
    .map(component => component.percentage);
}

function deviceStatusTone(device) {
  if (effectiveConnectionState(device) !== 'connected') {
    return 'disconnected';
  }

  const values = freshPercentages(device);
  if (values.length === 0) {
    return 'unavailable';
  }

  const minimum = Math.min(...values);
  if (minimum <= 10) return 'critical';
  if (minimum <= 20) return 'low';
  return 'connected';
}

/**
 * Chooses the live logo color for the GNOME panel. A critical active device
 * overrides the selected device so an urgent battery warning is never hidden.
 */
export function panelStatusTone(devices, selectedDevice) {
  const activeDevices = Array.isArray(devices)
    ? devices.filter(device => effectiveConnectionState(device) === 'connected')
    : [];
  const hasCriticalActiveDevice = activeDevices.some(device =>
    freshPercentages(device).some(percentage => percentage <= 10));

  return hasCriticalActiveDevice ? 'critical' : deviceStatusTone(selectedDevice);
}

function componentValue(components, type) {
  const component = components.find(item =>
    item?.componentType === type && item?.stale !== true && Number.isInteger(item?.percentage));
  return component
    ? `${isApproximate(component) ? '≈' : ''}${component.percentage}%`
    : '—';
}

/**
 * Produces the GNOME panel text without manufacturing per-component values.
 * AirPods retain fixed L/R/C slots so a temporarily unavailable case remains visible.
 */
export function panelSummary(device) {
  if (!device) return '';
  const components = Array.isArray(device.components) ? device.components : [];
  const hasLeft = components.some(component => component?.componentType === 'left');
  const hasRight = components.some(component => component?.componentType === 'right');
  const hasCase = components.some(component => component?.componentType === 'case');

  const hasSplitComponent = hasLeft || hasRight || hasCase;
  if ((device.deviceFamily === 'airPods' && (hasSplitComponent || components.length === 0)) ||
      (hasLeft && hasRight)) {
    const fields = [
      `L ${componentValue(components, 'left')}`,
      `R ${componentValue(components, 'right')}`,
    ];
    if (device.deviceFamily === 'airPods' || hasCase)
      fields.push(`C ${componentValue(components, 'case')}`);
    return fields.join('  ');
  }

  const percentage = compactPercentage(device);
  return percentage === null ? '—' : `${percentage}%`;
}
