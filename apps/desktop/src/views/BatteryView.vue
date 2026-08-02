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
    <header class="page-header battery-header">
      <div>
        <p v-if="!props.compact" class="eyebrow">Live batteries</p>
        <h1 id="battery-title">
          {{ props.compact ? (compactDevice?.displayName ?? 'AirBattery') : 'Connected batteries' }}
        </h1>
        <p v-if="!props.compact && displayedDevices.length > 0" class="battery-header__summary">
          {{ displayedDevices.length }} active {{ displayedDevices.length === 1 ? 'device' : 'devices' }}
        </p>
      </div>
    </header>

    <div v-if="displayedDevices.length > 0" class="connected-device-list">
      <template v-if="!props.compact">
        <article
          v-for="device in displayedDevices"
          :key="device.id"
          class="connected-device-card glass-card"
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
