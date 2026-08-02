<script setup lang="ts">
import { summarizeDevice } from '../domain/presentation.ts';
import { useDeviceStore } from '../stores/devices.ts';
import { useSettingsStore } from '../stores/settings.ts';
import StatusPill from '../components/StatusPill.vue';
import EmptyState from '../components/EmptyState.vue';

const devices = useDeviceStore();
const settings = useSettingsStore();

async function prefer(deviceId: string): Promise<void> {
  settings.value.preferredDeviceId = deviceId;
  await settings.persist();
}

async function hide(deviceId: string): Promise<void> {
  settings.hideDevice(deviceId);
  await settings.persist();
}
</script>

<template>
  <section class="page" aria-labelledby="devices-title">
    <header class="page-header">
      <div>
        <p class="eyebrow">Known hardware</p>
        <h1 id="devices-title">Devices</h1>
      </div>
      <button class="secondary-button" type="button" :disabled="devices.loading" @click="devices.refresh">
        {{ devices.loading ? 'Refreshing…' : 'Refresh' }}
      </button>
    </header>

    <div v-if="devices.visibleDevices.length" class="device-list">
      <article v-for="device in devices.visibleDevices" :key="device.id" class="device-row glass-card">
        <div class="device-avatar" :data-family="device.deviceFamily" aria-hidden="true">⌁</div>
        <div class="device-row__body">
          <div class="device-row__title">
            <h2>{{ device.displayName }}</h2>
            <StatusPill :state="device.connectionState" />
          </div>
          <p>{{ summarizeDevice(device) || 'Battery capability not currently available' }}</p>
          <div class="tag-list" aria-label="Device capabilities">
            <span v-for="capability in device.capabilities" :key="capability">{{ capability }}</span>
            <span>{{ device.transport }}</span>
          </div>
        </div>
        <div class="device-row__actions">
          <button
            class="text-button"
            type="button"
            :aria-pressed="settings.value.preferredDeviceId === device.id"
            @click="prefer(device.id)"
          >
            {{ settings.value.preferredDeviceId === device.id ? 'Preferred' : 'Set preferred' }}
          </button>
          <button class="text-button danger-text" type="button" @click="hide(device.id)">Hide</button>
        </div>
      </article>
    </div>
    <EmptyState
      v-else
      title="No visible devices"
      detail="AirBattery only lists devices reported by the operating system. Hidden devices can be restored from Settings."
      action-label="Refresh devices"
      :busy="devices.loading"
      @action="devices.refresh"
    />
  </section>
</template>
