import { computed, ref } from 'vue';
import { defineStore } from 'pinia';

import { getBackendClient, type BackendClient } from '../api/backend.ts';
import { filterVisibleDevices } from '../domain/device-list.ts';
import { selectDashboardDevices, selectPreferredDevice } from '../domain/presentation.ts';
import type { BackendStatus, BluetoothAudioDevice } from '../domain/types.ts';
import { useSettingsStore } from './settings.ts';

export const useDeviceStore = defineStore('devices', () => {
  const devices = ref<BluetoothAudioDevice[]>([]);
  const backendStatus = ref<BackendStatus | null>(null);
  const selectedDeviceId = ref<string | null>(null);
  const loading = ref(false);
  const initialized = ref(false);
  const error = ref<string | null>(null);
  const settings = useSettingsStore();
  let client: BackendClient | null = null;
  let stopListening: (() => void) | null = null;
  let stopStatusListening: (() => void) | null = null;

  const visibleDevices = computed(() =>
    filterVisibleDevices(devices.value, settings.value.hiddenDeviceIds),
  );

  const selectedDevice = computed(() => {
    const requested = selectedDeviceId.value ?? settings.value.preferredDeviceId;
    return selectPreferredDevice(visibleDevices.value, requested);
  });

  const dashboardDevices = computed(() =>
    selectDashboardDevices(
      visibleDevices.value,
      selectedDeviceId.value ?? settings.value.preferredDeviceId,
    ),
  );

  function acceptDevices(nextDevices: BluetoothAudioDevice[]): void {
    devices.value = nextDevices;
    if (
      selectedDeviceId.value !== null &&
      !nextDevices.some((device) => device.id === selectedDeviceId.value)
    ) {
      selectedDeviceId.value = null;
    }
  }

  async function initialize(): Promise<void> {
    if (initialized.value) return;
    initialized.value = true;
    loading.value = true;
    error.value = null;
    try {
      client = await getBackendClient();
      backendStatus.value = await client.getBackendStatus();
      acceptDevices(await client.getDevices());
      stopListening = await client.listenForDeviceChanges(acceptDevices);
      stopStatusListening = await client.listenForBackendStatus((status) => {
        backendStatus.value = status;
      });
    } catch (reason) {
      error.value = reason instanceof Error ? reason.message : String(reason);
    } finally {
      loading.value = false;
    }
  }

  async function refresh(): Promise<void> {
    loading.value = true;
    error.value = null;
    try {
      client ??= await getBackendClient();
      acceptDevices(await client.refreshDevices());
      backendStatus.value = await client.getBackendStatus();
    } catch (reason) {
      error.value = reason instanceof Error ? reason.message : String(reason);
    } finally {
      loading.value = false;
    }
  }

  async function runDiagnosticScan(): Promise<void> {
    loading.value = true;
    error.value = null;
    try {
      client ??= await getBackendClient();
      acceptDevices(await client.runDiagnosticScan());
      backendStatus.value = await client.getBackendStatus();
    } catch (reason) {
      error.value = reason instanceof Error ? reason.message : String(reason);
    } finally {
      loading.value = false;
    }
  }

  function select(deviceId: string): void {
    if (visibleDevices.value.some((device) => device.id === deviceId)) {
      selectedDeviceId.value = deviceId;
    }
  }

  function dispose(): void {
    stopListening?.();
    stopListening = null;
    stopStatusListening?.();
    stopStatusListening = null;
  }

  return {
    devices,
    backendStatus,
    selectedDeviceId,
    loading,
    initialized,
    error,
    visibleDevices,
    selectedDevice,
    dashboardDevices,
    initialize,
    refresh,
    runDiagnosticScan,
    select,
    dispose,
  };
});
