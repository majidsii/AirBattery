import assert from 'node:assert/strict';
import test from 'node:test';

import { resolveWindowMode } from '../src/domain/window-mode.ts';

test('widget query selects the compact widget surface', () => {
  assert.equal(resolveWindowMode('?window=widget'), 'widget');
  assert.equal(resolveWindowMode('?foo=bar&window=widget'), 'widget');
});

test('unknown or missing window query stays on the main application', () => {
  assert.equal(resolveWindowMode(''), 'main');
  assert.equal(resolveWindowMode('?window=other'), 'main');
});
