import assert from 'node:assert/strict';
import { existsSync, readFileSync } from 'node:fs';
import test from 'node:test';

const ambientUrl = new URL('../src/styles/ambient.css', import.meta.url);
const main = readFileSync(new URL('../src/main.ts', import.meta.url), 'utf8');
const app = readFileSync(new URL('../src/App.vue', import.meta.url), 'utf8');
const tokens = readFileSync(new URL('../src/styles/tokens.css', import.meta.url), 'utf8');
const glass = readFileSync(new URL('../src/styles/glass.css', import.meta.url), 'utf8');
const ambient = existsSync(ambientUrl) ? readFileSync(ambientUrl, 'utf8') : '';

test('approved Option 2 uses a dedicated monochrome ambient layer', () => {
  assert.equal(existsSync(ambientUrl), true, 'ambient.css must exist');
  assert.match(main, /\.\/styles\/ambient\.css/);
  assert.match(app, /ambient-background__art--top/);
  assert.match(app, /ambient-background__art--bottom/);
  assert.match(tokens, /--canvas:\s*#f4f4f5/i);
  assert.match(tokens, /--canvas:\s*#090909/i);
  assert.match(tokens, /--wave-fill:\s*255,\s*255,\s*255/);
  assert.match(tokens, /--wave-fill:\s*0,\s*0,\s*0/);
  const explicitDark = tokens.match(/:root\[data-theme="dark"\]\s*\{([\s\S]*?)\n\}/)?.[1] ?? '';
  assert.match(explicitDark, /--canvas:\s*#090909/);
  assert.match(explicitDark, /--wave-fill-opacity:\s*0\.82/);
  assert.match(explicitDark, /--wave-line-primary-opacity:\s*0\.52/);
  assert.match(
    tokens,
    /@media \(prefers-color-scheme:\s*dark\)[\s\S]*:root:not\(\[data-theme="light"\]\)[\s\S]*--canvas:\s*#090909[\s\S]*--wave-fill:\s*0,\s*0,\s*0/,
  );
  assert.match(ambient, /rgba\(var\(--wave-fill\),/);
});

test('ambient artwork contains only neutral black white and graphite depth', () => {
  assert.doesNotMatch(ambient, /(?:blue|green|purple|teal|violet|orange)/i);
  assert.doesNotMatch(ambient, /rgba\(\s*(?:56|65|69|72|80|83|86|92|93|94|95|96|99|119|181|183|211|218|223|229)\s*,/i);
  assert.doesNotMatch(glass, /ambient-background/);
});

test('ambient waves disappear for reduced transparency and high contrast', () => {
  assert.match(ambient, /prefers-reduced-transparency:\s*reduce/);
  assert.match(ambient, /prefers-contrast:\s*more/);
  assert.match(ambient, /data-glass-reduced="true"/);
});


test('ambient artwork stays responsive in normal, maximized and ultrawide viewports', () => {
  assert.match(
    ambient,
    /\.ambient-background__art\s*\{[^}]*inset:\s*0;[^}]*width:\s*100%;[^}]*height:\s*100%;/s,
  );
  assert.doesNotMatch(ambient, /@media\s*\(max-width:\s*780px\)/);
  assert.doesNotMatch(ambient, /\b(?:150|178)vw\b/);
});


test('approved background uses layered SVG ribbons instead of blurred oval blobs', () => {
  assert.equal(
    (app.match(/preserveAspectRatio="none"/g) ?? []).length,
    2,
  );
  assert.ok(
    (app.match(/ambient-background__line/g) ?? []).length >= 8,
  );
  assert.doesNotMatch(ambient, /radial-gradient/);
});
