<script setup lang="ts">
export type AppRoute = 'battery' | 'devices' | 'settings' | 'diagnostics' | 'about';

const props = defineProps<{ active: AppRoute }>();
const emit = defineEmits<{ navigate: [route: AppRoute] }>();

const primaryItems: Array<{ route: AppRoute; label: string; glyph: string }> = [
  { route: 'battery', label: 'Battery', glyph: '◉' },
  { route: 'devices', label: 'Devices', glyph: '⌁' },
  { route: 'settings', label: 'Settings', glyph: '⚙' },
  { route: 'diagnostics', label: 'Diagnostics', glyph: '⌘' },
];

const footerItems: Array<{ route: AppRoute; label: string; glyph: string }> = [
  { route: 'about', label: 'About', glyph: 'A' },
];
</script>

<template>
  <nav class="app-nav" aria-label="Primary navigation">
    <div class="brand-mark" aria-label="AirBattery"><span aria-hidden="true">A</span></div>
    <div class="app-nav__primary">
      <button
        v-for="item in primaryItems"
        :key="item.route"
        class="nav-button"
        :class="{ active: props.active === item.route }"
        :aria-current="props.active === item.route ? 'page' : undefined"
        :aria-label="item.label"
        type="button"
        @click="emit('navigate', item.route)"
      >
        <span aria-hidden="true">{{ item.glyph }}</span>
        <span class="nav-button__label">{{ item.label }}</span>
      </button>
    </div>
    <div class="app-nav__footer">
      <button
        v-for="item in footerItems"
        :key="item.route"
        class="nav-button"
        :class="{ active: props.active === item.route }"
        :aria-current="props.active === item.route ? 'page' : undefined"
        :aria-label="item.label"
        type="button"
        @click="emit('navigate', item.route)"
      >
        <span aria-hidden="true">{{ item.glyph }}</span>
        <span class="nav-button__label">{{ item.label }}</span>
      </button>
    </div>
  </nav>
</template>
