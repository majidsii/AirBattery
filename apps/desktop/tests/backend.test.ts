import assert from 'node:assert/strict';
import test from 'node:test';

import {
  BACKEND_STATUS_EVENT,
  createBackendClient,
  DEVICE_EVENT,
  NAVIGATE_EVENT,
} from '../src/api/backend.ts';
import { defaultSettings } from '../src/domain/settings.ts';

test('browser client reports unavailable backend and never invents devices', async () => {
  const client = createBackendClient(null);

  assert.deepEqual(await client.getDevices(), []);
  assert.deepEqual(await client.refreshDevices(), []);
  assert.deepEqual(await client.getBackendStatus(), {
    platform: 'unsupported',
    available: false,
    adapterName: null,
    powered: null,
    discovering: null,
    detail: 'Open AirBattery as a desktop application to access Bluetooth.',
  });
  assert.deepEqual(await client.getSettings(), defaultSettings);
});

test('native client delegates commands and live device events', async () => {
  const calls: Array<{ command: string; args?: Record<string, unknown> }> = [];
  let eventName = '';
  let deliveredPayload: unknown;
  const runtime = {
    async invoke<T>(command: string, args?: Record<string, unknown>): Promise<T> {
      calls.push({ command, args });
      if (command === 'get_devices') return [] as T;
      if (command === 'get_backend_status') {
        return {
          platform: 'linux',
          available: true,
          adapterName: 'hci0',
          powered: true,
          discovering: false,
          detail: 'BlueZ ready',
        } as T;
      }
      throw new Error(`Unexpected command ${command}`);
    },
    async listen<T>(event: string, handler: (payload: T) => void): Promise<() => void> {
      eventName = event;
      handler([] as T);
      return () => undefined;
    },
  };

  const client = createBackendClient(runtime);
  await client.getDevices();
  const status = await client.getBackendStatus();
  const unlisten = await client.listenForDeviceChanges((payload) => {
    deliveredPayload = payload;
  });

  assert.deepEqual(calls, [
    { command: 'get_devices', args: undefined },
    { command: 'get_backend_status', args: undefined },
  ]);
  assert.equal(status.platform, 'linux');
  assert.equal(eventName, DEVICE_EVENT);
  assert.deepEqual(deliveredPayload, []);
  assert.equal(typeof unlisten, 'function');
});


test('native client delivers navigation events without exposing window APIs', async () => {
  let observedEvent = '';
  let observedRoute = '';
  const runtime = {
    async invoke<T>(): Promise<T> {
      throw new Error('invoke should not be used');
    },
    async listen<T>(event: string, handler: (payload: T) => void): Promise<() => void> {
      observedEvent = event;
      handler('settings' as T);
      return () => undefined;
    },
  };

  const client = createBackendClient(runtime);
  await client.listenForNavigation((route) => { observedRoute = route; });

  assert.equal(observedEvent, NAVIGATE_EVENT);
  assert.equal(observedRoute, 'settings');
});


test('native client delivers backend health events', async () => {
  let observedEvent = '';
  let observedPlatform = '';
  const runtime = {
    async invoke<T>(): Promise<T> {
      throw new Error('invoke should not be used');
    },
    async listen<T>(event: string, handler: (payload: T) => void): Promise<() => void> {
      observedEvent = event;
      handler({ platform: 'linux' } as T);
      return () => undefined;
    },
  };

  const client = createBackendClient(runtime);
  await client.listenForBackendStatus((status) => { observedPlatform = status.platform; });

  assert.equal(observedEvent, BACKEND_STATUS_EVENT);
  assert.equal(observedPlatform, 'linux');
});
