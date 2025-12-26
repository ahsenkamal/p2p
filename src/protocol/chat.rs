use std::sync::{Arc, Mutex};
use crate::{protocol::{Packet, enums::MessageType}, ui::core::write_msg};
use crate::protocol::specs::USERNAME;
use anyhow::Result;
use serde::{Serialize, Deserialize};
use crate::network::State;

#[derive(Serialize, Deserialize, Debug)]
pub struct Chat {
    pub from: String,
    pub to: String,
    pub text: String,
}

impl Chat {
    pub fn new(to: String, text: String) -> Self {
        Self {
            from: USERNAME.get().unwrap().clone(),
            to,
            text,
        }
    }

    pub fn decode(payload: &[u8]) -> Result<Self> {
        Ok(bincode::deserialize(payload)?)
    }

    pub fn print(&self, input: Arc<Mutex<String>>, state: Arc<Mutex<State>>) {
        let msg = format!("{}: {}", self.from, self.text);
        write_msg(msg, input, state);
    }

    pub fn create_packet(&self) -> Result<Packet> {
        let payload = bincode::serialize(&self)?;
        Ok(Packet::new(MessageType::Chat, payload))
    }
}