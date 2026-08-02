import { effectiveConnectionState } from './presentation.ts';
import type { BluetoothAudioDevice } from './types.ts';

export function filterVisibleDevices(
  devices: readonly BluetoothAudioDevice[],
  hiddenDeviceIds: readonly string[],
): BluetoothAudioDevice[] {
  const hidden = new Set(hiddenDeviceIds);
  return devices.filter((device) => {
    const state = effectiveConnectionState(device);
    const active = state === 'connected' || state === 'connecting';
    return active || !hidden.has(device.id);
  });
}

export function toggleHiddenDevice(
  hiddenDeviceIds: readonly string[],
  deviceId: string,
  hidden: boolean,
): string[] {
  const next = new Set(hiddenDeviceIds);
  if (hidden) next.add(deviceId);
  else next.delete(deviceId);
  return [...next];
}
