use std::sync::Mutex;
use std::{io, sync::Arc};
use std::io::Write;
use crossterm::{
    cursor,
    execute,
    terminal::{self, ClearType},
};

pub fn redraw_input(input: &str) {
    let mut stdout = io::stdout();
    execute!(
        stdout,
        cursor::MoveToColumn(0),
        terminal::Clear(ClearType::CurrentLine),
    )
    .unwrap();

    print!("> {}", input);
    stdout.flush().unwrap();
}

pub fn write_msg(msg: String, input: Arc<Mutex<String>>) {
    let current = input.lock().unwrap().clone();
    let mut stdout = io::stdout();

    execute!(
        stdout,
        cursor::MoveToColumn(0),
        terminal::Clear(ClearType::CurrentLine),
    )
    .unwrap();

    println!("{}", msg);
    redraw_input(&current);
}