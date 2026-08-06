import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import { test } from 'node:test';

const ambientCss = readFileSync(
  new URL('../src/styles/ambient.css', import.meta.url),
  'utf8',
);

const appVue = readFileSync(
  new URL('../src/App.vue', import.meta.url),
  'utf8',
);

test('ambient artwork uses one responsive viewport geometry', () => {
  assert.match(
    ambientCss,
    /\.ambient-background__art\s*\{[^}]*inset:\s*0;[^}]*width:\s*100%;[^}]*height:\s*100%;/s,
  );

  assert.doesNotMatch(ambientCss, /@media\s*\(max-width:\s*780px\)/);
  assert.doesNotMatch(ambientCss, /\b(?:150|178)vw\b/);

  assert.equal(
    (appVue.match(/preserveAspectRatio="none"/g) ?? []).length,
    2,
  );
});

test('lower-left ribbons fade smoothly instead of ending abruptly', () => {
  assert.match(
    ambientCss,
    /\.ambient-background__art--bottom\s*\{[^}]*mask-image:\s*linear-gradient\(\s*to right,[^}]*transparent 88%\s*\)/s,
  );
});

test('upper-right ribbons use the mirrored responsive fade', () => {
  assert.match(
    ambientCss,
    /\.ambient-background__art--top\s*\{[^}]*mask-image:\s*linear-gradient\(\s*to left,[^}]*transparent 88%\s*\)/s,
  );
});
