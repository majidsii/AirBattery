import Adw from 'gi://Adw';
import Gio from 'gi://Gio';
import Gtk from 'gi://Gtk?version=4.0';

import {ExtensionPreferences} from 'resource:///org/gnome/Shell/Extensions/js/extensions/prefs.js';

export default class AirBatteryPreferences extends ExtensionPreferences {
    fillPreferencesWindow(window) {
        const settings = this.getSettings();
        window._airBatterySettings = settings;

        const page = new Adw.PreferencesPage({
            title: 'AirBattery',
            icon_name: 'audio-headphones-symbolic',
        });
        const appearance = new Adw.PreferencesGroup({
            title: 'Top bar',
            description: 'Choose what AirBattery shows beside its status icon.',
        });
        const percentage = new Adw.SwitchRow({
            title: 'Show percentage',
            subtitle: 'Uses the aggregate value, or the lower fresh earbud value.',
        });
        settings.bind('show-percentage', percentage, 'active', Gio.SettingsBindFlags.DEFAULT);
        appearance.add(percentage);

        const preferred = new Adw.ActionRow({
            title: 'Preferred display device',
            subtitle: 'Choose a device from the AirBattery top-bar menu.',
        });
        const reset = new Gtk.Button({
            label: 'Use automatic selection',
            valign: Gtk.Align.CENTER,
        });
        reset.connect('clicked', () => settings.set_string('preferred-device-id', ''));
        preferred.add_suffix(reset);
        appearance.add(preferred);

        page.add(appearance);
        window.add(page);
    }
}
