use anyhow::Result;
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::network::Peer;
use crate::network::State;
use crate::ui::core::redraw_input;
use crossterm::event::{self, Event, KeyCode};

pub fn list_peers(peers: Arc<Mutex<HashMap<String, Peer>>>) {
    let peers = peers.lock().unwrap();
    
    println!();
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
    println!();
    if peers_guard.contains_key(target) {
        state.lock().unwrap().change_to(target.to_string());
        println!("Connected to {}", target);
    } else {
        println!("Peer '{}' not found", target);
    }
}

pub fn handle_user(input_buffer: Arc<Mutex<String>>, peers: Arc<Mutex<HashMap<String, Peer>>>, state: Arc<Mutex<State>>) -> Result<()> {
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
            println!("\nUnknown command: {}", parts[0]);
        }
    }
}
