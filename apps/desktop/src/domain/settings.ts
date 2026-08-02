import type { AppSettings } from './types.ts';

export const defaultSettings: AppSettings = {
  schemaVersion: 1,
  startWithSystem: false,
  runInBackground: true,
  showTrayIcon: true,
  enableGnomeIntegration: true,
  enableDesktopWidget: false,
  updateMode: 'notify',
  preferredDeviceId: null,
  hiddenDeviceIds: [],
  appearance: {
    theme: 'system',
    transparency: 78,
    compactLayout: false,
    animations: true,
    reducedMotion: false,
  },
  notifications: {
    enabled: true,
    lowThreshold: 20,
    criticalThreshold: 10,
    componentSpecific: true,
    cooldownMinutes: 30,
    connectionEvents: false,
  },
};

type UnknownRecord = Record<string, unknown>;

function isRecord(value: unknown): value is UnknownRecord {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function bool(value: unknown, fallback: boolean): boolean {
  return typeof value === 'boolean' ? value : fallback;
}

function textOrNull(value: unknown, fallback: string | null): string | null {
  return typeof value === 'string' && value.trim().length > 0 ? value : fallback;
}

function integerInRange(value: unknown, fallback: number, minimum: number, maximum: number): number {
  if (typeof value !== 'number' || !Number.isFinite(value)) return fallback;
  return Math.min(maximum, Math.max(minimum, Math.round(value)));
}

function cloneDefaults(): AppSettings {
  return structuredClone(defaultSettings);
}

export function normalizeSettings(input: unknown): AppSettings {
  if (!isRecord(input)) return cloneDefaults();

  const appearance = isRecord(input.appearance) ? input.appearance : {};
  const notifications = isRecord(input.notifications) ? input.notifications : {};
  const theme = appearance.theme;
  const normalizedTheme = theme === 'light' || theme === 'dark' || theme === 'system' ? theme : 'system';
  const lowThreshold = integerInRange(
    notifications.lowThreshold,
    defaultSettings.notifications.lowThreshold,
    0,
    100,
  );
  const criticalCandidate = integerInRange(
    notifications.criticalThreshold,
    defaultSettings.notifications.criticalThreshold,
    0,
    100,
  );
  const hiddenDeviceIds = Array.isArray(input.hiddenDeviceIds)
    ? [...new Set(input.hiddenDeviceIds.filter(
        (id): id is string => typeof id === 'string' && id.trim().length > 0,
      ))]
    : [];

  return {
    schemaVersion: 1,
    startWithSystem: bool(input.startWithSystem, defaultSettings.startWithSystem),
    runInBackground: bool(input.runInBackground, defaultSettings.runInBackground),
    showTrayIcon: bool(input.showTrayIcon, defaultSettings.showTrayIcon),
    enableGnomeIntegration: bool(
      input.enableGnomeIntegration,
      defaultSettings.enableGnomeIntegration,
    ),
    enableDesktopWidget: bool(
      input.enableDesktopWidget,
      defaultSettings.enableDesktopWidget,
    ),
    updateMode:
      input.updateMode === 'automatic' || input.updateMode === 'manual' || input.updateMode === 'notify'
        ? input.updateMode
        : defaultSettings.updateMode,
    preferredDeviceId: textOrNull(input.preferredDeviceId, null),
    hiddenDeviceIds,
    appearance: {
      theme: normalizedTheme,
      transparency: integerInRange(
        appearance.transparency,
        defaultSettings.appearance.transparency,
        0,
        100,
      ),
      compactLayout: bool(appearance.compactLayout, defaultSettings.appearance.compactLayout),
      animations: bool(appearance.animations, defaultSettings.appearance.animations),
      reducedMotion: bool(appearance.reducedMotion, defaultSettings.appearance.reducedMotion),
    },
    notifications: {
      enabled: bool(notifications.enabled, defaultSettings.notifications.enabled),
      lowThreshold,
      criticalThreshold: Math.min(criticalCandidate, lowThreshold),
      componentSpecific: bool(
        notifications.componentSpecific,
        defaultSettings.notifications.componentSpecific,
      ),
      cooldownMinutes: integerInRange(
        notifications.cooldownMinutes,
        defaultSettings.notifications.cooldownMinutes,
        1,
        1440,
      ),
      connectionEvents: bool(
        notifications.connectionEvents,
        defaultSettings.notifications.connectionEvents,
      ),
    },
  };
}
