<script setup lang="ts">
withDefaults(defineProps<{ decorative?: boolean }>(), {
  decorative: false,
});
</script>

<template>
  <svg
    class="brand-mark__svg"
    viewBox="0 0 64 64"
    fill="none"
    xmlns="http://www.w3.org/2000/svg"
    :aria-hidden="decorative ? 'true' : undefined"
    :aria-label="decorative ? undefined : 'AirBattery logo'"
    role="img"
  >
    <rect class="brand-mark__terminal" x="26" y="7" width="12" height="8" rx="2.5" fill="currentColor" />
    <rect class="brand-mark__battery" x="17" y="14" width="30" height="43" rx="9" stroke="currentColor" stroke-width="4.5" />
    <path
      class="brand-mark__bluetooth"
      d="M32 24v20m0-20 7 6-7 5 7 5-7 4m0-9-6-5m6 5-6 5"
      stroke="currentColor"
      stroke-width="3.4"
      stroke-linecap="round"
      stroke-linejoin="round"
    />
  </svg>
</template>
