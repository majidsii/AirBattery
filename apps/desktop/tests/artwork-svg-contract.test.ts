import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

const component = readFileSync(new URL('../src/components/DeviceArtwork.vue', import.meta.url), 'utf8');
const catalog = readFileSync(new URL('../src/domain/artwork-catalog.ts', import.meta.url), 'utf8');
const generated = readFileSync(new URL('../src/domain/exact-artwork.generated.ts', import.meta.url), 'utf8');

const fallbackAssets = [
  '../src/assets/device-artwork/catalog/airpods-pro-single.svg',
  '../src/assets/device-artwork/catalog/airpods-pro-pair.svg',
  '../src/assets/device-artwork/catalog/airpods-pro-case.svg',
];

test('AirPods Pro keeps original vector fallback when no licensed exact photo exists', () => {
  for (const relativePath of fallbackAssets) {
    const text = readFileSync(new URL(relativePath, import.meta.url), 'utf8');
    assert.match(text, /<svg/);
    assert.ok(text.length > 300);
  }
  assert.match(catalog, /airpods-pro-single\.svg/);
  assert.match(catalog, /airpods-pro-pair\.svg/);
  assert.match(catalog, /airpods-pro-case\.svg/);
  assert.match(generated, /exactArtworkAssets/);
  assert.doesNotMatch(component, /device-artwork__studio-asset/);
});
