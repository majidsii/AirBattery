import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import test from 'node:test';

const catalogSource = readFileSync(new URL('../src/domain/artwork-catalog.ts', import.meta.url), 'utf8');
const keySource = readFileSync(new URL('../src/domain/artwork-keys.ts', import.meta.url), 'utf8');
const registrySource = readFileSync(new URL('../src/domain/pre-rendered-artwork.ts', import.meta.url), 'utf8');
const componentSource = readFileSync(new URL('../src/components/DeviceArtwork.vue', import.meta.url), 'utf8');

const requiredKeys = [
  'airpods-pro-1', 'airpods-4', 'qcy-t13', 'qcy-melobuds', 'qcy-crossky',
  'qcy-melobuds-n70', 'qcy-ailybuds-pro-plus', 'qcy-h3-pro', 'xiaomi-buds-5',
  'redmi-buds-6', 'soundcore-liberty', 'galaxy-buds3', 'sony-linkbuds-open',
  'sony-wf-1000xm4', 'soundcore-sport-x20', 'jbl-free', 'jbl-tour-pro',
  'pixel-buds-pro', 'nothing-ear', 'oneplus-buds-pro', 'huawei-freeclip',
  'beats-fit-pro',
];

test('model identity remains available for future exact reviewed artwork', () => {
  for (const key of requiredKeys) assert.match(keySource, new RegExp(`'${key}'`));
  assert.match(componentSource, /resolvePreRenderedArtworkAsset/);
});

test('unreviewed model drawings and product photos are absent from production assets', () => {
  assert.equal(existsSync(new URL('../src/assets/device-artwork/catalog/', import.meta.url)), false);
  assert.equal(existsSync(new URL('../src/assets/device-artwork/exact/', import.meta.url)), false);
  assert.equal(existsSync(new URL('../src/assets/device-artwork/studio/', import.meta.url)), false);
  assert.doesNotMatch(catalogSource, /device-artwork\/catalog/);
  assert.doesNotMatch(catalogSource, /\.svg'/);
  assert.doesNotMatch(catalogSource, /\.webp'/);
});

test('pre-rendered assets are exact-model only and start behind an empty review gate', () => {
  assert.match(catalogSource, /preRenderedArtworkAssets/);
  assert.match(catalogSource, /candidate\.modelKey !== normalizedKey/);
  assert.match(catalogSource, /candidate\.mode !== mode/);
  assert.match(catalogSource, /candidate\.renderMethod !== 'offline-3d'/);
  assert.match(catalogSource, /candidate\.source\.trim\(\)/);
  assert.match(catalogSource, /REVIEW_DATE\.test\(candidate\.reviewedAt\)/);
  assert.match(registrySource, /preRenderedArtworkAssets: PreRenderedArtworkAssetMap = \{\}/);
  assert.doesNotMatch(registrySource, /\.webp'/);
  assert.doesNotMatch(componentSource, /exact-photo/);
  assert.match(componentSource, /device-artwork__pre-rendered/);
});

test('reviewed left and right assets are explicit and never mirrored', () => {
  assert.match(componentSource, /componentType === 'right' \? 'right' : 'left'/);
  assert.doesNotMatch(catalogSource, /mirrorHorizontally/);
  assert.doesNotMatch(componentSource, /mirrored/);
});
