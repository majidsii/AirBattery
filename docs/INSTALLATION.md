# Installation

No public release artifact has been generated or validated yet. The commands below describe the intended installation paths after artifacts exist.

## Debian package

```bash
sudo apt install ./airbattery_<version>_amd64.deb
```

The leading `./` is required for a local package. Inspect the actual generated filename before installation.

## AppImage

```bash
chmod +x AirBattery_<version>_amd64.AppImage
./AirBattery_<version>_amd64.AppImage
```

## GNOME 50 companion

From a source checkout:

```bash
./scripts/install-gnome-extension.sh
```

The extension is installed per-user under `${XDG_DATA_HOME:-$HOME/.local/share}/gnome-shell/extensions/`. Do not use `sudo`.

After installation:

```bash
gnome-extensions enable airbattery@airbattery.github.io
```

A logout/login may be required after installing a new extension. The desktop application must be running with GNOME integration enabled for live data to appear.

## Windows installer

Run the generated NSIS executable as the current user. The authored configuration uses a per-user installation. No installer artifact or upgrade/uninstall result is currently verified.

## Removal

Source-installed GNOME extension:

```bash
gnome-extensions disable airbattery@airbattery.github.io
rm -rf "${XDG_DATA_HOME:-$HOME/.local/share}/gnome-shell/extensions/airbattery@airbattery.github.io"
```

Package removal and clean-uninstall behavior must be validated against real generated packages before release documentation is finalized.
