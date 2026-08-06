import type { BluetoothAudioDevice } from './types.ts';

export type DeviceBatteryTone =
  | 'charging'
  | 'healthy'
  | 'medium'
  | 'critical'
  | 'unknown'
  | 'disconnected';

export interface DeviceSections {
  active: BluetoothAudioDevice[];
  known: BluetoothAudioDevice[];
}

function freshPercentages(
  device: BluetoothAudioDevice,
): number[] {
  return device.components.flatMap((component) => {
    if (
      component.stale ||
      component.percentage === null ||
      !Number.isFinite(component.percentage)
    ) {
      return [];
    }

    return [component.percentage];
  });
}

export function isActiveDevice(
  device: BluetoothAudioDevice,
): boolean {
  return (
    device.connectionState === 'connected' ||
    device.connectionState === 'connecting'
  );
}

export function splitDeviceSections(
  devices: readonly BluetoothAudioDevice[],
): DeviceSections {
  const active: BluetoothAudioDevice[] = [];
  const known: BluetoothAudioDevice[] = [];

  for (const device of devices) {
    if (isActiveDevice(device)) active.push(device);
    else known.push(device);
  }

  return { active, known };
}

export function deviceBatteryTone(
  device: BluetoothAudioDevice,
): DeviceBatteryTone {
  if (!isActiveDevice(device)) return 'disconnected';

  const freshComponents = device.components.filter(
    (component) => !component.stale,
  );

  if (
    freshComponents.some(
      (component) => component.chargingState === 'charging',
    )
  ) {
    return 'charging';
  }

  const percentages = freshPercentages(device);

  if (percentages.length === 0) {
    return freshComponents.some(
      (component) => component.chargingState === 'full',
    )
      ? 'healthy'
      : 'unknown';
  }

  const lowestPercentage = Math.min(...percentages);

  if (lowestPercentage > 50) return 'healthy';
  if (lowestPercentage > 20) return 'medium';
  return 'critical';
}
