use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::thread;

// handles a client connection: echo receives data

fn handle_client(mut stream: TcpStream) {
    let mut buffer = [0u8; 50]; 
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // connection closed by client
                println!("Connection closed by client: {}", stream.peer_addr().unwrap());
                break;
            }
            Ok(n) => {
                // echo the data that is read/received
                if let Err(e) = stream.write_all(&buffer[..n]) {
                    eprintln!("Failed to send data to {}: {}", stream.peer_addr().unwrap(), e);
                    break;
                }
            }
            Err(e) => {
                eprintln!("Error reading from {}: {}", stream.peer_addr().unwrap(), e);
                break;
            }
        }
    }

    // error shutdown
    let _ = stream.shutdown(Shutdown::Both);
}

fn main() -> std::io::Result<()> {
    // bind the listener to the port
    let listener = TcpListener::bind("0.0.0.0:3333")?;
    println!("Server listening on port 3333");

    // incoming connection
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection from {}", stream.peer_addr().unwrap());
                // establish connection between clients?
                thread::spawn(move || {
                    handle_client(stream);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    Ok(())
}
