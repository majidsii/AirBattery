import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

const component = readFileSync(new URL('../src/components/DeviceArtwork.vue', import.meta.url), 'utf8');
const catalog = readFileSync(new URL('../src/domain/artwork-catalog.ts', import.meta.url), 'utf8');
const registry = readFileSync(new URL('../src/domain/pre-rendered-artwork.ts', import.meta.url), 'utf8');

test('AirPods use a generic inline fallback until exact reviewed 3D exists', () => {
  assert.match(component, /device-artwork__generic-fallback/);
  assert.match(component, /viewBox="0 0 180 130"/);
  assert.doesNotMatch(component, /airpods-pro-single/);
  assert.doesNotMatch(catalog, /airpods-pro-single/);
  assert.match(registry, /preRenderedArtworkAssets/);
  assert.doesNotMatch(component, /device-artwork__exact-photo/);
});
