use anyhow::{Result, anyhow};
use std::net::{TcpListener, TcpStream};
use std::sync::{Arc, Mutex};
use std::thread;
use crate::protocol::{Packet, Chat};
use crate::protocol::specs::CURRENT_VERSION;
use crate::protocol::enums::{MessageType};

pub fn setup_server(user_input: Arc<Mutex<String>>) -> Result<()> {
    let listener = TcpListener::bind("0.0.0.0:9000")?;
    println!("Listening on port 9000!");

    for stream in listener.incoming() {
        let user_input_clone = user_input.clone();
        let stream = stream?;
        thread::spawn(move || handle_connection(stream, user_input_clone));
    }   

    Ok(())
}

fn handle_connection(mut stream: TcpStream, user_input: Arc<Mutex<String>>) {
    loop {
        let packet = match Packet::read(&mut stream) {
            Ok(f) => f,
            Err(_) => {
                break;
            },
        };

        let _ = handle_packet(packet, user_input.clone());
    }
}

fn handle_packet(packet: Packet, user_input: Arc<Mutex<String>>) -> Result<()> {
    if packet.version != CURRENT_VERSION {
        return Err(anyhow!("version mismatch"));
    }

    match packet.msg_type {
        MessageType::Chat => {
            let chat = Chat::decode(&packet.payload)?;
            chat.print(user_input);
        },
    }

    Ok(())
}