<script setup lang="ts">
import { computed } from 'vue';

import {
  buildBatteryLayout,
  effectiveConnectionState,
  presentBatteryComponent,
  selectDeviceArtwork,
} from '../domain/presentation.ts';
import type { BluetoothAudioDevice, ComponentType } from '../domain/types.ts';
import DeviceArtwork from './DeviceArtwork.vue';

const props = withDefaults(
  defineProps<{
    device: BluetoothAudioDevice;
    compact?: boolean;
  }>(),
  { compact: false },
);

const layout = computed(() => buildBatteryLayout(props.device));
const artwork = computed(() => selectDeviceArtwork(props.device));
const connectionState = computed(() => effectiveConnectionState(props.device));

const labels: Record<ComponentType, string> = {
  left: 'Left',
  right: 'Right',
  case: 'Case',
  headset: 'Battery',
  aggregate: 'Battery',
  unknown: 'Battery',
};
</script>

<template>
  <div
    class="device-battery-panel"
    :class="[`layout-${layout.mode}`, { 'is-compact': compact }]"
    aria-live="polite"
  >
    <article
      v-for="slot in layout.slots"
      :key="slot.componentType"
      class="device-battery-slot glass-card"
      :class="`tone-${presentBatteryComponent(slot.component, connectionState).tone}`"
      :aria-label="presentBatteryComponent(slot.component, connectionState).ariaLabel"
    >
      <p v-if="layout.mode !== 'single'" class="device-battery-slot__label">
        {{ labels[slot.componentType] }}
      </p>

      <DeviceArtwork
        :artwork-key="artwork"
        :component-type="slot.componentType"
        :paired="layout.artworkMode === 'pairedEarbuds'"
        :label="`${device.displayName} ${labels[slot.componentType]}`"
      />

      <div class="device-battery-slot__value-row">
        <strong>{{ presentBatteryComponent(slot.component, connectionState).valueText }}</strong>
        <span
          v-if="slot.component.chargingState === 'charging'"
          class="device-battery-slot__charging"
          aria-label="Charging"
        >↯</span>
      </div>
      <p class="device-battery-slot__status">
        {{ presentBatteryComponent(slot.component, connectionState).statusText }}
      </p>
    </article>
  </div>
</template>
