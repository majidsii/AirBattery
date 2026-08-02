//! Event-driven Apple accessory battery monitoring over classic `Bluetooth` `L2CAP`.

use std::{
    collections::BTreeMap,
    sync::{Arc, OnceLock},
    time::Duration,
};

use bluer::{
    Address, AddressType,
    l2cap::{SeqPacket, SocketAddr},
};
use time::OffsetDateTime;
use tokio::sync::{Mutex, RwLock};

const APPLE_ACCESSORY_PSM: u16 = 0x1001;
const CONNECT_TIMEOUT: Duration = Duration::from_secs(4);
const RECEIVE_TIMEOUT: Duration = Duration::from_secs(20);
const RECONNECT_DELAY: Duration = Duration::from_secs(4);
const PACKET_FRESHNESS: time::Duration = time::Duration::seconds(30);
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

#[derive(Debug, Default)]
struct MonitorEntry {
    latest: RwLock<Option<AppleAccessoryPacket>>,
}

static MONITORS: OnceLock<Mutex<BTreeMap<String, Arc<MonitorEntry>>>> = OnceLock::new();

fn monitors() -> &'static Mutex<BTreeMap<String, Arc<MonitorEntry>>> {
    MONITORS.get_or_init(|| Mutex::new(BTreeMap::new()))
}

/// Starts an event-driven monitor for `address` when necessary and returns its
/// latest fresh exact battery packet without blocking the normal scan cycle.
pub async fn latest_apple_accessory_battery(address: &str) -> Option<AppleAccessoryPacket> {
    let (entry, created) = {
        let mut entries = monitors().lock().await;
        if let Some(entry) = entries.get(address) {
            (Arc::clone(entry), false)
        } else {
            let entry = Arc::new(MonitorEntry::default());
            entries.insert(address.to_owned(), Arc::clone(&entry));
            (entry, true)
        }
    };

    if created {
        let monitor_address = address.to_owned();
        let monitor_entry = Arc::clone(&entry);
        tokio::spawn(async move {
            monitor_loop(monitor_address, monitor_entry).await;
        });
    }

    let latest = entry.latest.read().await.clone();
    latest.filter(|packet| OffsetDateTime::now_utc() - packet.observed_at <= PACKET_FRESHNESS)
}

async fn monitor_loop(address: String, entry: Arc<MonitorEntry>) {
    loop {
        if let Err(error) = monitor_session(&address, &entry).await {
            tracing::debug!(device_address = %address, error = %error, "Apple accessory battery monitor disconnected");
        }
        tokio::time::sleep(RECONNECT_DELAY).await;
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
    use super::is_complete_battery_packet;

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
