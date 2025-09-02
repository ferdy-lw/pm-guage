use std::{
    net::Ipv4Addr,
    sync::{
        mpsc::{self, Receiver, SyncSender, TryRecvError},
        LazyLock, RwLock,
    },
    thread,
    time::Duration,
};

use esp_idf_svc::espnow::ReceiveInfo;
use log::{error, info};

use crate::ui::Info;
use crate::MacAddr;

type ChannelData = (MacAddr, heapless::Vec<u8, 250>);

/// Details about the elm device.
pub struct PeerInfo {
    pub mac: MacAddr,
    pub ip: Ipv4Addr,
    pub url_post: String,
}

pub static PEER: LazyLock<RwLock<Option<PeerInfo>>> = LazyLock::new(|| RwLock::new(None));

pub fn create_channel() -> (SyncSender<ChannelData>, Receiver<ChannelData>) {
    mpsc::sync_channel::<ChannelData>(2)
}

pub fn recv_cb(tx_now: &SyncSender<ChannelData>, info: &ReceiveInfo, data: &[u8]) {
    info!("EspNow recv cb: info ({info:?}), data ({data:?})");
    let peer = *info.src_addr;

    tx_now
        .send((peer, heapless::Vec::from_slice(data).unwrap()))
        .unwrap();
}

pub fn elm327_client_address(rx_now: Receiver<ChannelData>, rx_disc: Receiver<MacAddr>) {
    info!("Starting client address thread");
    Info::set_info(&Info::NoPeer);

    loop {
        // First check to see if we got an espnow msg from the elm device that tells
        // us it's IP addr, and that it's ready to receive http requests
        match rx_now.try_recv() {
            Ok((peer_mac, data)) => {
                if let Some(peer_ip) = data.first().and_then(|b| {
                    if b == &0x01 && data.len() == 5 {
                        Some(Ipv4Addr::new(data[1], data[2], data[3], data[4]))
                    } else {
                        None
                    }
                }) {
                    let peer_lock = PEER.write();
                    match peer_lock {
                        Ok(mut peer) => {
                            match &*peer {
                                None => {
                                    *peer = Some(PeerInfo {
                                        mac: peer_mac,
                                        ip: peer_ip,
                                        url_post: format!("http://{peer_ip}/post"),
                                    });
                                    Info::set_info(&Info::None);
                                    info!("Adding peer {peer_ip}");
                                }
                                Some(p) => {
                                    error!("Peer already active with mac ({:?}), ip ({}), ignoring new mac ({:?}), new ip ({})",
                        p.mac, p.ip, peer_mac, peer_ip);
                                }
                            };
                        }
                        Err(e) => {
                            error!("Peer lock error {e}");
                        }
                    }
                } else {
                    error!("Got ESPNOW msg that is not bcast IP: {data:?}");
                }
            }
            Err(TryRecvError::Empty) => {
                // Check to see if any AP clients have disconnected, if it's the elm device then
                // drop the details about it's IP addr so we wont keep sending HTTP requests.
                // If it reconnects to the AP it should send another espnow msg.
                match rx_disc.try_recv() {
                    Ok(mac) => {
                        let peer_lock = PEER.write();
                        match peer_lock {
                            Ok(mut peer) => {
                                match &*peer {
                                    None => {
                                        info!("Peer not active, ignoring mac {mac:?} disconnect");
                                    }
                                    Some(p) => {
                                        if p.mac == mac {
                                            info!("Dropping peer {}", p.ip);
                                            *peer = None;
                                            Info::set_info(&Info::NoPeer);
                                        } else {
                                            info!("Peer already active with mac ({:?}), ip ({}), ignoring mac ({mac:?}) disconnect",
                                    p.mac, p.ip);
                                        }
                                    }
                                };
                            }
                            Err(e) => {
                                error!("Peer lock error {e}");
                            }
                        }
                    }
                    Err(TryRecvError::Empty) => {}
                    Err(TryRecvError::Disconnected) => {
                        error!("Wifi sending channel disconnected. Stopping thread");
                        break;
                    }
                }
            }
            Err(TryRecvError::Disconnected) => {
                error!("ESPNOW sending channel disconnected. Stopping thread");
                break;
            }
        }

        thread::sleep(Duration::from_millis(20));
    }
}
