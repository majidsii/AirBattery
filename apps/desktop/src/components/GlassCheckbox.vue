<script setup lang="ts">
const props = withDefaults(
  defineProps<{
    modelValue: boolean;
    label: string;
    description?: string;
    disabled?: boolean;
  }>(),
  {
    description: '',
    disabled: false,
  },
);

const emit = defineEmits<{
  'update:modelValue': [value: boolean];
  change: [value: boolean];
}>();

function update(event: Event): void {
  const value = (event.target as HTMLInputElement).checked;

  emit('update:modelValue', value);
  emit('change', value);
}
</script>

<template>
  <label
    class="glass-checkbox"
    :class="{
      checked: props.modelValue,
      disabled: props.disabled,
    }"
  >
    <input
      class="sr-only"
      type="checkbox"
      :checked="props.modelValue"
      :disabled="props.disabled"
      @change="update"
    />

    <span
      class="glass-checkbox__box"
      aria-hidden="true"
    >
      <svg
        class="glass-checkbox__check-icon"
        viewBox="0 0 16 16"
        fill="none"
        focusable="false"
      >
        <path
          d="M3.2 8.15 6.35 11.05 12.8 4.75"
          stroke="currentColor"
          stroke-width="1.9"
          stroke-linecap="round"
          stroke-linejoin="round"
        />
      </svg>
    </span>

    <span class="glass-checkbox__copy">
      <strong>{{ props.label }}</strong>

      <small v-if="props.description">
        {{ props.description }}
      </small>
    </span>
  </label>
</template>
