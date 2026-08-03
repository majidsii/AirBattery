//! Event-driven Apple accessory battery monitoring over classic `Bluetooth` `L2CAP`.

use std::{
    collections::{BTreeMap, BTreeSet},
    sync::{Arc, OnceLock},
    time::Duration,
};

use bluer::{
    Address, AddressType,
    l2cap::{SeqPacket, SocketAddr},
};
use time::OffsetDateTime;
use tokio::sync::{Mutex, RwLock, watch};

const APPLE_ACCESSORY_PSM: u16 = 0x1001;
const CONNECT_TIMEOUT: Duration = Duration::from_secs(4);
const RECEIVE_TIMEOUT: Duration = Duration::from_secs(20);
const RECONNECT_DELAY: Duration = Duration::from_secs(4);
const HANDSHAKE_PACKET: [u8; 16] = [
    0x00, 0x00, 0x04, 0x00, 0x01, 0x00, 0x02, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
];
const REQUEST_NOTIFICATIONS_PACKET: [u8; 10] =
    [0x04, 0x00, 0x04, 0x00, 0x0f, 0x00, 0xff, 0xff, 0xff, 0xff];
const BATTERY_HEADER: [u8; 6] = [0x04, 0x00, 0x04, 0x00, 0x04, 0x00];

/// Latest exact packet received from an active Apple accessory channel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct AppleAccessoryPacket {
    /// Complete accessory packet, including header and opcode.
    pub payload: Vec<u8>,
    /// Time the packet was received from the socket.
    pub observed_at: OffsetDateTime,
}

#[derive(Debug)]
struct MonitorEntry {
    latest: RwLock<Option<AppleAccessoryPacket>>,
    shutdown: watch::Sender<bool>,
}

impl MonitorEntry {
    fn new() -> (Arc<Self>, watch::Receiver<bool>) {
        let (shutdown, receiver) = watch::channel(false);
        (
            Arc::new(Self {
                latest: RwLock::new(None),
                shutdown,
            }),
            receiver,
        )
    }
}

static MONITORS: OnceLock<Mutex<BTreeMap<String, Arc<MonitorEntry>>>> = OnceLock::new();

fn monitors() -> &'static Mutex<BTreeMap<String, Arc<MonitorEntry>>> {
    MONITORS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

fn normalized_address(address: &str) -> String {
    address.trim().to_ascii_uppercase()
}

/// Synchronizes long-lived Apple accessory monitors with the authoritative set
/// of currently connected Apple audio candidates.
///
/// Existing monitors are retained while their device remains connected, even
/// when a later scan temporarily lacks Apple advertisement identity. New
/// monitors are created only for confirmed candidates. Disconnected addresses
/// receive a cancellation signal so their sockets and retry loops are released.
/// Address keys are normalized to avoid duplicate monitors caused by casing
/// differences between platform snapshots.
pub async fn sync_apple_accessory_monitors(
    connected_addresses: &BTreeSet<String>,
    candidate_addresses: &BTreeSet<String>,
) {
    let connected = connected_addresses
        .iter()
        .map(|address| normalized_address(address))
        .collect::<BTreeSet<_>>();
    let candidates = candidate_addresses
        .iter()
        .map(|address| normalized_address(address))
        .collect::<BTreeSet<_>>();

    let mut entries = monitors().lock().await;
    let stale = entries
        .keys()
        .filter(|address| !connected.contains(*address))
        .cloned()
        .collect::<Vec<_>>();
    for address in stale {
        if let Some(entry) = entries.remove(&address) {
            let _result = entry.shutdown.send(true);
        }
    }

    for address in candidates {
        if !connected.contains(&address) || entries.contains_key(&address) {
            continue;
        }
        let (entry, receiver) = MonitorEntry::new();
        entries.insert(address.clone(), Arc::clone(&entry));
        tokio::spawn(async move {
            monitor_loop(address, entry, receiver).await;
        });
    }
}

/// Returns the latest exact battery packet captured for `address`.
///
/// This accessor intentionally retains the packet beyond transport-level retry
/// windows. Its original observation timestamp is preserved so the normalized
/// registry, not the socket layer, decides when the value becomes stale.
pub async fn latest_apple_accessory_battery(address: &str) -> Option<AppleAccessoryPacket> {
    let key = normalized_address(address);
    let entry = {
        let entries = monitors().lock().await;
        entries.get(&key).cloned()
    }?;
    let latest = entry.latest.read().await.clone();
    latest
}

async fn monitor_loop(
    address: String,
    entry: Arc<MonitorEntry>,
    mut shutdown: watch::Receiver<bool>,
) {
    loop {
        if *shutdown.borrow() {
            return;
        }

        let session_result = tokio::select! {
            result = monitor_session(&address, &entry) => Some(result),
            changed = shutdown.changed() => {
                if changed.is_err() || *shutdown.borrow() {
                    None
                } else {
                    continue;
                }
            }
        };

        let Some(session_result) = session_result else {
            return;
        };
        if let Err(error) = session_result {
            tracing::debug!(device_address = %address, error = %error, "Apple accessory battery monitor disconnected");
        }

        tokio::select! {
            () = tokio::time::sleep(RECONNECT_DELAY) => {}
            changed = shutdown.changed() => {
                if changed.is_err() || *shutdown.borrow() {
                    return;
                }
            }
        }
    }
}

async fn monitor_session(address: &str, entry: &MonitorEntry) -> Result<(), String> {
    let remote: Address = address
        .parse()
        .map_err(|error| format!("invalid Bluetooth address: {error}"))?;
    let socket_address = SocketAddr::new(remote, AddressType::BrEdr, APPLE_ACCESSORY_PSM);
    let socket = tokio::time::timeout(CONNECT_TIMEOUT, SeqPacket::connect(socket_address))
        .await
        .map_err(|_| "L2CAP connection timed out".to_owned())?
        .map_err(|error| format!("L2CAP connection failed: {error}"))?;

    send_packet(&socket, &HANDSHAKE_PACKET).await?;
    tokio::time::sleep(Duration::from_millis(60)).await;
    send_packet(&socket, &REQUEST_NOTIFICATIONS_PACKET).await?;

    let receive_mtu = socket
        .recv_mtu()
        .map_err(|error| format!("failed to read L2CAP receive MTU: {error}"))?
        .max(64);
    let mut buffer = vec![0_u8; receive_mtu];

    loop {
        match tokio::time::timeout(RECEIVE_TIMEOUT, socket.recv(&mut buffer)).await {
            Ok(Ok(received)) => {
                let packet = &buffer[..received];
                if is_complete_battery_packet(packet) {
                    let record_count = packet[6];
                    let observed_at = OffsetDateTime::now_utc();
                    *entry.latest.write().await = Some(AppleAccessoryPacket {
                        payload: packet.to_vec(),
                        observed_at,
                    });
                    tracing::debug!(
                        device_address = %address,
                        record_count,
                        "received exact Apple accessory battery packet"
                    );
                }
            }
            Ok(Err(error)) => return Err(format!("L2CAP receive failed: {error}")),
            Err(_) => send_packet(&socket, &REQUEST_NOTIFICATIONS_PACKET).await?,
        }
    }
}

async fn send_packet(socket: &SeqPacket, packet: &[u8]) -> Result<(), String> {
    let sent = socket
        .send(packet)
        .await
        .map_err(|error| format!("L2CAP send failed: {error}"))?;
    if sent == packet.len() {
        Ok(())
    } else {
        Err(format!(
            "L2CAP packet was only partially sent: expected {} bytes, sent {sent}",
            packet.len()
        ))
    }
}

fn is_complete_battery_packet(packet: &[u8]) -> bool {
    if packet.len() < 7 || packet[..6] != BATTERY_HEADER {
        return false;
    }
    let record_count = usize::from(packet[6]);
    packet.len() >= 7 + record_count * 5
}

#[cfg(test)]
mod tests {
    use super::{is_complete_battery_packet, normalized_address};

    #[test]
    fn normalizes_monitor_keys_across_platform_address_casing() {
        assert_eq!(
            normalized_address("  aa:bb:cc:dd:ee:ff "),
            "AA:BB:CC:DD:EE:FF"
        );
    }

    #[test]
    fn recognizes_complete_exact_battery_notifications_only() {
        assert!(is_complete_battery_packet(&[
            0x04, 0x00, 0x04, 0x00, 0x04, 0x00, 0x01, 0x02, 0x01, 0x44, 0x02, 0x01,
        ]));
        assert!(!is_complete_battery_packet(&[
            0x04, 0x00, 0x04, 0x00, 0x04, 0x00, 0x01, 0x02,
        ]));
        assert!(!is_complete_battery_packet(&[
            0x04, 0x00, 0x04, 0x00, 0x09, 0x00, 0x00,
        ]));
    }
}
