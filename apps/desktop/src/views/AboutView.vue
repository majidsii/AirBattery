<script setup lang="ts">
import BrandMark from '../components/BrandMark.vue';
</script>

<template>
  <section class="page about-page" aria-labelledby="about-title">
    <div class="about-hero glass-card">
      <div class="about-logo" aria-hidden="true"><BrandMark decorative /></div>
      <p class="eyebrow">Local-first Bluetooth battery monitoring</p>
      <h1 id="about-title">AirBattery</h1>
      <p class="about-version">Version 0.1.0-alpha.1</p>
      <p>Accurate battery state for Linux and Windows, with component-level AirPods support where the device and operating system provide reliable data.</p>
      <div class="about-actions">
        <a class="primary-button" href="https://github.com/majidsii/AirBattery" target="_blank" rel="noreferrer">Repository</a>
        <a class="secondary-button" href="https://github.com/majidsii/AirBattery/blob/main/LICENSE" target="_blank" rel="noreferrer">MIT License</a>
      </div>
    </div>
    <div class="about-principles">
      <article class="glass-card"><strong>No telemetry</strong><p>Battery monitoring stays on your computer.</p></article>
      <article class="glass-card"><strong>No invented values</strong><p>Unavailable and stale data are labelled honestly.</p></article>
      <article class="glass-card"><strong>Exact-or-generic artwork</strong><p>AirBattery never substitutes a similar product photo for the detected model.</p></article>
    </div>
  </section>
</template>
