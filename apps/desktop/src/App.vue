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
  <div class="ambient-background" aria-hidden="true">
    <svg
      class="ambient-background__art ambient-background__art--top"
      viewBox="0 0 960 520"
      preserveAspectRatio="none"
      focusable="false"
    >
      <path
        class="ambient-background__shape"
        d="M164 -10C352 48 443 150 532 263C641 402 766 474 970 512V-10Z"
      />
      <path class="ambient-background__line ambient-background__line--primary" d="M113 -8C323 55 419 172 512 286C616 413 752 482 966 517" />
      <path class="ambient-background__line" d="M190 -6C362 69 454 176 545 286C645 408 771 463 968 492" />
      <path class="ambient-background__line" d="M267 -4C410 82 491 177 579 279C674 390 792 441 970 466" />
      <path class="ambient-background__line ambient-background__line--soft" d="M344 -2C459 91 530 174 611 266C701 368 811 418 970 440" />
      <path class="ambient-background__line ambient-background__line--soft" d="M421 0C510 96 571 167 646 251C727 344 831 391 970 414" />
    </svg>

    <svg
      class="ambient-background__art ambient-background__art--bottom"
      viewBox="0 0 1120 620"
      preserveAspectRatio="none"
      focusable="false"
    >
      <path
        class="ambient-background__shape"
        d="M-20 214C168 392 368 489 590 521C807 553 975 525 1141 444V641H-20Z"
      />
      <path class="ambient-background__line ambient-background__line--primary" d="M-18 179C177 369 386 472 611 504C826 535 990 504 1140 421" />
      <path class="ambient-background__line" d="M-18 226C169 397 374 489 598 520C811 549 978 522 1140 450" />
      <path class="ambient-background__line" d="M-18 273C165 424 365 505 587 535C799 563 968 541 1140 480" />
      <path class="ambient-background__line ambient-background__line--soft" d="M-18 321C161 453 358 521 579 550C789 577 958 559 1140 510" />
      <path class="ambient-background__line ambient-background__line--soft" d="M-18 369C158 482 351 538 572 566C779 592 949 578 1140 540" />
    </svg>
  </div>
  <div
    class="app-shell"
    :data-window-mode="windowMode"
    :class="{
      'glass-shell': true,
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
