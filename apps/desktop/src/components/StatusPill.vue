<script setup lang="ts">
import { computed } from 'vue';
import type { ConnectionState } from '../domain/types.ts';

const props = defineProps<{
  state: ConnectionState;
}>();

const label = computed(() => {
  const labels: Record<ConnectionState, string> = {
    connected: 'Connected',
    disconnected: 'Disconnected',
    connecting: 'Connecting',
    unknown: 'Status unavailable',
  };
  return labels[props.state];
});
</script>

<template>
  <span class="status-pill" :class="`status-${state}`">
    <span class="status-pill__dot" aria-hidden="true" />
    {{ label }}
  </span>
</template>
