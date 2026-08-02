<script setup lang="ts">
import type { BluetoothAudioDevice } from '../domain/types.ts';

const props = defineProps<{
  devices: BluetoothAudioDevice[];
  selectedId: string | null;
}>();

const emit = defineEmits<{
  select: [deviceId: string];
}>();
</script>

<template>
  <label class="device-switcher">
    <span class="sr-only">Selected Bluetooth device</span>
    <select
      :value="props.selectedId ?? ''"
      :disabled="devices.length < 2"
      @change="emit('select', ($event.target as HTMLSelectElement).value)"
    >
      <option v-for="device in devices" :key="device.id" :value="device.id">
        {{ device.displayName }} · {{ device.connectionState }}
      </option>
    </select>
    <span aria-hidden="true" class="device-switcher__chevron">⌄</span>
  </label>
</template>
