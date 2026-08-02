import { defaultSettings, normalizeSettings } from '../domain/settings.ts';
import type {
  AppSettings,
  BackendStatus,
  BluetoothAudioDevice,
} from '../domain/types.ts';

export const DEVICE_EVENT = 'airbattery://devices-changed';
export const BACKEND_STATUS_EVENT = 'airbattery://backend-status-changed';
export const NAVIGATE_EVENT = 'airbattery://navigate';

export interface DiagnosticBundle {
  path: string;
  createdAt: string;
}

export interface NativeRuntime {
  invoke<T>(command: string, args?: Record<string, unknown>): Promise<T>;
  listen<T>(event: string, handler: (payload: T) => void): Promise<() => void>;
}

export interface BackendClient {
  getDevices(): Promise<BluetoothAudioDevice[]>;
  getBackendStatus(): Promise<BackendStatus>;
  refreshDevices(): Promise<BluetoothAudioDevice[]>;
  runDiagnosticScan(): Promise<BluetoothAudioDevice[]>;
  getSettings(): Promise<AppSettings>;
  saveSettings(settings: AppSettings): Promise<AppSettings>;
  exportDiagnostics(): Promise<DiagnosticBundle>;
  openWidget(): Promise<void>;
  openSettings(): Promise<void>;
  listenForDeviceChanges(
    handler: (devices: BluetoothAudioDevice[]) => void,
  ): Promise<() => void>;
  listenForBackendStatus(
    handler: (status: BackendStatus) => void,
  ): Promise<() => void>;
  listenForNavigation(handler: (route: string) => void): Promise<() => void>;
}

const unavailableStatus: BackendStatus = {
  platform: 'unsupported',
  available: false,
  adapterName: null,
  powered: null,
  discovering: null,
  detail: 'Open AirBattery as a desktop application to access Bluetooth.',
};

export function createBackendClient(runtime: NativeRuntime | null): BackendClient {
  if (runtime === null) {
    return {
      async getDevices() {
        return [];
      },
      async getBackendStatus() {
        return { ...unavailableStatus };
      },
      async refreshDevices() {
        return [];
      },
      async runDiagnosticScan() {
        return [];
      },
      async getSettings() {
        return structuredClone(defaultSettings);
      },
      async saveSettings(settings) {
        return normalizeSettings(settings);
      },
      async exportDiagnostics() {
        throw new Error('Diagnostic export requires the AirBattery desktop backend.');
      },
      async openWidget() {
        throw new Error('Desktop widget requires the AirBattery desktop backend.');
      },
      async openSettings() {
        throw new Error('Settings window requires the AirBattery desktop backend.');
      },
      async listenForDeviceChanges() {
        return () => undefined;
      },
      async listenForBackendStatus() {
        return () => undefined;
      },
      async listenForNavigation() {
        return () => undefined;
      },
    };
  }

  return {
    getDevices: () => runtime.invoke<BluetoothAudioDevice[]>('get_devices'),
    getBackendStatus: () => runtime.invoke<BackendStatus>('get_backend_status'),
    refreshDevices: () => runtime.invoke<BluetoothAudioDevice[]>('refresh_devices'),
    runDiagnosticScan: () => runtime.invoke<BluetoothAudioDevice[]>('run_diagnostic_scan'),
    getSettings: async () => normalizeSettings(await runtime.invoke<unknown>('get_settings')),
    saveSettings: (settings) =>
      runtime.invoke<AppSettings>('save_settings', { settings: normalizeSettings(settings) }),
    exportDiagnostics: () => runtime.invoke<DiagnosticBundle>('export_diagnostics'),
    openWidget: () => runtime.invoke<void>('open_widget'),
    openSettings: () => runtime.invoke<void>('open_settings'),
    listenForDeviceChanges: (handler) =>
      runtime.listen<BluetoothAudioDevice[]>(DEVICE_EVENT, handler),
    listenForBackendStatus: (handler) =>
      runtime.listen<BackendStatus>(BACKEND_STATUS_EVENT, handler),
    listenForNavigation: (handler) => runtime.listen<string>(NAVIGATE_EVENT, handler),
  };
}

let clientPromise: Promise<BackendClient> | null = null;

async function detectNativeRuntime(): Promise<NativeRuntime | null> {
  if (typeof window === 'undefined' || window.__TAURI_INTERNALS__ === undefined) return null;

  const [{ invoke }, { listen }] = await Promise.all([
    import('@tauri-apps/api/core'),
    import('@tauri-apps/api/event'),
  ]);

  return {
    invoke,
    async listen<T>(event: string, handler: (payload: T) => void) {
      return listen<T>(event, (message) => handler(message.payload));
    },
  };
}

export async function getBackendClient(): Promise<BackendClient> {
  clientPromise ??= detectNativeRuntime().then(createBackendClient);
  return clientPromise;
}

export async function exportDiagnostics(): Promise<DiagnosticBundle> {
  const client = await getBackendClient();
  return client.exportDiagnostics();
}

export async function openWidget(): Promise<void> {
  const client = await getBackendClient();
  await client.openWidget();
}

