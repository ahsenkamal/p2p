use std::io::{Read, Write};
use std::net::TcpStream;
use crate::protocol::specs::{CURRENT_VERSION, MAX_PACKET_SIZE};
use crate::protocol::enums::{MessageType, ProtocolVersion};
use serde::{Serialize, Deserialize};
use anyhow::{Result, anyhow};

#[derive(Serialize, Deserialize)]
pub struct Packet {
    pub version: ProtocolVersion,
    pub msg_type: MessageType,
    pub payload: Vec<u8>,
}

impl Packet {
    pub fn new(msg_type: MessageType, payload: Vec<u8>) -> Self {
        Self {
            version: CURRENT_VERSION,
            msg_type,
            payload,
        }
    }
    
    pub fn send(&self, stream: &mut TcpStream) -> Result<()> {
        let bytes = bincode::serialize(self)?;
        let len = bytes.len();
        if len > MAX_PACKET_SIZE {
            return Err(anyhow!("frame too large"));
        }

        let len = len as u32;
        stream.write_all(&len.to_be_bytes())?;
        stream.write_all(&bytes)?;

        Ok(())
    }

    pub fn read(stream: &mut TcpStream) -> Result<Self> {
        let mut len_buf = [0u8; 4];
        stream.read_exact(&mut len_buf)?;
        let len = u32::from_be_bytes(len_buf) as usize;

        if len > MAX_PACKET_SIZE {
            return Err(anyhow!("frame too large"));
        }

        let mut buf = vec![0u8; len];
        stream.read_exact(&mut buf)?;

        let frame = bincode::deserialize(&buf)?;
        Ok(frame)
    }
}