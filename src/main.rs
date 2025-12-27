mod network;
mod protocol;
mod ui;
mod utils;
mod services;

use std::{collections::HashMap, io, sync::{Arc, Mutex}, thread};
use anyhow::Result;
use network::connection;
use crossterm::terminal;
use network::Peer;
use crate::{network::{State, discovery}};
use utils::handle_user;
use protocol::specs::USERNAME;

fn main() -> Result<()> {
    
    println!("Please enter username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    username = username.trim().to_string();
    USERNAME.set(username.clone()).unwrap();

    let state = Arc::new(Mutex::new(State::new(username)));
    let state1 = state.clone();
    
    let input_buffer = Arc::new(Mutex::new(String::new()));
    let input_buffer_clone = input_buffer.clone();
        
    let peers:Arc<Mutex<HashMap<String, Peer>>> = Arc::new(Mutex::new(HashMap::new()));
    let peers1 = peers.clone();
    
    terminal::enable_raw_mode()?;

    thread::spawn(move || discovery::setup_discovery(peers1));
    thread::spawn(move || connection::setup_server(input_buffer_clone, state1));

    handle_user(input_buffer, peers.clone(), state)?;

    terminal::disable_raw_mode()?;
    Ok(())
}
