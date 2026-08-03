/**
 * Registry for exact, pre-rendered product artwork.
 *
 * Final product assets must be rendered offline from a licensed, model-accurate
 * source. The registry is intentionally empty until an asset set passes visual,
 * model, orientation, transparency, and licensing review.
 */
export type PreRenderedArtworkMode = 'left' | 'right' | 'pair' | 'case';

export interface PreRenderedArtworkAsset {
  src: string;
  modelKey: string;
  mode: PreRenderedArtworkMode;
  license: string;
  author: string;
  source: string;
  reviewedAt: string;
  renderMethod: 'offline-3d';
}

export interface PreRenderedArtworkAssetSet {
  left?: PreRenderedArtworkAsset;
  right?: PreRenderedArtworkAsset;
  pair?: PreRenderedArtworkAsset;
  case?: PreRenderedArtworkAsset;
}

export type PreRenderedArtworkAssetMap = Record<string, PreRenderedArtworkAssetSet>;

export const preRenderedArtworkAssets: PreRenderedArtworkAssetMap = {};
