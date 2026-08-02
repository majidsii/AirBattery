<script setup lang="ts">
import { computed } from 'vue';

import { presentBatteryComponent } from '../domain/presentation.ts';
import type { BatteryComponent, ConnectionState } from '../domain/types.ts';

const props = defineProps<{
  component: BatteryComponent;
  connectionState: ConnectionState;
}>();

const presented = computed(() => presentBatteryComponent(props.component, props.connectionState));
const circumference = 2 * Math.PI * 42;
const dashOffset = computed(() => {
  const value = props.component.percentage ?? 0;
  return circumference - (value / 100) * circumference;
});

const title = computed(() => {
  const labels: Record<BatteryComponent['componentType'], string> = {
    left: 'Left',
    right: 'Right',
    case: 'Case',
    headset: 'Headset',
    aggregate: 'Combined',
    unknown: 'Battery',
  };
  return labels[props.component.componentType];
});
</script>

<template>
  <article
    class="battery-card glass-card"
    :class="`tone-${presented.tone}`"
    :aria-label="presented.ariaLabel"
  >
    <div class="battery-ring" aria-hidden="true">
      <svg viewBox="0 0 100 100">
        <circle class="battery-ring__track" cx="50" cy="50" r="42" />
        <circle
          class="battery-ring__value"
          cx="50"
          cy="50"
          r="42"
          :style="{
            strokeDasharray: circumference,
            strokeDashoffset: dashOffset,
          }"
        />
      </svg>
      <div class="battery-ring__content">
        <span class="battery-ring__value-text">{{ presented.valueText }}</span>
        <span v-if="component.chargingState === 'charging'" class="charging-glyph">↯</span>
      </div>
    </div>
    <div class="battery-card__copy">
      <h3>{{ title }}</h3>
      <p>{{ presented.statusText }}</p>
    </div>
  </article>
</template>
