//! Native entry point for the `AirBattery` desktop application.

#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() {
    airbattery_desktop::run();
}
