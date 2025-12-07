// use serde::{Serialize, Deserialize};
use bincode;
// use std::collections::HashSet;
use std::io::{Read};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::sync::{Arc, Mutex};
use std::thread;
mod client;
use crate::client::SecretShare;

// pub struct SecretShare {
//     pub share: i32,      
//     pub share_policy: String,
// }

// impl SecretShare {
//     pub fn add(self, other: SecretShare) -> SecretShare {
//         let new_value = self.share + other.share;
//         let new_policy = self.share_policy.intersection(&other.share_policy).cloned().collect();
//         // need to change this based on what we want the function to return. example below
//         // let new_policy = if self.share_policy == other.share_policy {
//         //     self.share_policy.clone()
//         // } else {
//         //     "none".to_string()
//         // };
//         SecretShare { share: new_value, share_policy: new_policy }
//     }
// }

// fn add_secret_shares(shares: Vec<SecretShare>) {
//     let output = shares[0].add(shares[1]).add(shares[2]);
//     let name = String::from("server1.txt");
//     output.reveal(name);
// }


// sum a vector of shares

//fn add_numbers(shares: Vec<usize>) -> usize {
//    shares.iter().sum()
//}


// handles a client connection

fn handle_client(mut stream: TcpStream, shared_shares: Arc<Mutex<Vec<SecretShare>>>,
    thread_count: Arc<Mutex<usize>>,) {
    let mut buffer = [0u8; 128]; 
    loop {
        match stream.read(&mut buffer) {
            Ok(0) => {
                // connection closed by client
                println!("Connection closed by client: {}", stream.peer_addr().unwrap());
                break;
            }
            Err(e) => {
                eprintln!("Error reading from {}: {}", stream.peer_addr().unwrap(), e);
                break;
            }
            Ok(_n) => {
                let parsed: SecretShare = bincode::deserialize(&buffer).unwrap();

                // list of shares from all clients
                let mut vec = shared_shares.lock().unwrap();
                vec.push(parsed);

                let mut count = thread_count.lock().unwrap();
                *count += 1;

                if *count == 3 {
                    // let vec = shared_shares.lock().unwrap();
                    // add_secret_shares(*vec)

                    let output = vec[0].clone().add(vec[1].clone()).add(vec[2].clone());
                    let name = String::from("server1.txt");
                    output.reveal(name);

                    shared_shares.lock().unwrap().clear();
                    *count = 0;
                }
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
    let new_shares = Arc::new(Mutex::new(Vec::<SecretShare>::new()));
    // shared thread counter
    let count = Arc::new(Mutex::new(0));

    // incoming connection
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("New connection from {}", stream.peer_addr().unwrap());

                let shared_shares = Arc::clone(&new_shares);
                let thread_count = Arc::clone(&count);
                // establish connection between clients?
                thread::spawn(move || {
                    handle_client(stream, shared_shares, thread_count);
                });
            }
            Err(e) => {
                eprintln!("Connection failed: {}", e);
            }
        }
    }
    Ok(())
}
