//! Build script for the `AirBattery` desktop application.

fn main() {
    // Tauri embeds the application/window icon at compile time. Explicitly
    // tracking icon sources prevents a stale development binary from keeping
    // an older dock/taskbar icon after branding updates.
    for icon in [
        "icons/icon.svg",
        "icons/icon.png",
        "icons/32x32.png",
        "icons/128x128.png",
        "icons/128x128@2x.png",
        "icons/icon.ico",
    ] {
        println!("cargo:rerun-if-changed={icon}");
    }

    tauri_build::build();
}
