import Clutter from 'gi://Clutter';
import Gio from 'gi://Gio';
import GLib from 'gi://GLib';
import St from 'gi://St';

import {Extension} from 'resource:///org/gnome/shell/extensions/extension.js';
import * as Main from 'resource:///org/gnome/shell/ui/main.js';
import * as PanelMenu from 'resource:///org/gnome/shell/ui/panelMenu.js';
import * as PopupMenu from 'resource:///org/gnome/shell/ui/popupMenu.js';

import {
    effectiveConnectionState,
    componentText,
    normalizeSnapshot,
    panelStatusTone,
    panelSummary,
    selectPanelDevice,
} from './snapshot.js';
import {REFRESH_INTERVAL_MS, RefreshController} from './refresh-controller.js';
import {BUS_NAME, OBJECT_PATH, SERVICE_XML} from './service.js';

const AirBatteryProxy = Gio.DBusProxy.makeProxyWrapper(SERVICE_XML);
const PANEL_STATUS_CLASSES = Object.freeze([
    'airbattery-status-connected',
    'airbattery-status-low',
    'airbattery-status-critical',
    'airbattery-status-unavailable',
    'airbattery-status-disconnected',
]);

const EMPTY_SNAPSHOT = Object.freeze({
    schemaVersion: 1,
    generatedAt: null,
    backend: null,
    devices: [],
    preferredDeviceId: null,
});

function remoteCall(proxy, methodName) {
    return new Promise((resolve, reject) => {
        const method = proxy?.[`${methodName}Remote`];
        if (typeof method !== 'function') {
            reject(new Error(`D-Bus method ${methodName} is unavailable`));
            return;
        }
        method.call(proxy, (result, error) => {
            if (error) {
                reject(error);
                return;
            }
            resolve(Array.isArray(result) ? result[0] : result);
        });
    });
}

function connectionLabel(state) {
    switch (state) {
    case 'connected':
        return 'Connected';
    case 'connecting':
        return 'Connecting';
    case 'disconnected':
        return 'Disconnected';
    default:
        return 'Connection unavailable';
    }
}

function componentTitle(type) {
    return {
        left: 'Left earbud',
        right: 'Right earbud',
        case: 'Charging case',
        headset: 'Headset',
        aggregate: 'Combined battery',
        unknown: 'Battery',
    }[type] ?? 'Battery';
}

export default class AirBatteryExtension extends Extension {
    enable() {
        this._settings = this.getSettings();
        this._snapshot = {...EMPTY_SNAPSHOT, devices: []};
        this._proxy = null;
        this._snapshotSignalId = 0;
        this._ownerSignalId = 0;
        this._menuSignalId = 0;
        this._settingsSignalIds = [];
        this._refreshController = null;

        this._indicator = new PanelMenu.Button(0.0, this.metadata.name, false);
        this._indicatorBox = new St.BoxLayout({style_class: 'airbattery-panel-box'});
        this._icon = new St.Icon({
            gicon: Gio.icon_new_for_string(`${this.path}/icons/airbattery-symbolic.svg`),
            style_class: 'system-status-icon airbattery-panel-icon',
        });
        this._percentageLabel = new St.Label({
            text: '',
            y_align: Clutter.ActorAlign.CENTER,
            style_class: 'airbattery-panel-percentage',
        });
        this._indicatorBox.add_child(this._icon);
        this._indicatorBox.add_child(this._percentageLabel);
        this._indicator.add_child(this._indicatorBox);
        Main.panel.addToStatusArea(this.uuid, this._indicator);

        this._settingsSignalIds.push(
            this._settings.connect('changed::show-percentage', () => this._render()),
            this._settings.connect('changed::preferred-device-id', () => this._render()),
        );
        this._refreshController = new RefreshController({
            isAvailable: () => Boolean(this._proxy?.g_name_owner),
            load: () => this._loadSnapshot(),
            schedule: (intervalMs, callback) => GLib.timeout_add(
                GLib.PRIORITY_DEFAULT,
                intervalMs,
                () => {
                    callback();
                    return GLib.SOURCE_CONTINUE;
                },
            ),
            cancel: sourceId => GLib.Source.remove(sourceId),
            intervalMs: REFRESH_INTERVAL_MS,
        });
        this._menuSignalId = this._indicator.menu.connect(
            'open-state-changed',
            (_menu, isOpen) => this._refreshController?.menuOpened(isOpen),
        );
        this._refreshController.start();
        this._connectService();
        this._render();
    }

    disable() {
        this._refreshController?.stop();
        if (this._indicator?.menu && this._menuSignalId)
            this._indicator.menu.disconnect(this._menuSignalId);
        if (this._proxy && this._snapshotSignalId)
            this._proxy.disconnectSignal(this._snapshotSignalId);
        if (this._proxy && this._ownerSignalId)
            this._proxy.disconnect(this._ownerSignalId);
        for (const signalId of this._settingsSignalIds ?? [])
            this._settings?.disconnect(signalId);

        this._indicator?.destroy();
        this._indicator = null;
        this._indicatorBox = null;
        this._icon = null;
        this._percentageLabel = null;
        this._proxy = null;
        this._snapshotSignalId = 0;
        this._ownerSignalId = 0;
        this._menuSignalId = 0;
        this._settingsSignalIds = [];
        this._refreshController = null;
        this._snapshot = null;
        this._settings = null;
    }

    _connectService() {
        this._proxy = new AirBatteryProxy(
            Gio.DBus.session,
            BUS_NAME,
            OBJECT_PATH,
            (proxy, error) => {
                if (!this._indicator || this._proxy !== proxy)
                    return;
                if (error) {
                    console.warn(`AirBattery D-Bus connection failed: ${error.message}`);
                    this._setUnavailable();
                    return;
                }
                this._snapshotSignalId = proxy.connectSignal(
                    'SnapshotChanged',
                    (_source, _sender, [snapshotJson]) => this._applySnapshot(snapshotJson),
                );
                this._ownerSignalId = proxy.connect('notify::g-name-owner', () => {
                    if (proxy.g_name_owner)
                        this._refreshController?.serviceAvailable(true);
                    else
                        this._setUnavailable();
                });
                if (proxy.g_name_owner)
                    this._refreshController?.serviceAvailable(true);
                else
                    this._setUnavailable();
            },
        );
    }

    async _loadSnapshot() {
        const proxy = this._proxy;
        if (!proxy?.g_name_owner)
            return;
        try {
            const snapshotJson = await remoteCall(proxy, 'GetSnapshot');
            if (this._proxy === proxy && this._indicator)
                this._applySnapshot(snapshotJson);
        } catch (error) {
            console.warn(`AirBattery snapshot request failed: ${error.message}`);
            if (this._proxy === proxy && this._indicator)
                this._setUnavailable();
        }
    }

    async _invokeAction(methodName) {
        try {
            await remoteCall(this._proxy, methodName);
        } catch (error) {
            console.warn(`AirBattery ${methodName} failed: ${error.message}`);
        }
    }

    _applySnapshot(snapshotJson) {
        this._snapshot = normalizeSnapshot(snapshotJson);
        this._render();
    }

    _setUnavailable() {
        this._snapshot = {...EMPTY_SNAPSHOT, devices: []};
        this._render();
    }


    _setPanelStatusTone(tone) {
        if (!this._icon)
            return;
        for (const className of PANEL_STATUS_CLASSES)
            this._icon.remove_style_class_name(className);
        this._icon.add_style_class_name(`airbattery-status-${tone}`);
    }

    _selectedDevice() {
        const localPreferred = this._settings?.get_string('preferred-device-id') ?? '';
        const preferred = localPreferred || this._snapshot?.preferredDeviceId || '';
        return selectPanelDevice(this._snapshot?.devices ?? [], preferred);
    }

    _render() {
        if (!this._indicator)
            return;

        const device = this._selectedDevice();
        const shouldShow = Boolean(device);
        this._indicator.visible = shouldShow;
        if (!shouldShow) {
            this._percentageLabel.text = '';
            this._indicator.accessible_name = 'AirBattery, no connected Bluetooth device';
            this._indicator.menu.removeAll();
            return;
        }

        const tone = panelStatusTone(this._snapshot?.devices ?? [], device);
        this._setPanelStatusTone(tone);

        const showPercentage = this._settings?.get_boolean('show-percentage') ?? true;
        const summary = panelSummary(device);
        this._percentageLabel.text = showPercentage ? summary : '';
        this._indicator.accessible_name = device
            ? `AirBattery, ${device.displayName}, ${summary || 'battery unavailable'}`
            : 'AirBattery, device unavailable';

        this._indicator.menu.removeAll();
        if (!device) {
            const title = new PopupMenu.PopupMenuItem('AirBattery', {reactive: false});
            title.label.add_style_class_name('airbattery-menu-title');
            this._indicator.menu.addMenuItem(title);
            const detail = this._snapshot?.backend?.detail ?? 'AirBattery service is unavailable.';
            this._indicator.menu.addMenuItem(
                new PopupMenu.PopupMenuItem(detail, {reactive: false}),
            );
        } else {
            const title = new PopupMenu.PopupMenuItem(device.displayName, {reactive: false});
            title.label.add_style_class_name('airbattery-menu-title');
            this._indicator.menu.addMenuItem(title);
            this._indicator.menu.addMenuItem(
                new PopupMenu.PopupMenuItem(connectionLabel(effectiveConnectionState(device)), {reactive: false}),
            );
            this._indicator.menu.addMenuItem(new PopupMenu.PopupSeparatorMenuItem());

            const components = Array.isArray(device.components) ? device.components : [];
            if (components.length === 0) {
                this._indicator.menu.addMenuItem(
                    new PopupMenu.PopupMenuItem('Battery data unavailable', {reactive: false}),
                );
            } else {
                const order = ['left', 'right', 'case', 'headset', 'aggregate', 'unknown'];
                for (const type of order) {
                    for (const component of components.filter(item => item.componentType === type)) {
                        const row = new PopupMenu.PopupMenuItem(
                            `${componentTitle(type)}  ${componentText(component)}`,
                            {reactive: false},
                        );
                        if (component.stale === true)
                            row.label.add_style_class_name('airbattery-menu-stale');
                        this._indicator.menu.addMenuItem(row);
                    }
                }
            }
        }

        const devices = (this._snapshot?.devices ?? []).filter(candidate => {
            const state = effectiveConnectionState(candidate);
            return state === 'connected';
        });
        if (devices.length > 1) {
            this._indicator.menu.addMenuItem(new PopupMenu.PopupSeparatorMenuItem());
            const deviceMenu = new PopupMenu.PopupSubMenuMenuItem('Devices');
            const selectedId = device?.id ?? '';
            for (const candidate of devices) {
                const item = new PopupMenu.PopupMenuItem(candidate.displayName);
                item.setOrnament(
                    candidate.id === selectedId
                        ? PopupMenu.Ornament.DOT
                        : PopupMenu.Ornament.NONE,
                );
                item.connect('activate', () => {
                    this._settings.set_string('preferred-device-id', candidate.id);
                });
                deviceMenu.menu.addMenuItem(item);
            }
            this._indicator.menu.addMenuItem(deviceMenu);
        }

        this._indicator.menu.addMenuItem(new PopupMenu.PopupSeparatorMenuItem());
        const settings = new PopupMenu.PopupMenuItem('AirBattery settings');
        settings.connect('activate', () => this._invokeAction('OpenSettings'));
        this._indicator.menu.addMenuItem(settings);

        const open = new PopupMenu.PopupMenuItem('Open AirBattery desktop app');
        open.connect('activate', () => this._invokeAction('ShowMainWindow'));
        this._indicator.menu.addMenuItem(open);
    }
}
