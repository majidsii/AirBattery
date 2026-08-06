import assert from 'node:assert/strict';
import test from 'node:test';

interface GlassSelectOption {
  value: string;
  label: string;
  description?: string;
  disabled?: boolean;
}

async function loadSubject() {
  return import('../src/domain/glass-controls.ts');
}

const options: GlassSelectOption[] = [
  { value: 'system', label: 'System' },
  { value: 'light', label: 'Light', disabled: true },
  { value: 'dark', label: 'Dark' },
];

test('first enabled option skips disabled entries', async () => {
  const { firstEnabledOptionIndex } = await loadSubject();

  assert.equal(firstEnabledOptionIndex(options), 0);
  assert.equal(
    firstEnabledOptionIndex([
      { value: 'a', label: 'A', disabled: true },
      { value: 'b', label: 'B' },
    ]),
    1,
  );
});

test('selected option falls back to the first enabled option', async () => {
  const { selectedEnabledOptionIndex } = await loadSubject();

  assert.equal(selectedEnabledOptionIndex(options, 'dark'), 2);
  assert.equal(selectedEnabledOptionIndex(options, 'light'), 0);
  assert.equal(selectedEnabledOptionIndex(options, 'missing'), 0);
});

test('arrow navigation wraps and skips disabled options', async () => {
  const { moveEnabledOptionIndex } = await loadSubject();

  assert.equal(moveEnabledOptionIndex(options, 0, 1), 2);
  assert.equal(moveEnabledOptionIndex(options, 2, 1), 0);
  assert.equal(moveEnabledOptionIndex(options, 0, -1), 2);
});

test('home and end resolve to enabled edges', async () => {
  const { edgeEnabledOptionIndex } = await loadSubject();

  assert.equal(edgeEnabledOptionIndex(options, 'first'), 0);
  assert.equal(edgeEnabledOptionIndex(options, 'last'), 2);
});

test('navigation returns minus one when no option is enabled', async () => {
  const {
    edgeEnabledOptionIndex,
    firstEnabledOptionIndex,
    moveEnabledOptionIndex,
    selectedEnabledOptionIndex,
  } = await loadSubject();

  const disabled: GlassSelectOption[] = [
    { value: 'a', label: 'A', disabled: true },
  ];

  assert.equal(firstEnabledOptionIndex(disabled), -1);
  assert.equal(selectedEnabledOptionIndex(disabled, 'a'), -1);
  assert.equal(moveEnabledOptionIndex(disabled, 0, 1), -1);
  assert.equal(edgeEnabledOptionIndex(disabled, 'last'), -1);
});
