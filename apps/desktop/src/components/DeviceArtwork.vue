<script setup lang="ts">
import { computed } from 'vue';


import { resolveArtworkAssetSet } from '../domain/artwork-catalog.ts';
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
  if (['airpodsPro', 'airpodsClassic'].includes(variant.value)) return 'airpods';
  if (['galaxyBuds', 'pixelBuds', 'stemEarbuds', 'genericEarbuds'].includes(variant.value)) {
    return 'earbuds';
  }
  return variant.value;
});


const catalogArtwork = computed(() => {
  const assets = resolveArtworkAssetSet(props.artworkKey);
  if (!assets) return null;
  if (props.componentType === 'case') return assets.case;
  if (props.paired || ['aggregate', 'headset', 'unknown'].includes(props.componentType)) {
    return assets.pair;
  }
  return assets.single;
});

const artworkSourceKind = computed(() => catalogArtwork.value?.sourceKind ?? null);
const mirrorArtwork = computed(() =>
  props.componentType === 'right' &&
  (!catalogArtwork.value ||
    catalogArtwork.value.sourceKind === 'vector-fallback' ||
    catalogArtwork.value.mirrorSafe),
);

</script>

<template>
  <div
    class="device-artwork"
    :class="[`device-artwork--${family}`, { 'device-artwork--paired': paired, mirrored: mirrorArtwork }]"
    role="img"
    :aria-label="label"
  >
    <img
      v-if="catalogArtwork"
      class="device-artwork__catalog-asset"
      :class="{
        'device-artwork__exact-photo': artworkSourceKind === 'exact-photo',
        'device-artwork__vector-fallback': artworkSourceKind === 'vector-fallback',
      }"
      :src="catalogArtwork.src"
      alt=""
      aria-hidden="true"
    />
    <svg v-else-if="family === 'airpods' || family === 'earbuds'" viewBox="0 0 180 130" aria-hidden="true">
      <g v-if="componentType === 'case'" class="art-case">
        <template v-if="variant === 'galaxyBuds' || variant === 'pixelBuds'">
          <rect x="27" y="39" width="126" height="67" rx="27" />
          <path d="M27 64h126" />
          <circle cx="90" cy="74" r="3" />
        </template>
        <template v-else>
          <rect x="28" y="32" width="124" height="78" rx="31" />
          <path d="M28 63h124" />
          <circle cx="90" cy="72" r="3" />
        </template>
      </g>

      <g v-else-if="variant === 'airpodsPro'" class="art-pair-root">
        <g class="art-bud art-bud--left art-bud--pro">
          <ellipse cx="61" cy="48" rx="25" ry="28" />
          <rect x="57" y="59" width="15" height="43" rx="8" />
          <circle cx="46" cy="45" r="6" />
        </g>
        <g v-if="paired" class="art-bud art-bud--right art-bud--pro">
          <ellipse cx="119" cy="48" rx="25" ry="28" />
          <rect x="108" y="59" width="15" height="43" rx="8" />
          <circle cx="134" cy="45" r="6" />
        </g>
      </g>

      <g v-else-if="variant === 'airpodsClassic'" class="art-pair-root">
        <g class="art-bud art-bud--left art-bud--classic">
          <ellipse cx="61" cy="43" rx="22" ry="26" />
          <rect x="58" y="54" width="13" height="61" rx="7" />
          <circle cx="47" cy="41" r="5" />
        </g>
        <g v-if="paired" class="art-bud art-bud--right art-bud--classic">
          <ellipse cx="119" cy="43" rx="22" ry="26" />
          <rect x="109" y="54" width="13" height="61" rx="7" />
          <circle cx="133" cy="41" r="5" />
        </g>
      </g>

      <g v-else-if="variant === 'galaxyBuds'" class="art-pair-root">
        <g class="art-bud art-bud--left art-bud--bean">
          <path d="M33 60c0-25 17-43 39-43 17 0 29 11 29 27 0 22-18 45-42 52-16 5-26-9-26-36Z" />
          <circle cx="62" cy="54" r="6" />
        </g>
        <g v-if="paired" class="art-bud art-bud--right art-bud--bean">
          <path d="M147 60c0-25-17-43-39-43-17 0-29 11-29 27 0 22 18 45 42 52 16 5 26-9 26-36Z" />
          <circle cx="118" cy="54" r="6" />
        </g>
      </g>

      <g v-else-if="variant === 'pixelBuds'" class="art-pair-root">
        <g class="art-bud art-bud--left art-bud--round">
          <ellipse cx="61" cy="59" rx="31" ry="34" />
          <path class="art-outline" d="M36 44c-11-10-14-22-7-28 7-5 19 1 27 14" />
          <circle cx="63" cy="57" r="8" />
        </g>
        <g v-if="paired" class="art-bud art-bud--right art-bud--round">
          <ellipse cx="119" cy="59" rx="31" ry="34" />
          <path class="art-outline" d="M144 44c11-10 14-22 7-28-7-5-19 1-27 14" />
          <circle cx="117" cy="57" r="8" />
        </g>
      </g>

      <g v-else-if="variant === 'stemEarbuds'" class="art-pair-root">
        <g class="art-bud art-bud--left art-bud--stem">
          <path d="M35 47c0-18 13-31 31-31 17 0 28 10 28 25 0 16-12 28-28 28-18 0-31-8-31-22Z" />
          <path d="M67 60h17v52c0 9-17 9-17 0Z" />
          <circle cx="51" cy="45" r="6" />
        </g>
        <g v-if="paired" class="art-bud art-bud--right art-bud--stem">
          <path d="M145 47c0-18-13-31-31-31-17 0-28 10-28 25 0 16 12 28 28 28 18 0 31-8 31-22Z" />
          <path d="M96 60h17v52c0 9-17 9-17 0Z" />
          <circle cx="129" cy="45" r="6" />
        </g>
      </g>

      <g v-else class="art-pair-root">
        <g class="art-bud art-bud--left art-bud--generic">
          <ellipse cx="61" cy="59" rx="30" ry="34" />
          <circle cx="61" cy="58" r="8" />
        </g>
        <g v-if="paired" class="art-bud art-bud--right art-bud--generic">
          <ellipse cx="119" cy="59" rx="30" ry="34" />
          <circle cx="119" cy="58" r="8" />
        </g>
      </g>
    </svg>

    <svg v-else-if="family === 'headset'" viewBox="0 0 180 130" aria-hidden="true">
      <path class="art-outline" d="M36 72V63a54 54 0 0 1 108 0v9" />
      <rect class="art-fill" x="25" y="62" width="34" height="56" rx="17" />
      <rect class="art-fill" x="121" y="62" width="34" height="56" rx="17" />
    </svg>

    <svg v-else-if="family === 'speaker'" viewBox="0 0 180 130" aria-hidden="true">
      <rect class="art-fill" x="21" y="34" width="138" height="71" rx="28" />
      <circle class="art-cut" cx="58" cy="70" r="18" />
      <circle class="art-cut" cx="122" cy="70" r="18" />
      <circle class="art-outline" cx="90" cy="70" r="5" />
    </svg>

    <svg v-else-if="family === 'mouse'" viewBox="0 0 180 130" aria-hidden="true">
      <path class="art-fill" d="M90 17c-32 0-52 23-52 56 0 31 19 46 52 46s52-15 52-46c0-33-20-56-52-56Z" />
      <path class="art-cut" d="M90 18v45" />
      <rect class="art-cut" x="85" y="30" width="10" height="22" rx="5" />
    </svg>

    <svg v-else-if="family === 'keyboard'" viewBox="0 0 180 130" aria-hidden="true">
      <rect class="art-fill" x="17" y="34" width="146" height="72" rx="12" />
      <g class="art-keys">
        <path d="M31 49h118M31 65h118M31 81h118M44 43v50M61 43v50M78 43v50M95 43v50M112 43v50M129 43v50" />
      </g>
    </svg>

    <svg v-else-if="family === 'controller'" viewBox="0 0 180 130" aria-hidden="true">
      <path class="art-fill" d="M53 38h74c20 0 35 40 30 63-3 14-16 16-25 5l-17-20H65l-17 20c-9 11-22 9-25-5-5-23 10-63 30-63Z" />
      <path class="art-cut" d="M56 55v28M42 69h28" />
      <circle class="art-cut" cx="126" cy="61" r="6" />
      <circle class="art-cut" cx="141" cy="76" r="6" />
    </svg>

    <svg v-else-if="family === 'stylus'" viewBox="0 0 180 130" aria-hidden="true">
      <g transform="rotate(-28 90 65)">
        <rect class="art-fill" x="24" y="54" width="128" height="22" rx="11" />
        <path class="art-fill" d="m152 54 23 11-23 11Z" />
      </g>
    </svg>

    <svg v-else viewBox="0 0 180 130" aria-hidden="true">
      <circle class="art-fill" cx="90" cy="65" r="48" />
      <path class="bluetooth-mark" d="m85 31 27 23-22 18 22 20-27 22V31Zm8 18v17l10-9-10-8Zm0 32v17l10-8-10-9ZM67 48l45 42M67 87l45-39" />
    </svg>
  </div>
</template>
