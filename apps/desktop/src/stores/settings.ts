import { ref } from 'vue';
import { defineStore } from 'pinia';

import { getBackendClient } from '../api/backend.ts';
import { toggleHiddenDevice } from '../domain/device-list.ts';
import { defaultSettings, normalizeSettings } from '../domain/settings.ts';
import type { AppSettings } from '../domain/types.ts';

export const useSettingsStore = defineStore('settings', () => {
  const value = ref<AppSettings>(structuredClone(defaultSettings));
  const loading = ref(false);
  const saving = ref(false);
  const error = ref<string | null>(null);

  function applyAppearance(): void {
    if (typeof document === 'undefined') return;
    const root = document.documentElement;
    root.dataset.theme = value.value.appearance.theme;
    root.dataset.motion = value.value.appearance.reducedMotion ? 'reduced' : 'full';
    root.style.setProperty('--glass-opacity', String(value.value.appearance.transparency / 100));
  }

  async function load(): Promise<void> {
    loading.value = true;
    error.value = null;
    try {
      const client = await getBackendClient();
      value.value = normalizeSettings(await client.getSettings());
    } catch (reason) {
      error.value = reason instanceof Error ? reason.message : String(reason);
      value.value = structuredClone(defaultSettings);
    } finally {
      applyAppearance();
      loading.value = false;
    }
  }

  async function persist(): Promise<void> {
    saving.value = true;
    error.value = null;
    value.value = normalizeSettings(value.value);
    applyAppearance();
    try {
      const client = await getBackendClient();
      value.value = normalizeSettings(await client.saveSettings(value.value));
    } catch (reason) {
      error.value = reason instanceof Error ? reason.message : String(reason);
    } finally {
      saving.value = false;
    }
  }

  function hideDevice(deviceId: string): void {
    value.value.hiddenDeviceIds = toggleHiddenDevice(
      value.value.hiddenDeviceIds,
      deviceId,
      true,
    );
  }

  function showDevice(deviceId: string): void {
    value.value.hiddenDeviceIds = toggleHiddenDevice(
      value.value.hiddenDeviceIds,
      deviceId,
      false,
    );
  }

  return {
    value,
    loading,
    saving,
    error,
    load,
    persist,
    applyAppearance,
    hideDevice,
    showDevice,
  };
});
