use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::network::State;
use crate::network::Peer;
use crate::protocol::Chat;

pub fn send_chat(text: String, peers: Arc<Mutex<HashMap<String, Peer>>>, state: Arc<Mutex<State>>) {
    let to = String::from(state.lock().unwrap().get_to());
    
    let mut peers = peers.lock().unwrap();
    let peer = peers.get_mut(&to).unwrap();
    let stream = peer.get_tcp_stream();

    let chat = Chat::new(to, text);
    let packet = chat.create_packet().unwrap();
    // packet.send(stream);
}