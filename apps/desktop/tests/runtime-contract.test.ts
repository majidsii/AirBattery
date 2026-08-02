import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

const refreshSource = readFileSync(
  new URL('../src-tauri/src/refresh.rs', import.meta.url),
  'utf8',
);
const linuxSource = readFileSync(
  new URL('../src-tauri/src/platform/linux.rs', import.meta.url),
  'utf8',
);
const windowsSource = readFileSync(
  new URL('../src-tauri/src/platform/windows.rs', import.meta.url),
  'utf8',
);
const baseCss = readFileSync(new URL('../src/styles/base.css', import.meta.url), 'utf8');

const nativeSurfaceSource = readFileSync(
  new URL('../src-tauri/src/native_surface.rs', import.meta.url),
  'utf8',
);
const settingsViewSource = readFileSync(
  new URL('../src/views/SettingsView.vue', import.meta.url),
  'utf8',
);

function cssRule(selector: string): string {
  const escaped = selector.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  const match = baseCss.match(new RegExp(`${escaped}\\s*\\{([^}]*)\\}`, 's'));
  assert.ok(match, `Missing CSS rule for ${selector}`);
  return match[1];
}

test('automatic refresh uses a three-second cadence with two-second discovery windows', () => {
  assert.match(
    refreshSource,
    /const REFRESH_INTERVAL: Duration = Duration::from_secs\(3\);/,
  );
  assert.match(
    linuxSource,
    /const DISCOVERY_WINDOW: Duration = Duration::from_secs\(2\);/,
  );
  assert.match(
    windowsSource,
    /const DISCOVERY_WINDOW: Duration = Duration::from_secs\(2\);/,
  );
  assert.match(
    linuxSource,
    /automatic advertisement scans are bounded to two seconds\./,
  );
  assert.match(
    refreshSource,
    /MissedTickBehavior::Skip/,
    'slow scans must not queue overlapping refresh work',
  );
});

test('navigation tooltip layer stays above the content panel', () => {
  const nav = cssRule('.app-nav');
  const content = cssRule('.app-content');
  const label = cssRule('.nav-button__label');

  assert.match(nav, /position:\s*relative/);
  assert.match(nav, /isolation:\s*isolate/);
  assert.match(nav, /z-index:\s*20/);
  assert.match(content, /position:\s*relative/);
  assert.match(content, /z-index:\s*1/);
  assert.match(label, /z-index:\s*30/);
  assert.match(label, /isolation:\s*isolate/);
});


test('native status surfaces are selected per operating system and desktop session', () => {
  assert.match(nativeSurfaceSource, /PlatformKind::Windows/);
  assert.match(nativeSurfaceSource, /PlatformKind::Linux/);
  assert.match(nativeSurfaceSource, /DesktopEnvironment::Gnome/);
  assert.match(nativeSurfaceSource, /enable_gnome_integration/);
  assert.match(settingsViewSource, /Windows system tray/);
  assert.match(settingsViewSource, /GNOME Shell extension/);
  assert.match(settingsViewSource, /Native status icon fallback/);
});
