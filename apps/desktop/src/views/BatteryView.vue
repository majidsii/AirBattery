<script setup lang="ts">
import { computed } from 'vue';

import DeviceBatteryPanel from '../components/DeviceBatteryPanel.vue';
import EmptyState from '../components/EmptyState.vue';
import StatusPill from '../components/StatusPill.vue';
import { effectiveConnectionState } from '../domain/presentation.ts';
import { useDeviceStore } from '../stores/devices.ts';

const props = withDefaults(defineProps<{ compact?: boolean }>(), { compact: false });
const devices = useDeviceStore();

const compactDevice = computed(() => devices.selectedDevice);
const displayedDevices = computed(() => {
  if (!props.compact) return devices.dashboardDevices;
  return compactDevice.value ? [compactDevice.value] : [];
});

const emptyTitle = computed(() => {
  if (devices.loading) return 'Looking for battery data';
  if (!devices.backendStatus?.available) return 'Bluetooth backend unavailable';
  if (devices.backendStatus.powered === false) return 'Bluetooth is turned off';
  return 'No compatible device data yet';
});

const emptyDetail = computed(() =>
  devices.error ??
  devices.backendStatus?.detail ??
  'Connect a Bluetooth device. AirBattery automatically shows every connected device.',
);
</script>

<template>
  <section
    class="page battery-page"
    :class="{ 'battery-page--compact': props.compact }"
    aria-labelledby="battery-title"
  >
    <header v-if="!props.compact" class="overview-header">
      <div>
        <p class="eyebrow">Active devices</p>
        <h1 id="battery-title">Overview</h1>
        <p class="overview-header__summary">
          Truthful battery readings for connected Bluetooth devices.
        </p>
      </div>
      <div class="overview-header__status glass-chip" aria-live="polite">
        <span aria-hidden="true" />
        <strong>{{ displayedDevices.length }}</strong>
        <small>{{ displayedDevices.length === 1 ? 'active device' : 'active devices' }}</small>
      </div>
    </header>

    <header v-else class="overview-compact-header glass-titlebar">
      <h1 id="battery-title">{{ compactDevice?.displayName ?? 'AirBattery' }}</h1>
      <StatusPill v-if="compactDevice" :state="effectiveConnectionState(compactDevice)" />
    </header>

    <div v-if="displayedDevices.length > 0" class="connected-device-list">
      <template v-if="!props.compact">
        <article
          v-for="device in displayedDevices"
          :key="device.id"
          class="connected-device-card glass-card liquid-surface"
        >
          <header class="connected-device-card__header">
            <div>
              <p class="connected-device-card__name">{{ device.displayName }}</p>
              <p v-if="device.model" class="connected-device-card__model">{{ device.model }}</p>
            </div>
            <div class="connected-device-card__meta">
              <StatusPill :state="effectiveConnectionState(device)" />
              <time v-if="device.lastUpdatedAt" :datetime="device.lastUpdatedAt">
                {{ new Date(device.lastUpdatedAt).toLocaleTimeString([], { hour: '2-digit', minute: '2-digit' }) }}
              </time>
            </div>
          </header>
          <DeviceBatteryPanel :device="device" />
        </article>
      </template>

      <template v-else-if="compactDevice">
        <div class="device-meta">
          <StatusPill :state="effectiveConnectionState(compactDevice)" />
        </div>
        <DeviceBatteryPanel :device="compactDevice" compact />
      </template>

      <aside v-if="!props.compact" class="freshness-note glass-subtle">
        <span aria-hidden="true">◎</span>
        <p>
          Case values remain visible as the last valid reading when the case stops reporting.
          Approximate Bluetooth advertisements are marked with “≈”; exact accessory readings are not.
        </p>
      </aside>
    </div>

    <EmptyState v-else :title="emptyTitle" :detail="emptyDetail" />
  </section>
</template>
