<script setup lang="ts">
import { onBeforeUnmount, onMounted, ref, type Component } from 'vue';

import { getBackendClient } from './api/backend.ts';
import AppNav, { type AppRoute } from './components/AppNav.vue';
import { resolveWindowMode } from './domain/window-mode.ts';
import AboutView from './views/AboutView.vue';
import BatteryView from './views/BatteryView.vue';
import DevicesView from './views/DevicesView.vue';
import DiagnosticsView from './views/DiagnosticsView.vue';
import SettingsView from './views/SettingsView.vue';
import { useDeviceStore } from './stores/devices.ts';
import { useSettingsStore } from './stores/settings.ts';

const route = ref<AppRoute>('battery');
const devices = useDeviceStore();
const settings = useSettingsStore();
const windowMode = resolveWindowMode(
  typeof window === 'undefined' ? '' : window.location.search,
);
let stopNavigation: (() => void) | null = null;

const views: Record<AppRoute, Component> = {
  battery: BatteryView,
  devices: DevicesView,
  settings: SettingsView,
  diagnostics: DiagnosticsView,
  about: AboutView,
};

function isAppRoute(value: string): value is AppRoute {
  return value in views;
}

onMounted(async () => {
  await settings.load();
  await devices.initialize();
  if (windowMode === 'main') {
    const client = await getBackendClient();
    stopNavigation = await client.listenForNavigation((nextRoute) => {
      if (isAppRoute(nextRoute)) route.value = nextRoute;
    });
  }
});

onBeforeUnmount(() => {
  devices.dispose();
  stopNavigation?.();
});
</script>

<template>
  <a v-if="windowMode === 'main'" class="skip-link" href="#main-content">Skip to content</a>
  <div class="ambient-background" aria-hidden="true"><span /><span /><span /></div>
  <div
    class="app-shell"
    :class="{
      compact: settings.value.appearance.compactLayout,
      'widget-shell': windowMode === 'widget',
    }"
  >
    <AppNav v-if="windowMode === 'main'" :active="route" @navigate="route = $event" />
    <main id="main-content" class="app-content" tabindex="-1">
      <BatteryView v-if="windowMode === 'widget'" compact />
      <Transition v-else name="page" mode="out-in">
        <component :is="views[route]" :key="route" />
      </Transition>
    </main>
  </div>
</template>
