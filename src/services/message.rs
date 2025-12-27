use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::network::State;
use crate::network::Peer;
use crate::protocol::Chat;
use crate::ui::core::write_msg;

fn send_to_peer(to: String, peer: &mut Peer, text: String) {
    let stream = peer.get_tcp_stream();
    let chat = Chat::new(to, text);
    let packet = chat.create_packet().unwrap();
    packet.send(stream).unwrap();
}

pub fn send_chat(text: String, peers: Arc<Mutex<HashMap<String, Peer>>>, state: Arc<Mutex<State>>) {
    let to = String::from(state.lock().unwrap().get_to());
    let mut peers = peers.lock().unwrap();
    
    if to == "BROADCAST" {
        for (username, peer) in peers.iter_mut() {
            send_to_peer(username.clone(), peer, text.clone());
        }
    } else {
        let peer = peers.get_mut(&to).unwrap();
        send_to_peer(to, peer, text.clone());
    }

    println!("You: {}", text);
}