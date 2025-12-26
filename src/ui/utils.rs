use std::sync::Mutex;
use std::{io, sync::Arc};
use std::io::Write;
use crossterm::{
    cursor,
    execute,
    terminal::{self, ClearType},
};
use crate::HashMap;
use crate::network::Peer;
use crate::network::State;

pub fn list_peers(peers: Arc<Mutex<HashMap<String, Peer>>>) {
    let peers = peers.lock().unwrap();

    if peers.is_empty() {
        println!("No peers connected");
    } else {
        println!("Connected peers:");
        for name in peers.keys() {
            println!("  - {}", name);
        }
    }
}

pub fn select_peer(target: &str, peers: Arc<Mutex<HashMap<String, Peer>>>, state: Arc<Mutex<State>>) {
    let peers_guard = peers.lock().unwrap();
    if peers_guard.contains_key(target) {
        state.lock().unwrap().change_to(target.to_string());
        println!("Connected to {}", target);
    } else {
        println!("Peer '{}' not found", target);
    }
}

pub fn redraw_input(input: &str, state: Arc<Mutex<State>>) {
    let mut stdout = io::stdout();
    execute!(
        stdout,
        cursor::MoveToColumn(0),
        terminal::Clear(ClearType::CurrentLine),
    )
    .unwrap();

    print!("{} {}", state.lock().unwrap().get_to(), input);
    stdout.flush().unwrap();
}

pub fn write_msg(msg: String, input: Arc<Mutex<String>>, state: Arc<Mutex<State>>) {
    let current = input.lock().unwrap().clone();
    let mut stdout = io::stdout();

    execute!(
        stdout,
        cursor::MoveToColumn(0),
        terminal::Clear(ClearType::CurrentLine),
    )
    .unwrap();

    println!("{}", msg);
    redraw_input(&current, state);
}