// === src/network_monitor/mod.rs ===
use pnet::datalink::{self, NetworkInterface};
use pnet::packet::{ethernet::EthernetPacket, Packet};
use crate::controllers::log_controller::log_packet;
use mongodb::Database;

pub async fn start_sniffer(db: Database) {
    let interfaces = datalink::interfaces();
    let interface = interfaces
        .into_iter()
        .find(|iface| iface.is_up() && !iface.ips.is_empty() && !iface.is_loopback())
        .expect("Nu s-a găsit interfață de rețea activă");

    let (_, mut rx) = match datalink::channel(&interface, Default::default()) {
        Ok(datalink::Channel::Ethernet(_, rx)) => (_, rx),
        _ => panic!("Eroare creare canal datalink"),
    };

    log::info!("Monitorizare pe interfața: {}", interface.name);

    while let Ok(packet) = rx.next() {
        if let Some(ethernet) = EthernetPacket::new(packet) {
            let source_mac = ethernet.get_source().to_string();
            let dest_mac = ethernet.get_destination().to_string();
            let ethertype = format!("{:?}", ethernet.get_ethertype());

            let suspicious = ethertype.contains("Arp") || source_mac == dest_mac;

            log_packet(
                &db,
                "0.0.0.0".to_string(),
                "0.0.0.0".to_string(),
                source_mac,
                dest_mac,
                ethertype,
                suspicious,
            )
            .await;
        }
    }
}
