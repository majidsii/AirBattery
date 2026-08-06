import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';

const app = readFileSync(new URL('../src/App.vue', import.meta.url), 'utf8');
const settingsView = readFileSync(new URL('../src/views/SettingsView.vue', import.meta.url), 'utf8');
const settingsStore = readFileSync(new URL('../src/stores/settings.ts', import.meta.url), 'utf8');
const batteryView = readFileSync(new URL('../src/views/BatteryView.vue', import.meta.url), 'utf8');
const glassCss = readFileSync(new URL('../src/styles/glass.css', import.meta.url), 'utf8');
const main = readFileSync(new URL('../src/main.ts', import.meta.url), 'utf8');
const glassDomain = readFileSync(new URL('../src/domain/glass.ts', import.meta.url), 'utf8');

test('the app binds its native window role to the glass material system', () => {
  assert.match(app, /:data-window-mode="windowMode"/);
  assert.match(app, /'glass-shell': true/);
  assert.match(settingsStore, /root\.dataset\.glassPreset/);
  assert.match(settingsStore, /glassCssVariables/);
});

test('appearance settings expose presets and independent surface controls', () => {
  assert.match(settingsView, /Glass material/);
  assert.match(glassDomain, /label: 'Off'/);
  assert.match(glassDomain, /label: 'Subtle'/);
  assert.match(glassDomain, /label: 'Balanced'/);
  assert.match(glassDomain, /label: 'Deep'/);
  assert.match(glassDomain, /label: 'Crystal'/);
  assert.match(settingsView, /Main window/);
  assert.match(settingsView, /Desktop widget/);
  assert.match(settingsView, /Connection popup/);
  assert.match(settingsView, /Reduce transparency/);
});

test('the approved overview reserves premium glass for device content', () => {
  assert.match(batteryView, /class="overview-header"/);
  assert.doesNotMatch(batteryView, /overview-hero glass-titlebar/);
  assert.match(batteryView, /v-for="device in displayedDevices"/);
  assert.match(batteryView, /class="connected-device-card glass-card liquid-surface"/);
  assert.match(batteryView, /<DeviceBatteryPanel\s+:device="device"/);
});

test('glass styling includes specular highlights and accessibility fallbacks', () => {
  assert.match(main, /\.\/styles\/glass\.css/);
  assert.match(glassCss, /\.glass-card::before/);
  assert.match(glassCss, /backdrop-filter: blur/);
  assert.match(glassCss, /prefers-reduced-transparency/);
  assert.match(glassCss, /@supports not \(backdrop-filter/);
  assert.match(glassCss, /data-glass-reduced="true"/);
});

const appNav = readFileSync(new URL('../src/components/AppNav.vue', import.meta.url), 'utf8');
const aboutView = readFileSync(new URL('../src/views/AboutView.vue', import.meta.url), 'utf8');
const brandMark = readFileSync(new URL('../src/components/BrandMark.vue', import.meta.url), 'utf8');
const buildScript = readFileSync(new URL('../src-tauri/build.rs', import.meta.url), 'utf8');

test('the approved battery and Bluetooth mark replaces placeholder A branding', () => {
  assert.match(appNav, /import BrandMark/);
  assert.match(appNav, /<BrandMark/);
  assert.doesNotMatch(appNav, />A</);
  assert.match(aboutView, /import BrandMark/);
  assert.match(aboutView, /<BrandMark/);
  assert.match(brandMark, /'AirBattery logo'/);
  assert.match(brandMark, /class="brand-mark__battery"/);
  assert.match(brandMark, /class="brand-mark__bluetooth"/);
});

test('glass content remains vertically scrollable at every window size', () => {
  assert.doesNotMatch(glassCss, /\.glass-shell \.app-content\s*\{[^}]*overflow:\s*clip/s);
  assert.match(glassCss, /\.glass-shell \.app-content\s*\{[^}]*overflow-y:\s*auto/s);
  assert.match(glassCss, /scrollbar-gutter:\s*stable/);
});

test('icon source changes force the native desktop binary to rebuild', () => {
  assert.match(buildScript, /"icons\/icon\.png"/);
  assert.match(buildScript, /"icons\/icon\.svg"/);
  assert.match(buildScript, /cargo:rerun-if-changed=\{icon\}/);
});


const tauriConfig = readFileSync(new URL('../src-tauri/tauri.conf.json', import.meta.url), 'utf8');
const linuxDesktop = readFileSync(
  new URL('../../../packaging/linux/io.github.airbattery.airbattery.desktop', import.meta.url),
  'utf8',
);

test('the refined overview keeps glass on content cards instead of a giant title panel', () => {
  assert.match(batteryView, /class="overview-header"/);
  assert.doesNotMatch(batteryView, /overview-hero glass-titlebar/);
  assert.match(glassCss, /\.overview-header\s*\{/);
  assert.match(glassCss, /\.connected-device-card\s*\{/);
});

test('the scrollbar stays visually hidden until the content surface is engaged', () => {
  assert.match(glassCss, /scrollbar-width:\s*thin/);
  assert.match(glassCss, /scrollbar-color:\s*transparent transparent/);
  assert.match(glassCss, /\.app-content:hover::-webkit-scrollbar-thumb/);
  assert.match(glassCss, /::-webkit-scrollbar-button\s*\{[^}]*display:\s*none/s);
});

test('Linux development windows expose the canonical GTK app id for dock matching', () => {
  const parsed = JSON.parse(tauriConfig) as { app?: { enableGTKAppId?: boolean } };
  assert.equal(parsed.app?.enableGTKAppId, true);
  assert.match(linuxDesktop, /StartupWMClass=io\.github\.airbattery\.airbattery/);
});


const linuxDesktopInstaller = readFileSync(
  new URL('../../../scripts/install-linux-desktop-integration.sh', import.meta.url),
  'utf8',
);

test('Linux dock integration installs the canonical desktop id and icon in user scope', () => {
  assert.match(linuxDesktopInstaller, /APP_ID="io\.github\.airbattery\.airbattery"/);
  assert.match(linuxDesktopInstaller, /DESKTOP_TARGET="\$APPLICATIONS_DIR\/\$APP_ID\.desktop"/);
  assert.match(linuxDesktopInstaller, /ICONS_ROOT\/scalable\/apps\/\$APP_ID\.svg/);
  assert.match(linuxDesktopInstaller, /for alias in AirBattery airbattery/);
  assert.match(linuxDesktopInstaller, /NoDisplay=true/);
  assert.match(linuxDesktopInstaller, /update-desktop-database/);
  assert.match(linuxDesktopInstaller, /gtk-update-icon-cache/);
});
