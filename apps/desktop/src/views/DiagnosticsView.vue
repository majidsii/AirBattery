<script setup lang="ts">
import { computed, ref } from 'vue';

import { exportDiagnostics } from '../api/backend.ts';
import { selectDeviceArtwork } from '../domain/presentation.ts';
import { useDeviceStore } from '../stores/devices.ts';

const devices = useDeviceStore();
const exportedPath = ref<string | null>(null);
const error = ref<string | null>(null);
const exporting = ref(false);
const scanning = ref(false);

const componentRows = computed(() =>
  devices.devices.flatMap((device) =>
    device.components.map((component) => ({
      deviceId: device.id,
      deviceName: device.displayName,
      family: device.deviceFamily,
      visual: selectDeviceArtwork(device),
      component: component.componentType,
      percentage: component.percentage,
      charging: component.chargingState,
      source: component.source,
      confidence: component.confidence,
      stale: component.stale,
    })),
  ),
);

async function runScan(): Promise<void> {
  scanning.value = true;
  error.value = null;
  try {
    await devices.runDiagnosticScan();
    if (devices.error) throw new Error(devices.error);
  } catch (reason) {
    error.value = reason instanceof Error ? reason.message : String(reason);
  } finally {
    scanning.value = false;
  }
}

async function exportBundle(): Promise<void> {
  exporting.value = true;
  error.value = null;
  try {
    exportedPath.value = (await exportDiagnostics()).path;
  } catch (reason) {
    error.value = reason instanceof Error ? reason.message : String(reason);
  } finally {
    exporting.value = false;
  }
}
</script>

<template>
  <section class="page" aria-labelledby="diagnostics-title">
    <header class="page-header diagnostics-header">
      <div>
        <p class="eyebrow">Local support data</p>
        <h1 id="diagnostics-title">Diagnostics</h1>
      </div>
      <div class="header-actions">
        <button class="secondary-button" type="button" :disabled="scanning" @click="runScan">
          {{ scanning ? 'Scanning for 30 seconds…' : 'Run 30-second device scan' }}
        </button>
        <button class="primary-button" type="button" :disabled="exporting" @click="exportBundle">
          {{ exporting ? 'Exporting…' : 'Export sanitized report' }}
        </button>
      </div>
    </header>

    <p v-if="error" class="inline-error" role="alert">{{ error }}</p>
    <p v-if="exportedPath" class="inline-success" role="status">Saved to {{ exportedPath }}</p>

    <div class="diagnostic-grid">
      <article class="metric-card glass-card">
        <span>Backend</span>
        <strong>{{ devices.backendStatus?.available ? 'Available' : 'Unavailable' }}</strong>
        <small>{{ devices.backendStatus?.detail ?? 'Not initialized' }}</small>
      </article>
      <article class="metric-card glass-card">
        <span>Adapter</span>
        <strong>{{ devices.backendStatus?.adapterName ?? '—' }}</strong>
        <small>Powered: {{ devices.backendStatus?.powered ?? 'unknown' }}</small>
      </article>
      <article class="metric-card glass-card">
        <span>Known devices</span>
        <strong>{{ devices.devices.length }}</strong>
        <small>Known devices plus bounded BLE advertisements</small>
      </article>
      <article class="metric-card glass-card">
        <span>Data sources</span>
        <strong>{{ new Set(componentRows.map((row) => row.source)).size }}</strong>
        <small>Selected normalized battery providers</small>
      </article>
    </div>

    <section class="diagnostic-details glass-card">
      <h2>Selected battery evidence</h2>
      <p>
        These are the exact values chosen by the component resolver. Missing values remain unavailable;
        an overall percentage is never copied into left, right, or case.
      </p>
      <div class="diagnostic-table-wrap">
        <table class="diagnostic-table">
          <thead>
            <tr>
              <th>Device</th>
              <th>Artwork</th>
              <th>Component</th>
              <th>Battery</th>
              <th>Charging</th>
              <th>Source</th>
              <th>Confidence</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="row in componentRows" :key="`${row.deviceId}:${row.component}`">
              <td>{{ row.deviceName }}<small>{{ row.family }}</small></td>
              <td><code>{{ row.visual }}</code></td>
              <td>{{ row.component }}</td>
              <td>{{ row.percentage === null ? '—' : `${row.percentage}%` }}<small v-if="row.stale">last known</small></td>
              <td>{{ row.charging }}</td>
              <td><code>{{ row.source }}</code></td>
              <td>{{ row.confidence }}</td>
            </tr>
            <tr v-if="componentRows.length === 0">
              <td colspan="7">No battery evidence has been received yet.</td>
            </tr>
          </tbody>
        </table>
      </div>

      <h2>Privacy</h2>
      <p>
        The exported report contains privacy-safe identifiers, selected component provenance, model
        classification, and artwork keys. It excludes Bluetooth addresses and raw manufacturer payloads.
      </p>

      <h2>AirPods hardware validation</h2>
      <code>cargo run -p airbattery-cli -- scan --seconds 30 --json</code>
      <p>
        Start the scan first, then open the case near the computer. Repeat with only the left earbud removed,
        only the right removed, both removed, and while each component is charging.
      </p>
    </section>
  </section>
</template>
