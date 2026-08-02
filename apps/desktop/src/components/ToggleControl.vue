<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    modelValue: boolean;
    label: string;
    description?: string;
    disabled?: boolean;
  }>(),
  { description: '', disabled: false },
);

const emit = defineEmits<{ 'update:modelValue': [value: boolean] }>();
</script>

<template>
  <label class="setting-row" :class="{ disabled: props.disabled }">
    <span class="setting-row__copy">
      <strong>{{ props.label }}</strong>
      <small v-if="props.description">{{ props.description }}</small>
    </span>
    <input
      class="sr-only"
      type="checkbox"
      role="switch"
      :checked="props.modelValue"
      :disabled="props.disabled"
      @change="emit('update:modelValue', ($event.target as HTMLInputElement).checked)"
    />
    <span class="toggle" aria-hidden="true"><span /></span>
  </label>
</template>
