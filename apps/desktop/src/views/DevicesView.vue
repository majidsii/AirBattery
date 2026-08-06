<script setup lang="ts">
import { computed, ref } from 'vue';

import BrandMark from '../components/BrandMark.vue';
import EmptyState from '../components/EmptyState.vue';
import StatusPill from '../components/StatusPill.vue';
import {
  deviceBatteryTone,
  splitDeviceSections,
} from '../domain/device-sections.ts';
import { summarizeDevice } from '../domain/presentation.ts';
import type { BluetoothAudioDevice } from '../domain/types.ts';
import { useDeviceStore } from '../stores/devices.ts';
import { useSettingsStore } from '../stores/settings.ts';

const devices = useDeviceStore();
const settings = useSettingsStore();
const showKnownDevices = ref(false);

function preferredFirst(
  source: readonly BluetoothAudioDevice[],
): BluetoothAudioDevice[] {
  const preferredId = settings.value.preferredDeviceId;

  return [...source].sort((left, right) => {
    const preferredOrder =
      Number(right.id === preferredId) -
      Number(left.id === preferredId);

    if (preferredOrder !== 0) return preferredOrder;

    return left.displayName.localeCompare(right.displayName);
  });
}

const deviceSections = computed(() =>
  splitDeviceSections(devices.visibleDevices),
);

const activeDevices = computed(() =>
  preferredFirst(deviceSections.value.active),
);

const knownDevices = computed(() =>
  preferredFirst(deviceSections.value.known),
);

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

      <button
        class="secondary-button"
        type="button"
        :disabled="devices.loading"
        @click="devices.refresh"
      >
        {{ devices.loading ? 'Refreshing…' : 'Refresh' }}
      </button>
    </header>

    <div v-if="devices.visibleDevices.length" class="device-sections">
      <section
        class="device-section"
        aria-labelledby="active-devices-heading"
      >
        <header class="device-section__heading">
          <div>
            <h2 id="active-devices-heading">Active devices</h2>
            <p>Connected hardware available to AirBattery right now.</p>
          </div>

          <span class="device-section__count">
            {{ activeDevices.length }}
          </span>
        </header>

        <div v-if="activeDevices.length" class="device-list">
          <article
            v-for="device in activeDevices"
            :key="device.id"
            class="device-row glass-card"
          >
            <div
              class="device-avatar"
              :class="`device-avatar--${deviceBatteryTone(device)}`"
              :data-family="device.deviceFamily"
              :data-battery-tone="deviceBatteryTone(device)"
              aria-hidden="true"
            >
              <BrandMark decorative />
            </div>

            <div class="device-row__body">
              <div class="device-row__title">
                <h2>{{ device.displayName }}</h2>
                <StatusPill :state="device.connectionState" />
              </div>

              <p>
                {{ summarizeDevice(device) || 'Battery capability not currently available' }}
              </p>

              <div class="tag-list" aria-label="Device capabilities">
                <span
                  v-for="capability in device.capabilities"
                  :key="capability"
                >
                  {{ capability }}
                </span>
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

              <button
                class="text-button danger-text"
                type="button"
                @click="hide(device.id)"
              >
                Hide
              </button>
            </div>
          </article>
        </div>

        <div v-else class="device-active-empty glass-card">
          <BrandMark decorative />
          <div>
            <strong>No active devices</strong>
            <p>Connect a known Bluetooth device, then refresh this page.</p>
          </div>
        </div>
      </section>

      <section
        v-if="knownDevices.length"
        class="device-section device-section--known"
        aria-labelledby="known-devices-heading"
      >
        <button
          class="device-section__toggle"
          type="button"
          :aria-expanded="showKnownDevices"
          aria-controls="known-device-list"
          @click="showKnownDevices = !showKnownDevices"
        >
          <span class="device-section__toggle-copy">
            <strong id="known-devices-heading">Known devices</strong>
            <small>Disconnected devices remain available for management.</small>
          </span>

          <span class="device-section__toggle-meta">
            <span class="device-section__count">
              {{ knownDevices.length }}
            </span>

            <svg
              class="device-section__chevron"
              :class="{ 'is-open': showKnownDevices }"
              viewBox="0 0 16 16"
              fill="none"
              aria-hidden="true"
            >
              <path
                d="m4 6 4 4 4-4"
                stroke="currentColor"
                stroke-width="1.7"
                stroke-linecap="round"
                stroke-linejoin="round"
              />
            </svg>
          </span>
        </button>

        <div
          v-show="showKnownDevices"
          id="known-device-list"
          class="device-list device-list--known"
        >
          <article
            v-for="device in knownDevices"
            :key="device.id"
            class="device-row glass-card"
          >
            <div
              class="device-avatar"
              :class="`device-avatar--${deviceBatteryTone(device)}`"
              :data-family="device.deviceFamily"
              :data-battery-tone="deviceBatteryTone(device)"
              aria-hidden="true"
            >
              <BrandMark decorative />
            </div>

            <div class="device-row__body">
              <div class="device-row__title">
                <h2>{{ device.displayName }}</h2>
                <StatusPill :state="device.connectionState" />
              </div>

              <p>
                {{ summarizeDevice(device) || 'Battery capability not currently available' }}
              </p>

              <div class="tag-list" aria-label="Device capabilities">
                <span
                  v-for="capability in device.capabilities"
                  :key="capability"
                >
                  {{ capability }}
                </span>
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

              <button
                class="text-button danger-text"
                type="button"
                @click="hide(device.id)"
              >
                Hide
              </button>
            </div>
          </article>
        </div>
      </section>
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
