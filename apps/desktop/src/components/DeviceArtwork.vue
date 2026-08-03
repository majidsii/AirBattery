<script setup lang="ts">
import { computed } from 'vue';

import {
  resolvePreRenderedArtworkAsset,
  type DeviceArtworkMode,
} from '../domain/artwork-catalog.ts';
import { resolveArtworkVariant } from '../domain/presentation.ts';
import type { ComponentType } from '../domain/types.ts';

const props = withDefaults(
  defineProps<{
    artworkKey: string;
    componentType?: ComponentType;
    paired?: boolean;
    label?: string;
  }>(),
  {
    componentType: 'unknown',
    paired: false,
    label: 'Bluetooth device',
  },
);

const variant = computed(() => resolveArtworkVariant(props.artworkKey));
const family = computed(() => {
  if (['airpodsPro', 'airpodsClassic', 'galaxyBuds', 'pixelBuds', 'stemEarbuds', 'genericEarbuds']
    .includes(variant.value)) {
    return 'earbuds';
  }
  return variant.value;
});

const artworkMode = computed<DeviceArtworkMode>(() => {
  if (props.componentType === 'case') return 'case';
  if (props.paired || ['aggregate', 'headset', 'unknown'].includes(props.componentType)) return 'pair';
  return props.componentType === 'right' ? 'right' : 'left';
});

const catalogArtwork = computed(() =>
  resolvePreRenderedArtworkAsset(props.artworkKey, artworkMode.value));
</script>

<template>
  <div
    class="device-artwork"
    :class="[`device-artwork--${family}`, { 'device-artwork--paired': paired }]"
    role="img"
    :aria-label="label"
  >
    <img
      v-if="catalogArtwork"
      class="device-artwork__catalog-asset device-artwork__pre-rendered"
      :src="catalogArtwork.src"
      alt=""
      aria-hidden="true"
    />

    <svg
      v-else-if="family === 'earbuds'"
      class="device-artwork__generic-fallback"
      viewBox="0 0 180 130"
      aria-hidden="true"
    >
      <g v-if="componentType === 'case'" class="art-case">
        <rect x="31" y="38" width="118" height="67" rx="27" />
        <path d="M31 64h118" />
        <circle cx="90" cy="75" r="3" />
      </g>
      <g v-else class="art-pair-root">
        <g class="art-bud art-bud--left art-bud--generic">
          <ellipse cx="61" cy="59" rx="27" ry="31" />
          <circle cx="61" cy="58" r="7" />
        </g>
        <g v-if="paired" class="art-bud art-bud--right art-bud--generic">
          <ellipse cx="119" cy="59" rx="27" ry="31" />
          <circle cx="119" cy="58" r="7" />
        </g>
      </g>
    </svg>

    <svg v-else-if="family === 'headset'" class="device-artwork__generic-fallback" viewBox="0 0 180 130" aria-hidden="true">
      <path class="art-outline" d="M36 72V63a54 54 0 0 1 108 0v9" />
      <rect class="art-fill" x="25" y="62" width="34" height="56" rx="17" />
      <rect class="art-fill" x="121" y="62" width="34" height="56" rx="17" />
    </svg>

    <svg v-else-if="family === 'speaker'" class="device-artwork__generic-fallback" viewBox="0 0 180 130" aria-hidden="true">
      <rect class="art-fill" x="21" y="34" width="138" height="71" rx="28" />
      <circle class="art-cut" cx="58" cy="70" r="18" />
      <circle class="art-cut" cx="122" cy="70" r="18" />
      <circle class="art-outline" cx="90" cy="70" r="5" />
    </svg>

    <svg v-else-if="family === 'mouse'" class="device-artwork__generic-fallback" viewBox="0 0 180 130" aria-hidden="true">
      <path class="art-fill" d="M90 17c-32 0-52 23-52 56 0 31 19 46 52 46s52-15 52-46c0-33-20-56-52-56Z" />
      <path class="art-cut" d="M90 18v45" />
      <rect class="art-cut" x="85" y="30" width="10" height="22" rx="5" />
    </svg>

    <svg v-else-if="family === 'keyboard'" class="device-artwork__generic-fallback" viewBox="0 0 180 130" aria-hidden="true">
      <rect class="art-fill" x="17" y="34" width="146" height="72" rx="12" />
      <g class="art-keys">
        <path d="M31 49h118M31 65h118M31 81h118M44 43v50M61 43v50M78 43v50M95 43v50M112 43v50M129 43v50" />
      </g>
    </svg>

    <svg v-else-if="family === 'controller'" class="device-artwork__generic-fallback" viewBox="0 0 180 130" aria-hidden="true">
      <path class="art-fill" d="M53 38h74c20 0 35 40 30 63-3 14-16 16-25 5l-17-20H65l-17 20c-9 11-22 9-25-5-5-23 10-63 30-63Z" />
      <path class="art-cut" d="M56 55v28M42 69h28" />
      <circle class="art-cut" cx="126" cy="61" r="6" />
      <circle class="art-cut" cx="141" cy="76" r="6" />
    </svg>

    <svg v-else-if="family === 'stylus'" class="device-artwork__generic-fallback" viewBox="0 0 180 130" aria-hidden="true">
      <g transform="rotate(-28 90 65)">
        <rect class="art-fill" x="24" y="54" width="128" height="22" rx="11" />
        <path class="art-fill" d="m152 54 23 11-23 11Z" />
      </g>
    </svg>

    <svg v-else class="device-artwork__generic-fallback" viewBox="0 0 180 130" aria-hidden="true">
      <circle class="art-fill" cx="90" cy="65" r="48" />
      <path class="bluetooth-mark" d="m85 31 27 23-22 18 22 20-27 22V31Zm8 18v17l10-9-10-8Zm0 32v17l10-8-10-9ZM67 48l45 42M67 87l45-39" />
    </svg>
  </div>
</template>
