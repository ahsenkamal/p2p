mod network;
mod protocol;
mod ui;

use std::{
    io,
    sync::{Arc, Mutex},
    thread,
};
use anyhow::Result;
use network::connection;
use crossterm::{
    event::{self, Event, KeyCode},
    terminal,
};
use ui::print::redraw_input;
use protocol::specs::USERNAME;

fn handle_user(input_buffer: Arc<Mutex<String>>) -> Result<()> {
    redraw_input("");

    loop {
        if let Event::Key(key) = event::read()? {
            let mut buf = input_buffer.lock().unwrap();

            match key.code {
                KeyCode::Char(c) => buf.push(c),
                KeyCode::Backspace => {
                    buf.pop();
                }
                KeyCode::Enter => {
                    println!();
                    println!("sent: {}", buf.as_str());
                    buf.clear();
                }
                KeyCode::Esc => break,
                _ => {}
            }

            redraw_input(&buf);
        }
    }

    Ok(())
}
fn main() -> Result<()> {
    terminal::enable_raw_mode()?;

    println!("Please enter username:");
    let mut username = String::new();
    io::stdin().read_line(&mut username).unwrap();
    USERNAME.set(username.trim().to_string()).expect("USERNAME already set");

    let input_buffer = Arc::new(Mutex::new(String::new()));
    let input_buffer_clone = input_buffer.clone();

    thread::spawn(move || connection::setup_server(input_buffer_clone));

    handle_user(input_buffer)?;
    terminal::disable_raw_mode()?;

    Ok(())
}
