import { ref } from 'vue';
import { defineStore } from 'pinia';

import { getBackendClient } from '../api/backend.ts';
import { toggleHiddenDevice } from '../domain/device-list.ts';
import {
  applyGlassPreset,
  defaultGlassPreferences,
  glassCssVariables,
  loadGlassPreferences,
  saveGlassPreferences,
  setGlassIntensity as updateGlassIntensity,
  setGlassSurface as updateGlassSurface,
  type GlassSurface,
  type NamedGlassPreset,
} from '../domain/glass.ts';
import { defaultSettings, normalizeSettings } from '../domain/settings.ts';
import type { AppSettings } from '../domain/types.ts';

export const useSettingsStore = defineStore('settings', () => {
  const value = ref<AppSettings>(structuredClone(defaultSettings));
  const glass = ref(structuredClone(defaultGlassPreferences));
  const loading = ref(false);
  const saving = ref(false);
  const error = ref<string | null>(null);

  function applyAppearance(): void {
    if (typeof document === 'undefined') return;
    const root = document.documentElement;
    root.dataset.theme = value.value.appearance.theme;
    root.dataset.motion = value.value.appearance.reducedMotion ? 'reduced' : 'full';
    root.dataset.glassPreset = glass.value.preset;
    root.dataset.glassAdaptive = glass.value.adaptive ? 'true' : 'false';
    root.dataset.glassReduced = glass.value.reduceTransparency ? 'true' : 'false';
    root.style.setProperty('--glass-opacity', String(value.value.appearance.transparency / 100));
    for (const [name, variable] of Object.entries(glassCssVariables(glass.value))) {
      root.style.setProperty(name, variable);
    }
  }

  async function load(): Promise<void> {
    loading.value = true;
    error.value = null;
    glass.value = loadGlassPreferences(
      typeof window === 'undefined' ? null : window.localStorage,
    );
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
    saveGlassPreferences(
      typeof window === 'undefined' ? null : window.localStorage,
      glass.value,
    );
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

  function setGlassPreset(preset: NamedGlassPreset): void {
    glass.value = applyGlassPreset(glass.value, preset);
    applyAppearance();
  }

  function setGlassIntensity(intensity: number): void {
    glass.value = updateGlassIntensity(glass.value, intensity);
    applyAppearance();
  }

  function setGlassSurface(surface: GlassSurface, strength: number): void {
    glass.value = updateGlassSurface(glass.value, surface, strength);
    applyAppearance();
  }

  function setGlassAdaptive(enabled: boolean): void {
    glass.value.adaptive = enabled;
    glass.value.preset = 'custom';
    applyAppearance();
  }

  function setReduceTransparency(enabled: boolean): void {
    glass.value.reduceTransparency = enabled;
    glass.value.preset = 'custom';
    applyAppearance();
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
    glass,
    loading,
    saving,
    error,
    load,
    persist,
    applyAppearance,
    setGlassPreset,
    setGlassIntensity,
    setGlassSurface,
    setGlassAdaptive,
    setReduceTransparency,
    hideDevice,
    showDevice,
  };
});
