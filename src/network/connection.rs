use anyhow::{Result, anyhow};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::protocol::{Packet, Chat};
use crate::protocol::specs::CURRENT_VERSION;
use crate::protocol::enums::{MessageType};
use crate::network::State;

pub fn setup_server(user_input: Arc<Mutex<String>>, state: Arc<Mutex<State>>) -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9000")?;

    for stream in listener.incoming() {
        let user_input_clone = user_input.clone();
        let state_clone = state.clone();
        let stream = stream?;
        thread::spawn(move || handle_connection(stream, user_input_clone, state_clone));
    }   

    Ok(())
}

fn handle_connection(mut stream: TcpStream, user_input: Arc<Mutex<String>>, state: Arc<Mutex<State>>) {
    loop {
        let packet = match Packet::read(&mut stream) {
            Ok(f) => f,
            Err(_) => {
                break;
            },
        };

        let _ = handle_packet(packet, user_input.clone(), state.clone());
    }
}

fn handle_packet(packet: Packet, user_input: Arc<Mutex<String>>, state: Arc<Mutex<State>>) -> Result<()> {
    if packet.version != CURRENT_VERSION {
        return Err(anyhow!("version mismatch"));
    }

    match packet.msg_type {
        MessageType::Chat => {
            let chat = Chat::decode(&packet.payload)?;
            chat.print(user_input, state);
        },
    }

    Ok(())
}