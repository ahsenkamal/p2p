use std::net::{TcpStream, IpAddr};

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
}