import assert from 'node:assert/strict';
import { existsSync, readFileSync, readdirSync } from 'node:fs';
import test from 'node:test';

const catalogSource = readFileSync(new URL('../src/domain/artwork-catalog.ts', import.meta.url), 'utf8');
const keySource = readFileSync(new URL('../src/domain/artwork-keys.ts', import.meta.url), 'utf8');
const exactSource = readFileSync(new URL('../src/domain/exact-artwork.generated.ts', import.meta.url), 'utf8');
const componentSource = readFileSync(new URL('../src/components/DeviceArtwork.vue', import.meta.url), 'utf8');

const requiredKeys = [
  'airpods-pro-1',
  'airpods-4',
  'qcy-t13',
  'qcy-melobuds',
  'qcy-crossky',
  'qcy-melobuds-n70',
  'qcy-ailybuds-pro-plus',
  'qcy-h3-pro',
  'xiaomi-buds-5',
  'redmi-buds-6',
  'soundcore-liberty',
  'galaxy-buds3',
  'sony-linkbuds-open',
  'sony-wf-1000xm4',
  'soundcore-sport-x20',
  'jbl-free',
  'jbl-tour-pro',
  'pixel-buds-pro',
  'nothing-ear',
  'oneplus-buds-pro',
  'huawei-freeclip',
  'beats-fit-pro',
];

test('model-aware artwork catalog covers major official earbud families', () => {
  for (const key of requiredKeys) assert.match(keySource, new RegExp(`'${key}'`));
  assert.match(catalogSource, /artworkAliases/);
  assert.match(componentSource, /resolveArtworkAssetSet/);
  assert.match(componentSource, /catalogArtwork/);
});

test('catalog keeps original SVG families as the truthful always-available fallback', () => {
  const directory = new URL('../src/assets/device-artwork/catalog/', import.meta.url);
  const files = readdirSync(directory).filter((name) => name.endsWith('.svg'));
  assert.equal(files.length, 87);
  assert.match(catalogSource, /catalog\/.+\.svg/);
  assert.doesNotMatch(catalogSource, /studio\/.+\.webp/);
  assert.equal(existsSync(new URL('../src/assets/device-artwork/studio/', import.meta.url)), false);
});

test('exact licensed photographs are optional per model and per component mode', () => {
  assert.match(catalogSource, /exactArtworkAssets/);
  assert.match(catalogSource, /mergeAssetSets/);
  assert.match(exactSource, /ExactArtworkAssetMap/);
  assert.match(exactSource, /exactArtworkAssets/);
  assert.match(componentSource, /device-artwork__exact-photo/);
  assert.match(componentSource, /artworkSourceKind/);
});
