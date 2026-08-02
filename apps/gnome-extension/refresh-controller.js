export const REFRESH_INTERVAL_MS = 3000;

/**
 * Serializes lightweight cached-snapshot reads and owns the fallback timer.
 * The desktop process already performs bounded Bluetooth refreshes, so the
 * extension only reads GetSnapshot and never starts a competing scan.
 * Scheduling is injected so the lifecycle can be tested outside GNOME Shell.
 */
export class RefreshController {
  constructor({
    isAvailable,
    load,
    schedule,
    cancel,
    intervalMs = REFRESH_INTERVAL_MS,
  }) {
    this._isAvailable = isAvailable;
    this._load = load;
    this._schedule = schedule;
    this._cancel = cancel;
    this._intervalMs = intervalMs;
    this._running = false;
    this._inFlight = false;
    this._sourceId = null;
    this._lifecycleToken = 0;
  }

  start() {
    if (this._running) return;

    this._running = true;
    this._lifecycleToken += 1;
    this._sourceId = this._schedule(this._intervalMs, () => {
      void this.requestSnapshot();
    });
    void this.requestSnapshot();
  }

  stop() {
    if (!this._running && this._sourceId === null) return;

    this._running = false;
    this._lifecycleToken += 1;
    this._inFlight = false;
    if (this._sourceId !== null) {
      this._cancel(this._sourceId);
      this._sourceId = null;
    }
  }

  menuOpened(isOpen) {
    if (isOpen) void this.requestSnapshot();
  }

  serviceAvailable(hasOwner) {
    if (hasOwner) void this.requestSnapshot();
  }

  requestSnapshot() {
    return this._run(this._load);
  }

  async _run(operation) {
    if (!this._running || this._inFlight || !this._isAvailable()) return false;

    const lifecycleToken = this._lifecycleToken;
    this._inFlight = true;
    try {
      await operation();
      return true;
    } finally {
      if (lifecycleToken === this._lifecycleToken) this._inFlight = false;
    }
  }
}
