import assert from 'node:assert/strict';
import test from 'node:test';

import {
  REFRESH_INTERVAL_MS,
  RefreshController,
} from '../refresh-controller.js';

function deferred() {
  let resolve;
  const promise = new Promise(done => {
    resolve = done;
  });
  return {promise, resolve};
}

function flush() {
  return new Promise(resolve => setImmediate(resolve));
}

function harness({available = true, loadImpl} = {}) {
  const calls = [];
  const scheduled = [];
  const cancelled = [];
  let isAvailable = available;

  const controller = new RefreshController({
    isAvailable: () => isAvailable,
    load: loadImpl ?? (async () => {
      calls.push('load');
    }),
    schedule: (intervalMs, callback) => {
      scheduled.push({intervalMs, callback});
      return 41;
    },
    cancel: sourceId => {
      cancelled.push(sourceId);
    },
  });

  return {
    calls,
    scheduled,
    cancelled,
    controller,
    setAvailable(value) {
      isAvailable = value;
    },
  };
}

test('loads the cached snapshot immediately and polls it every three seconds', async () => {
  const state = harness();

  state.controller.start();
  await flush();

  assert.deepEqual(state.calls, ['load']);
  assert.equal(state.scheduled.length, 1);
  assert.equal(state.scheduled[0].intervalMs, REFRESH_INTERVAL_MS);
  assert.equal(REFRESH_INTERVAL_MS, 3000);

  state.scheduled[0].callback();
  await flush();

  assert.deepEqual(state.calls, ['load', 'load']);
});

test('menu opening and service recovery request the cached snapshot immediately', async () => {
  const state = harness({available: false});

  state.controller.start();
  await flush();
  assert.deepEqual(state.calls, []);

  state.controller.menuOpened(false);
  await flush();
  assert.deepEqual(state.calls, []);

  state.setAvailable(true);
  state.controller.serviceAvailable(true);
  await flush();
  assert.deepEqual(state.calls, ['load']);

  state.controller.menuOpened(true);
  await flush();
  assert.deepEqual(state.calls, ['load', 'load']);
});

test('suppresses overlapping snapshot requests without blocking later polls', async () => {
  const pending = deferred();
  const calls = [];
  const state = harness({
    loadImpl: async () => {
      calls.push('load');
      await pending.promise;
    },
  });

  state.controller.start();
  await flush();
  assert.deepEqual(calls, ['load']);

  state.scheduled[0].callback();
  state.controller.menuOpened(true);
  await flush();
  assert.deepEqual(calls, ['load']);

  pending.resolve();
  await flush();

  state.scheduled[0].callback();
  await flush();
  assert.deepEqual(calls, ['load', 'load']);
});

test('stop cancels the timer and blocks callbacks until restarted', async () => {
  const state = harness();

  state.controller.start();
  await flush();
  state.controller.stop();

  assert.deepEqual(state.cancelled, [41]);

  state.scheduled[0].callback();
  state.controller.menuOpened(true);
  state.controller.serviceAvailable(true);
  await flush();

  assert.deepEqual(state.calls, ['load']);
});
