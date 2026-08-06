export type NamedGlassPreset = 'off' | 'subtle' | 'balanced' | 'deep' | 'crystal';
export type GlassPreset = NamedGlassPreset | 'custom';
export type GlassSurface = 'main' | 'widget' | 'popup';

export interface GlassSurfaceStrengths {
  main: number;
  widget: number;
  popup: number;
}

export interface GlassPreferences {
  preset: GlassPreset;
  intensity: number;
  adaptive: boolean;
  reduceTransparency: boolean;
  surfaces: GlassSurfaceStrengths;
}

export const GLASS_STORAGE_KEY = 'airbattery.glass.v1';

export const glassPresetOptions: ReadonlyArray<{
  value: NamedGlassPreset;
  label: string;
  description: string;
}> = [
  { value: 'off', label: 'Off', description: 'Solid, lightweight surfaces.' },
  { value: 'subtle', label: 'Subtle', description: 'A quiet translucent finish.' },
  { value: 'balanced', label: 'Balanced', description: 'Clear hierarchy with restrained depth.' },
  { value: 'deep', label: 'Deep', description: 'Richer blur and stronger highlights.' },
  { value: 'crystal', label: 'Crystal', description: 'Maximum glass depth and refraction.' },
];

const presetValues: Record<NamedGlassPreset, Omit<GlassPreferences, 'preset'>> = {
  off: {
    intensity: 0,
    adaptive: false,
    reduceTransparency: true,
    surfaces: { main: 0, widget: 0, popup: 0 },
  },
  subtle: {
    intensity: 32,
    adaptive: true,
    reduceTransparency: false,
    surfaces: { main: 24, widget: 32, popup: 40 },
  },
  balanced: {
    intensity: 62,
    adaptive: true,
    reduceTransparency: false,
    surfaces: { main: 56, widget: 66, popup: 74 },
  },
  deep: {
    intensity: 82,
    adaptive: true,
    reduceTransparency: false,
    surfaces: { main: 76, widget: 86, popup: 92 },
  },
  crystal: {
    intensity: 96,
    adaptive: true,
    reduceTransparency: false,
    surfaces: { main: 90, widget: 96, popup: 100 },
  },
};

export const defaultGlassPreferences: GlassPreferences = {
  preset: 'balanced',
  ...structuredClone(presetValues.balanced),
};

type UnknownRecord = Record<string, unknown>;

function isRecord(value: unknown): value is UnknownRecord {
  return typeof value === 'object' && value !== null && !Array.isArray(value);
}

function clampPercentage(value: unknown, fallback: number): number {
  if (typeof value !== 'number' || !Number.isFinite(value)) return fallback;
  return Math.min(100, Math.max(0, Math.round(value)));
}

function bool(value: unknown, fallback: boolean): boolean {
  return typeof value === 'boolean' ? value : fallback;
}

function isGlassPreset(value: unknown): value is GlassPreset {
  return value === 'off' || value === 'subtle' || value === 'balanced'
    || value === 'deep' || value === 'crystal' || value === 'custom';
}

export function normalizeGlassPreferences(input: unknown): GlassPreferences {
  if (!isRecord(input)) return structuredClone(defaultGlassPreferences);

  const surfaces = isRecord(input.surfaces) ? input.surfaces : {};
  return {
    preset: isGlassPreset(input.preset) ? input.preset : defaultGlassPreferences.preset,
    intensity: clampPercentage(input.intensity, defaultGlassPreferences.intensity),
    adaptive: bool(input.adaptive, defaultGlassPreferences.adaptive),
    reduceTransparency: bool(
      input.reduceTransparency,
      defaultGlassPreferences.reduceTransparency,
    ),
    surfaces: {
      main: clampPercentage(surfaces.main, defaultGlassPreferences.surfaces.main),
      widget: clampPercentage(surfaces.widget, defaultGlassPreferences.surfaces.widget),
      popup: clampPercentage(surfaces.popup, defaultGlassPreferences.surfaces.popup),
    },
  };
}

export function applyGlassPreset(
  current: GlassPreferences,
  preset: NamedGlassPreset,
): GlassPreferences {
  return normalizeGlassPreferences({
    ...current,
    preset,
    ...structuredClone(presetValues[preset]),
  });
}

export function setGlassIntensity(
  current: GlassPreferences,
  intensity: number,
): GlassPreferences {
  const value = clampPercentage(intensity, current.intensity);
  return normalizeGlassPreferences({
    ...current,
    preset: 'custom',
    intensity: value,
    surfaces: {
      main: Math.max(0, value - 6),
      widget: Math.min(100, value + 4),
      popup: Math.min(100, value + 12),
    },
  });
}

export function setGlassSurface(
  current: GlassPreferences,
  surface: GlassSurface,
  strength: number,
): GlassPreferences {
  return normalizeGlassPreferences({
    ...current,
    preset: 'custom',
    surfaces: {
      ...current.surfaces,
      [surface]: strength,
    },
  });
}

function materialVariables(strength: number, prefix: string): Record<string, string> {
  const ratio = strength / 100;
  // Stronger glass is optically clearer, not more opaque: blur, edge light,
  // and refraction increase while the tint layer becomes more transparent.
  const blur = Math.round(8 + ratio * 34);
  const alpha = 0.58 - ratio * 0.28;
  const stroke = 0.18 + ratio * 0.32;
  const refraction = 0.10 + ratio * 0.30;
  const highlight = 0.10 + ratio * 0.24;
  return {
    [`--glass-${prefix}-strength`]: ratio.toFixed(2),
    [`--glass-${prefix}-blur`]: `${blur}px`,
    [`--glass-${prefix}-alpha`]: alpha.toFixed(2),
    [`--glass-${prefix}-stroke`]: stroke.toFixed(2),
    [`--glass-${prefix}-refraction`]: refraction.toFixed(2),
    [`--glass-${prefix}-highlight`]: highlight.toFixed(2),
  };
}

export function glassCssVariables(preferences: GlassPreferences): Record<string, string> {
  const normalized = normalizeGlassPreferences(preferences);
  const effectiveIntensity = normalized.reduceTransparency ? 0 : normalized.intensity;
  return {
    '--glass-intensity': (effectiveIntensity / 100).toFixed(2),
    '--glass-saturation': `${Math.round(100 + effectiveIntensity * 0.72)}%`,
    '--glass-luminance': (0.05 + effectiveIntensity / 1000).toFixed(2),
    ...materialVariables(normalized.reduceTransparency ? 0 : normalized.surfaces.main, 'main'),
    ...materialVariables(normalized.reduceTransparency ? 0 : normalized.surfaces.widget, 'widget'),
    ...materialVariables(normalized.reduceTransparency ? 0 : normalized.surfaces.popup, 'popup'),
  };
}

interface StorageReader {
  getItem(key: string): string | null;
}

interface StorageWriter {
  setItem(key: string, value: string): void;
}

export function loadGlassPreferences(storage: StorageReader | null): GlassPreferences {
  if (storage === null) return structuredClone(defaultGlassPreferences);
  try {
    const value = storage.getItem(GLASS_STORAGE_KEY);
    return value === null
      ? structuredClone(defaultGlassPreferences)
      : normalizeGlassPreferences(JSON.parse(value));
  } catch {
    return structuredClone(defaultGlassPreferences);
  }
}

export function saveGlassPreferences(
  storage: StorageWriter | null,
  preferences: GlassPreferences,
): void {
  if (storage === null) return;
  try {
    storage.setItem(GLASS_STORAGE_KEY, JSON.stringify(normalizeGlassPreferences(preferences)));
  } catch {
    // Appearance persistence must never prevent the native settings save.
  }
}
