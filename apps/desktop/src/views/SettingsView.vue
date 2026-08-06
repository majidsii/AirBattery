<script setup lang="ts">
import { computed } from 'vue';

import GlassCheckbox from '../components/GlassCheckbox.vue';
import GlassSelect from '../components/GlassSelect.vue';
import ToggleControl from '../components/ToggleControl.vue';
import type { GlassSelectOption } from '../domain/glass-controls.ts';
import { glassPresetOptions, type GlassSurface } from '../domain/glass.ts';
import { openWidget } from '../api/backend.ts';
import { useDeviceStore } from '../stores/devices.ts';
import { useSettingsStore } from '../stores/settings.ts';

const settings = useSettingsStore();
const devices = useDeviceStore();

const themeOptions: readonly GlassSelectOption[] = [
  {
    value: 'system',
    label: 'System',
    description: 'Follow the operating-system appearance.',
  },
  {
    value: 'light',
    label: 'Light',
    description: 'Use the pearl light interface.',
  },
  {
    value: 'dark',
    label: 'Dark',
    description: 'Use the neutral black interface.',
  },
];

const platform = computed(() => devices.backendStatus?.platform ?? 'unsupported');
const isLinux = computed(() => platform.value === 'linux');
const isWindows = computed(() => platform.value === 'windows');
const trayLabel = computed(() => isWindows.value ? 'Windows system tray' : 'Native status icon fallback');
const trayDescription = computed(() => isLinux.value
  ? 'Used outside GNOME Shell, or when GNOME integration is disabled.'
  : isWindows.value
    ? 'Show AirBattery in the Windows notification area.'
    : 'Show the native status surface supported by this operating system.');

const glassPresetTitle = computed(() => {
  const preset = settings.glass.preset;
  return preset === 'custom' ? 'Custom glass' : `${preset[0].toUpperCase()}${preset.slice(1)} glass`;
});

const hiddenDevices = computed(() =>
  devices.devices.filter((device) => settings.value.hiddenDeviceIds.includes(device.id)),
);


function updateGlassIntensity(event: Event): void {
  settings.setGlassIntensity(Number((event.target as HTMLInputElement).value));
}

function updateSurface(surface: GlassSurface, event: Event): void {
  settings.setGlassSurface(surface, Number((event.target as HTMLInputElement).value));
}

function updateTheme(value: string): void {
  if (
    value !== 'system'
    && value !== 'light'
    && value !== 'dark'
  ) {
    return;
  }

  settings.value.appearance.theme = value;
  settings.applyAppearance();
}

function updateAdaptive(value: boolean): void {
  settings.setGlassAdaptive(value);
}

function updateReduceTransparency(value: boolean): void {
  settings.setReduceTransparency(value);
}

async function restoreDevice(deviceId: string): Promise<void> {
  settings.showDevice(deviceId);
  await settings.persist();
}
</script>

<template>
  <section class="page settings-page" aria-labelledby="settings-title">
    <header class="page-header">
      <div>
        <p class="eyebrow">Personalize AirBattery</p>
        <h1 id="settings-title">Settings</h1>
      </div>
      <button class="primary-button" type="button" :disabled="settings.saving" @click="settings.persist">
        {{ settings.saving ? 'Saving…' : 'Save changes' }}
      </button>
    </header>

    <p v-if="settings.error" class="inline-error" role="alert">{{ settings.error }}</p>

    <div class="settings-stack">
      <section class="settings-group glass-card" aria-labelledby="general-heading">
        <div class="settings-group__heading">
          <span aria-hidden="true">⌘</span>
          <div><h2 id="general-heading">General</h2><p>Background behavior and system integration.</p></div>
        </div>
        <ToggleControl v-model="settings.value.startWithSystem" label="Start with system" description="Launch for the current user after sign-in." />
        <ToggleControl v-model="settings.value.runInBackground" label="Run in background" description="Keep battery updates active after closing the window." />
        <ToggleControl v-model="settings.value.showTrayIcon" :label="trayLabel" :description="trayDescription" />
        <ToggleControl v-if="isLinux" v-model="settings.value.enableGnomeIntegration" label="GNOME Shell extension" description="Use the native GNOME top-panel surface and suppress the duplicate AppIndicator icon." />
        <ToggleControl v-model="settings.value.enableDesktopWidget" label="Desktop widget" description="Show an optional floating, always-on-top battery surface." />
        <button class="setting-action" type="button" @click="openWidget">Open widget now <span aria-hidden="true">→</span></button>
      </section>

      <section class="settings-group glass-card" aria-labelledby="appearance-heading">
        <div class="settings-group__heading">
          <span aria-hidden="true">◐</span>
          <div><h2 id="appearance-heading">Appearance</h2><p>Readable glass surfaces with system-aware fallbacks.</p></div>
        </div>
        <div class="setting-row">
          <span class="setting-row__copy">
            <strong>Theme</strong>
            <small>Follow the system or choose a fixed appearance.</small>
          </span>
          <GlassSelect
            :model-value="settings.value.appearance.theme"
            :options="themeOptions"
            label="Theme"
            @update:model-value="updateTheme"
          />
        </div>
        <div class="glass-control" aria-labelledby="glass-material-heading">
          <div class="glass-control__heading">
            <span class="setting-row__copy">
              <strong id="glass-material-heading">Glass material</strong>
              <small>Choose a starting point, then tune every native surface independently.</small>
            </span>
            <output>{{ settings.glass.intensity }}%</output>
          </div>

          <div class="glass-presets" role="group" aria-label="Glass material preset">
            <button
              v-for="preset in glassPresetOptions"
              :key="preset.value"
              class="glass-preset"
              :class="{ active: settings.glass.preset === preset.value }"
              type="button"
              :title="preset.description"
              @click="settings.setGlassPreset(preset.value)"
            >
              {{ preset.label }}
            </button>
          </div>

          <label class="glass-range glass-range--master">
            <span><strong>Overall intensity</strong><small>Controls blur, saturation, tint, depth, and edge highlights.</small></span>
            <input :value="settings.glass.intensity" type="range" min="0" max="100" step="1" @input="updateGlassIntensity" />
          </label>

          <div class="glass-surface-grid" aria-label="Per-surface glass intensity">
            <label class="glass-range">
              <span><strong>Main window</strong><small>{{ settings.glass.surfaces.main }}%</small></span>
              <input :value="settings.glass.surfaces.main" type="range" min="0" max="100" step="1" @input="updateSurface('main', $event)" />
            </label>
            <label class="glass-range">
              <span><strong>Desktop widget</strong><small>{{ settings.glass.surfaces.widget }}%</small></span>
              <input :value="settings.glass.surfaces.widget" type="range" min="0" max="100" step="1" @input="updateSurface('widget', $event)" />
            </label>
            <label class="glass-range">
              <span><strong>Connection popup</strong><small>{{ settings.glass.surfaces.popup }}%</small></span>
              <input :value="settings.glass.surfaces.popup" type="range" min="0" max="100" step="1" @input="updateSurface('popup', $event)" />
            </label>
          </div>

          <div class="glass-accessibility">
            <GlassCheckbox
              :model-value="settings.glass.adaptive"
              label="Adaptive glass"
              description="Balance contrast against the wallpaper and active theme."
              @update:model-value="updateAdaptive"
            />
            <GlassCheckbox
              :model-value="settings.glass.reduceTransparency"
              label="Reduce transparency"
              description="Use solid, high-contrast surfaces while retaining spacing and hierarchy."
              @update:model-value="updateReduceTransparency"
            />
          </div>

          <div class="glass-preview" aria-label="Glass material preview">
            <span class="glass-preview__orb" aria-hidden="true" />
            <div>
              <strong>{{ glassPresetTitle }}</strong>
              <small>Main {{ settings.glass.surfaces.main }} · Widget {{ settings.glass.surfaces.widget }} · Popup {{ settings.glass.surfaces.popup }}</small>
            </div>
          </div>
        </div>
        <ToggleControl v-model="settings.value.appearance.compactLayout" label="Compact layout" description="Use tighter spacing in the popup and widget." />
        <ToggleControl v-model="settings.value.appearance.animations" label="Interface animations" description="Animate live updates and navigation transitions." />
        <ToggleControl v-model="settings.value.appearance.reducedMotion" label="Reduce motion" description="Disable spring and charging motion for accessibility." />
      </section>

      <section class="settings-group glass-card" aria-labelledby="notifications-heading">
        <div class="settings-group__heading">
          <span aria-hidden="true">◔</span>
          <div><h2 id="notifications-heading">Battery notifications</h2><p>Local alerts with a cooldown to prevent noise.</p></div>
        </div>
        <ToggleControl v-model="settings.value.notifications.enabled" label="Battery notifications" />
        <label class="setting-row">
          <span class="setting-row__copy"><strong>Low battery</strong><small>Warn when a known component reaches this level.</small></span>
          <input v-model.number="settings.value.notifications.lowThreshold" class="number-input" type="number" min="1" max="99" />
        </label>
        <label class="setting-row">
          <span class="setting-row__copy"><strong>Critical battery</strong><small>Must remain below the low threshold.</small></span>
          <input v-model.number="settings.value.notifications.criticalThreshold" class="number-input" type="number" min="1" max="99" />
        </label>
        <ToggleControl v-model="settings.value.notifications.componentSpecific" label="Component-specific alerts" description="Name the left, right, case, or headset component." />
        <ToggleControl v-model="settings.value.notifications.connectionEvents" label="Connection alerts" description="Notify on meaningful connect and disconnect transitions." />
      </section>

      <section v-if="hiddenDevices.length" class="settings-group glass-card" aria-labelledby="hidden-heading">
        <div class="settings-group__heading"><span aria-hidden="true">◌</span><div><h2 id="hidden-heading">Hidden devices</h2><p>Restore devices to the main list.</p></div></div>
        <div v-for="device in hiddenDevices" :key="device.id" class="setting-row">
          <span class="setting-row__copy"><strong>{{ device.displayName }}</strong><small>{{ device.connectionState }}</small></span>
          <button class="text-button" type="button" @click="restoreDevice(device.id)">Restore</button>
        </div>
      </section>
    </div>
  </section>
</template>
