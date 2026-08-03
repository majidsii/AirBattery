import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

const css = readFileSync(new URL('../src/styles/base.css', import.meta.url), 'utf8');

test('battery values remain visually stronger than fallback product artwork', () => {
  assert.match(css, /device-battery-slot__value-row strong \{ font-size: 38px/);
  assert.match(css, /device-artwork \{ width: min\(148px, 100%\); height: 104px/);
  assert.match(css, /device-artwork__pre-rendered/);
  assert.doesNotMatch(css, /device-artwork__exact-photo/);
});
