use std::collections::HashMap;
use std::net::{Ipv4Addr, UdpSocket};
use std::sync::{Arc, Mutex};
use anyhow::Result;
use std::thread;
use std::time::Duration;
use crate::network::Peer;
use crate::protocol::specs::USERNAME;

pub fn setup_discovery(peers: Arc<Mutex<HashMap<String, Peer>>>) -> Result<()> {
    let socket = UdpSocket::bind("0.0.0.0:8888")?;
    let group = Ipv4Addr::new(239, 42, 42, 42);
    let port = 8888;
    let interface = Ipv4Addr::UNSPECIFIED;

    socket.join_multicast_v4(&group, &interface)?;

    let mut buf = [0u8; 1024];

    loop {
        thread::sleep(Duration::from_secs(1));
        socket.send_to(USERNAME.get().unwrap().as_bytes(), (group, port))?;

        let (len, src) = socket.recv_from(&mut buf)?;
        let peer = String::from_utf8_lossy(&buf[..len]).to_string();
        let peer_clone = peer.clone();

        peers.lock().unwrap().insert(peer, Peer::new(peer_clone, src.ip(), None));
    }
}