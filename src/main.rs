mod network;
mod protocol;
mod ui;

use std::{
    collections::HashMap, io, sync::{Arc, Mutex}, thread
};
use anyhow::Result;
use network::connection;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};
use ui::utils::{
    redraw_input,
    list_peers,
    select_peer,
};
use network::Peer;
use crate::{network::{State, discovery}};

fn handle_command(command: String, peers: Arc<Mutex<HashMap<String, Peer>>>, state: Arc<Mutex<State>>) {
    let parts: Vec<&str> = command.trim().split_whitespace().collect();

    if parts.is_empty() {
        return;
    }

    match parts[0] {
        "/list" => {
            list_peers(peers);
        }

        "/connect" => {
            if parts.len() == 1 {
                state.lock().unwrap().change_to("BROADCAST".to_string());
                return;
            }

            let target = parts[1];

            select_peer(target, peers, state);
        }

        _ => {
            println!("Unknown command: {}", parts[0]);
        }
    }
}

fn handle_user(input_buffer: Arc<Mutex<String>>, peers: Arc<Mutex<HashMap<String, Peer>>>, state: Arc<Mutex<State>>) -> Result<()> {
    redraw_input("", state.clone());

    loop {
        if let Event::Key(key) = event::read()? {
            let mut buf = input_buffer.lock().unwrap();

            match key.code {
                KeyCode::Char(c) => buf.push(c),
                KeyCode::Backspace => {
                    buf.pop();
                }
                KeyCode::Enter => {
                    handle_command(buf.clone(), peers.clone(), state.clone());
                    buf.clear();
                }
                KeyCode::Esc => break,
                _ => {}
            }

            redraw_input(&buf, state.clone());
        }
    }

    Ok(())
}
fn main() -> Result<()> {
    terminal::enable_raw_mode()?;

    println!("Please enter username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();

    let state = Arc::new(Mutex::new(State::new(username)));
    let state1 = state.clone();

    let input_buffer = Arc::new(Mutex::new(String::new()));
    let input_buffer_clone = input_buffer.clone();

    let peers:Arc<Mutex<HashMap<String, Peer>>> = Arc::new(Mutex::new(HashMap::new()));
    let peers1 = peers.clone();

    thread::spawn(move || discovery::setup_discovery(peers1));
    thread::spawn(move || connection::setup_server(input_buffer_clone, state1));

    handle_user(input_buffer, peers.clone(), state)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
