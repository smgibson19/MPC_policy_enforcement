use std::io::{Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::sync::{Arc, Mutex};
use std::thread;

// sum a vector of shares
fn add_numbers(shares: Vec<usize>) -> usize {
    shares.iter().sum()
}


// handles a client connection

fn handle_client(mut stream: TcpStream, shared_shares: Arc<Mutex<Vec<usize>>>,
    thread_count: Arc<Mutex<usize>>,) {
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
                //if let Err(e) = stream.write_all(&buffer[..n]) {
                //    eprintln!("Failed to send data to {}: {}", stream.peer_addr().unwrap(), e);
                //    break;
                //}
                let received = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                match received.parse::<usize>() {
                    Ok(share) => {
                        {
                            let mut shares = shared_shares.lock().unwrap();
                            shares.push(share);
                        }

                        // thread count increment
                        let mut count = thread_count.lock().unwrap();
                        *count += 1;

                        println!(
                            "Received share: {} | Thread count: {}",
                            share, *count
                        );

                        // call add_numbers when thread count is 3
                        if *count >= 3 {
                            let sum = {
                                let shares = shared_shares.lock().unwrap();
                                add_numbers((*shares).clone())
                            };
                            println!("Sum of shares: {}", sum);

                            // reset count for next input
                            let mut shares = shared_shares.lock().unwrap();
                            shares.clear();
                            *count = 0;
                        }
                    }
                    Err(_) => {
                        println!("Invalid input from {}: {}", stream.peer_addr().unwrap(), received);
                    }
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

    // shared vector to collect shares
    let shared_shares = Arc::new(Mutex::new(Vec::new()));
    // shared thread counter
    let thread_count = Arc::new(Mutex::new(0));

    // incoming connection
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection from {}", stream.peer_addr().unwrap());

                let shares_clone = Arc::clone(&shared_shares);
                let count_clone = Arc::clone(&thread_count);
                // establish connection between clients?
                thread::spawn(move || {
                    handle_client(stream, shares_clone, count_clone);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }

    // mutex question, trying to pass mutex is giving weird 
    // struct -> sesame stuff ask some specific syntax about
    // I think we should have both servers print out result to cmd line 
    // we calculate manual for now

    Ok(())
}
