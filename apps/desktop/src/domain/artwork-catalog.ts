import { preRenderedArtworkAssets } from './pre-rendered-artwork.ts';

const REVIEW_DATE = /^\d{4}-\d{2}-\d{2}$/;

export type DeviceArtworkMode = 'left' | 'right' | 'pair' | 'case';

export interface DeviceArtworkAsset {
  src: string;
  sourceKind: 'pre-rendered-3d';
}

/**
 * Resolves only artwork that passed the exact-model review gate.
 *
 * The caller renders a neutral category glyph when this function returns null.
 * Approximate family drawings and model substitutions are intentionally absent.
 */
export function resolvePreRenderedArtworkAsset(
  artworkKey: string,
  mode: DeviceArtworkMode,
): DeviceArtworkAsset | null {
  const normalizedKey = artworkKey.trim().toLowerCase();
  const candidate = preRenderedArtworkAssets[normalizedKey]?.[mode];
  if (!candidate || candidate.modelKey !== normalizedKey || candidate.mode !== mode) return null;
  if (candidate.renderMethod !== 'offline-3d') return null;
  if (!candidate.src.trim() || !candidate.license.trim() || !candidate.author.trim()) return null;
  if (!candidate.source.trim() || !REVIEW_DATE.test(candidate.reviewedAt)) return null;
  return {
    src: candidate.src,
    sourceKind: 'pre-rendered-3d',
  };
}
