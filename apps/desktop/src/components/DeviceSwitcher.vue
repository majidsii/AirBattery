<script setup lang="ts">
import { computed } from 'vue';

import GlassSelect from './GlassSelect.vue';
import type { GlassSelectOption } from '../domain/glass-controls.ts';
import type { BluetoothAudioDevice } from '../domain/types.ts';

const props = defineProps<{
  devices: BluetoothAudioDevice[];
  selectedId: string | null;
}>();

const emit = defineEmits<{
  select: [deviceId: string];
}>();

const options = computed<readonly GlassSelectOption[]>(() =>
  props.devices.map((device) => ({
    value: device.id,
    label: device.displayName,
    description: device.connectionState,
  })),
);

function selectDevice(deviceId: string): void {
  if (!deviceId || deviceId === props.selectedId) return;

  emit('select', deviceId);
}
</script>

<template>
  <div class="device-switcher">
    <GlassSelect
      :model-value="props.selectedId ?? ''"
      :options="options"
      label="Selected Bluetooth device"
      :disabled="props.devices.length < 2"
      empty-label="No Bluetooth device"
      @update:model-value="selectDevice"
    />
  </div>
</template>
