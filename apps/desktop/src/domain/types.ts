export type ComponentType =
  | 'left'
  | 'right'
  | 'case'
  | 'headset'
  | 'aggregate'
  | 'unknown';

export type ChargingState = 'charging' | 'notCharging' | 'full' | 'unknown';

export type DataSource =
  | 'airPodsAdvertisement'
  | 'bluezBattery'
  | 'standardBleBattery'
  | 'windowsBattery'
  | 'serviceData'
  | 'hidBattery'
  | 'vendorProtocol'
  | 'aggregatePlatform';

export type DataConfidence = 'unknown' | 'low' | 'medium' | 'high' | 'verified';

export type ConnectionState = 'connected' | 'disconnected' | 'connecting' | 'unknown';
export type DeviceFamily =
  | 'airPods'
  | 'earbuds'
  | 'headset'
  | 'speaker'
  | 'mouse'
  | 'keyboard'
  | 'gameController'
  | 'stylus'
  | 'genericBle'
  | 'unknown';
export type Transport = 'lowEnergy' | 'classic' | 'dual' | 'unknown';
export type VisualConfidence = 'exact' | 'category' | 'fallback';
export type Capability =
  | 'earbudBattery'
  | 'caseBattery'
  | 'aggregateBattery'
  | 'chargingState';

export interface BatteryComponent {
  componentType: ComponentType;
  percentage: number | null;
  chargingState: ChargingState;
  updatedAt: string;
  source: DataSource;
  confidence: DataConfidence;
  stale: boolean;
}

export interface DeviceVisual {
  key: string;
  confidence: VisualConfidence;
}

export interface BluetoothAudioDevice {
  id: string;
  displayName: string;
  systemName: string | null;
  manufacturer: string | null;
  model: string | null;
  deviceFamily: DeviceFamily;
  /** Optional while older backends migrate to explicit visual metadata. */
  visual?: DeviceVisual;
  transport: Transport;
  connectionState: ConnectionState;
  lastSeenAt: string | null;
  lastUpdatedAt: string | null;
  capabilities: Capability[];
  components: BatteryComponent[];
}

export interface BackendStatus {
  platform: 'linux' | 'windows' | 'unsupported';
  available: boolean;
  adapterName: string | null;
  powered: boolean | null;
  discovering: boolean | null;
  detail: string;
}

export interface NotificationSettings {
  enabled: boolean;
  lowThreshold: number;
  criticalThreshold: number;
  componentSpecific: boolean;
  cooldownMinutes: number;
  connectionEvents: boolean;
}

export interface AppearanceSettings {
  theme: 'system' | 'light' | 'dark';
  transparency: number;
  compactLayout: boolean;
  animations: boolean;
  reducedMotion: boolean;
}

export interface AppSettings {
  schemaVersion: 1;
  startWithSystem: boolean;
  runInBackground: boolean;
  showTrayIcon: boolean;
  enableGnomeIntegration: boolean;
  enableDesktopWidget: boolean;
  updateMode: 'automatic' | 'notify' | 'manual';
  preferredDeviceId: string | null;
  hiddenDeviceIds: string[];
  appearance: AppearanceSettings;
  notifications: NotificationSettings;
}
