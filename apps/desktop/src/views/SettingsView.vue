<script setup lang="ts">
import { computed } from 'vue';

import ToggleControl from '../components/ToggleControl.vue';
import { openWidget } from '../api/backend.ts';
import { useDeviceStore } from '../stores/devices.ts';
import { useSettingsStore } from '../stores/settings.ts';

const settings = useSettingsStore();
const devices = useDeviceStore();

const platform = computed(() => devices.backendStatus?.platform ?? 'unsupported');
const isLinux = computed(() => platform.value === 'linux');
const isWindows = computed(() => platform.value === 'windows');
const trayLabel = computed(() => isWindows.value ? 'Windows system tray' : 'Native status icon fallback');
const trayDescription = computed(() => isLinux.value
  ? 'Used outside GNOME Shell, or when GNOME integration is disabled.'
  : isWindows.value
    ? 'Show AirBattery in the Windows notification area.'
    : 'Show the native status surface supported by this operating system.');

const hiddenDevices = computed(() =>
  devices.devices.filter((device) => settings.value.hiddenDeviceIds.includes(device.id)),
);

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
        <label class="setting-row">
          <span class="setting-row__copy"><strong>Theme</strong><small>Follow the system or choose a fixed appearance.</small></span>
          <select v-model="settings.value.appearance.theme" @change="settings.applyAppearance">
            <option value="system">System</option><option value="light">Light</option><option value="dark">Dark</option>
          </select>
        </label>
        <label class="setting-row setting-row--range">
          <span class="setting-row__copy"><strong>Transparency</strong><small>{{ settings.value.appearance.transparency }}%</small></span>
          <input v-model.number="settings.value.appearance.transparency" type="range" min="35" max="95" step="1" @input="settings.applyAppearance" />
        </label>
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
