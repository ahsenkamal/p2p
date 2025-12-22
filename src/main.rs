use std::net::{TcpListener, TcpStream};
use std::io::{Read};
use std::thread;

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 1024];

    loop {
        match stream.read(&mut buffer) {
            Ok(0) => break,
            Ok(bytes_read) => {
                println!("Received: {}", String::from_utf8_lossy(&buffer[..bytes_read]));
            }
            Err(_) => break,
        }
    }
}

fn main() -> std::io::Result<()>{
    let listener = TcpListener::bind("0.0.0.0:9000")?;
    println!("Listening on port 9000!");

    for stream in listener.incoming() {
        let stream = stream?;
        thread::spawn(|| handle_connection(stream));
    }

    Ok(())
}
