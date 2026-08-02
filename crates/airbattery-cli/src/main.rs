//! `AirBattery` hardware and parser diagnostics.

use std::{process::ExitCode, time::Duration};

use bluetooth_linux::{BluezDeviceSnapshot, LinuxBluetoothBackend};
use clap::{Parser, Subcommand};
use diagnostics::sanitize_identifier;
use protocol_airpods::parse_proximity_pairing;
use serde_json::{Value, json};
use tracing_subscriber::EnvFilter;

#[derive(Debug, Parser)]
#[command(name = "airbattery", version, about = "AirBattery diagnostic CLI")]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Show `BlueZ` adapter status.
    Status,
    /// Run a bounded Bluetooth scan.
    Scan {
        /// Maximum scan duration.
        #[arg(long, default_value_t = 20, value_parser = clap::value_parser!(u64).range(1..=120))]
        seconds: u64,
        /// Emit machine-readable JSON.
        #[arg(long)]
        json: bool,
    },
    /// Parse Apple manufacturer payload bytes supplied as hexadecimal.
    ParseAirpods {
        /// Hexadecimal bytes after company id 0x004C.
        #[arg(long)]
        hex: String,
    },
}

fn sanitized_snapshot(snapshot: &BluezDeviceSnapshot) -> Value {
    let airpods = snapshot
        .manufacturer_data
        .get(&protocol_airpods::APPLE_MANUFACTURER_ID)
        .and_then(|payload| parse_proximity_pairing(payload).ok().flatten());
    let manufacturer_payload_lengths = snapshot
        .manufacturer_data
        .iter()
        .map(|(company_id, payload)| (company_id.to_string(), payload.len()))
        .collect::<std::collections::BTreeMap<_, _>>();

    json!({
        "id": sanitize_identifier(&snapshot.address),
        "name": snapshot.alias.as_deref(),
        "connected": snapshot.connected,
        "paired": snapshot.paired,
        "vendorId": snapshot.vendor_id,
        "batteryPercentage": snapshot.battery_percentage,
        "icon": snapshot.icon.as_deref(),
        "class": snapshot.class,
        "appearance": snapshot.appearance,
        "serviceUuids": snapshot.service_uuids,
        "rssi": snapshot.rssi,
        "manufacturerCompanyIds": snapshot.manufacturer_data.keys().collect::<Vec<_>>(),
        "manufacturerPayloadLengths": manufacturer_payload_lengths,
        "airPods": airpods,
        "observedAt": snapshot.observed_at,
    })
}

async fn run(command: Command) -> Result<(), String> {
    match command {
        Command::ParseAirpods { hex } => {
            let bytes = hex::decode(hex.trim()).map_err(|error| format!("invalid hex: {error}"))?;
            let parsed = parse_proximity_pairing(&bytes).map_err(|error| error.to_string())?;
            println!(
                "{}",
                serde_json::to_string_pretty(&parsed)
                    .map_err(|error| format!("failed to serialize parser result: {error}"))?
            );
        }
        Command::Status => {
            let backend = LinuxBluetoothBackend::new()
                .await
                .map_err(|error| error.to_string())?;
            let status = backend.status().await.map_err(|error| error.to_string())?;
            println!(
                "{}",
                serde_json::to_string_pretty(&status)
                    .map_err(|error| format!("failed to serialize status: {error}"))?
            );
        }
        Command::Scan { seconds, json } => {
            let backend = LinuxBluetoothBackend::new()
                .await
                .map_err(|error| error.to_string())?;
            let snapshots = backend
                .scan_window(Duration::from_secs(seconds))
                .await
                .map_err(|error| error.to_string())?;
            if json {
                let values: Vec<_> = snapshots.iter().map(sanitized_snapshot).collect();
                println!(
                    "{}",
                    serde_json::to_string_pretty(&values)
                        .map_err(|error| format!("failed to serialize scan: {error}"))?
                );
            } else if snapshots.is_empty() {
                println!("No Bluetooth advertisements observed in the scan window.");
            } else {
                for snapshot in &snapshots {
                    println!(
                        "{}  connected={} battery={}",
                        sanitize_identifier(&snapshot.address),
                        snapshot.connected,
                        snapshot
                            .battery_percentage
                            .map_or_else(|| "unavailable".to_owned(), |value| format!("{value}%"))
                    );
                }
            }
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> ExitCode {
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .with_target(false)
        .compact()
        .init();

    match run(Cli::parse().command).await {
        Ok(()) => ExitCode::SUCCESS,
        Err(error) => {
            eprintln!("error: {error}");
            ExitCode::FAILURE
        }
    }
}
