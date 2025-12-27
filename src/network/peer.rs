use std::net::{TcpStream, IpAddr};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;

#[derive(Debug)]
pub struct Peer {
    username: String,
    ip: IpAddr,
    tcp_stream: Option<TcpStream>,
}

impl Peer {
    pub fn new(username: String, ip: IpAddr, tcp_stream: Option<TcpStream>) -> Self {
        Self {
            username,
            ip,
            tcp_stream,
        }
    }

    pub fn get_tcp_stream(&mut self) -> &mut TcpStream {
        self.tcp_stream.get_or_insert_with(|| {
            let addr = format!("{}:9000", self.ip);
            TcpStream::connect(addr).unwrap()
        })
    }
}