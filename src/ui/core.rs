use std::sync::Mutex;
use std::{io, sync::Arc};
use std::io::Write;
use crossterm::{
    cursor,
    execute,
    terminal::{self, ClearType},
};
use crate::network::State;

pub fn redraw_input(input: &str, state: Arc<Mutex<State>>) {
    let mut stdout = io::stdout();
    execute!(
        stdout,
        cursor::MoveToColumn(0),
        terminal::Clear(ClearType::CurrentLine),
    )
    .unwrap();

    print!("{}> {}", state.lock().unwrap().get_to(), input);
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

    println!("\n{}", msg);
    redraw_input(&current, state);
}