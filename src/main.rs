use std::io::(Read, Write);
use std::net::TcpListener;

fn main() {
    let listener = TcpListener::bind("127.0.0.1:6379").unwrap();

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                // On Connection Accepted
                handle_connection(&mut stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_connection(stream: &mut TcpListener) {
    let mut buffer = [0; 512];
    stream.read(&mut buffer).unwrap();

    let response = "+PONG\r\n";
    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}