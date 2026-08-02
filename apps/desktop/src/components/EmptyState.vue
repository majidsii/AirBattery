<script setup lang="ts">
withDefaults(
  defineProps<{
    title: string;
    detail: string;
    actionLabel?: string;
    busy?: boolean;
  }>(),
  { actionLabel: '', busy: false },
);
const emit = defineEmits<{ action: [] }>();
</script>

<template>
  <section class="empty-state glass-card">
    <div class="empty-state__illustration" aria-hidden="true">
      <span class="signal-ring signal-ring--one" />
      <span class="signal-ring signal-ring--two" />
      <span class="empty-state__dot" />
    </div>
    <h2>{{ title }}</h2>
    <p>{{ detail }}</p>
    <button
      v-if="actionLabel"
      class="primary-button"
      type="button"
      :disabled="busy"
      @click="emit('action')"
    >
      {{ busy ? 'Refreshing…' : actionLabel }}
    </button>
  </section>
</template>
