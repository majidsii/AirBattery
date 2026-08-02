<script setup lang="ts">
import attributionJson from '../assets/device-artwork/exact/ATTRIBUTION.generated.json';

interface ProductPhotoAttribution {
  id: string;
  artworkKey: string;
  mode: string;
  sourcePage: string;
  author: string;
  license: string;
  licenseUrl: string;
  modifications: string[];
}

const productPhotoAttributions = (
  attributionJson as { sources: ProductPhotoAttribution[] }
).sources;
</script>

<template>
  <section class="page about-page" aria-labelledby="about-title">
    <div class="about-hero glass-card">
      <div class="about-logo" aria-hidden="true">A</div>
      <p class="eyebrow">Local-first Bluetooth battery monitoring</p>
      <h1 id="about-title">AirBattery</h1>
      <p class="about-version">Version 0.1.0-alpha.1</p>
      <p>Accurate battery state for Linux and Windows, with component-level AirPods support where the device and operating system provide reliable data.</p>
      <div class="about-actions">
        <a class="primary-button" href="https://github.com/airbattery/airbattery" target="_blank" rel="noreferrer">Repository</a>
        <a class="secondary-button" href="https://github.com/airbattery/airbattery/blob/main/LICENSE" target="_blank" rel="noreferrer">MIT License</a>
      </div>
    </div>
    <div class="about-principles">
      <article class="glass-card"><strong>No telemetry</strong><p>Battery monitoring stays on your computer.</p></article>
      <article class="glass-card"><strong>No invented values</strong><p>Unavailable and stale data are labelled honestly.</p></article>
      <article class="glass-card"><strong>Open architecture</strong><p>Device protocols and native backends remain extensible.</p></article>
    </div>

    <details v-if="productPhotoAttributions.length > 0" class="glass-card artwork-attribution-card">
      <summary>Product photo credits ({{ productPhotoAttributions.length }})</summary>
      <p>Exact product photographs remain under their individual Creative Commons licenses. AirBattery crops, proportionally resizes, and converts them to WebP without changing product geometry.</p>
      <ul class="artwork-attribution-list">
        <li v-for="item in productPhotoAttributions" :key="item.id">
          <strong>{{ item.artworkKey }} · {{ item.mode }}</strong>
          <span>Photo by {{ item.author }} · {{ item.license }}</span>
          <span>{{ item.modifications.join(', ') }}</span>
          <div class="artwork-attribution-links">
            <a :href="item.sourcePage" target="_blank" rel="noreferrer">Source</a>
            <a :href="item.licenseUrl" target="_blank" rel="noreferrer">License</a>
          </div>
        </li>
      </ul>
    </details>
  </section>
</template>
