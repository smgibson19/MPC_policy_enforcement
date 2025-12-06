use serde::{Serialize, Deserialize};
use std::collections::HashSet;
use std::io::{Read};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::sync::{Arc, Mutex};
use std::thread;



pub struct SecretShare {
    pub share: i32,      
    pub share_policy: String,
}

impl SecretShare {
    pub fn add(self, other: SecretShare) -> SecretShare {
        let new_value = self.share + other.share;
        let new_policy = self.share_policy.intersection(&other.share_policy).cloned().collect();
        // need to change this based on what we want the function to return. example below
        // let new_policy = if self.share_policy == other.share_policy {
        //     self.share_policy.clone()
        // } else {
        //     "none".to_string()
        // };
        SecretShare { share: new_value, share_policy: new_policy }
    }
}

fn add_secret_shares(shares: Vec<SecretShare>) -> SecretShare {
    let mut shares_sum = shares[0].clone();
    for s in shares.into_iter().skip(1) {
        shares_sum = shares_sum.add(s);
    }
    shares_sum
}


// sum a vector of shares

//fn add_numbers(shares: Vec<usize>) -> usize {
//    shares.iter().sum()
//}


// handles a client connection

fn handle_client(mut stream: TcpStream, shared_shares: Arc<Mutex<Vec<SecretShare>>>,
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
                let received = String::from_utf8_lossy(&buffer[..n]).trim().to_string();
                //match received.parse::<usize>() {
                //deserializing the received share and policy
                let parsed: SecretShare = match serde_json::from_str(&received) {
                    Ok(share) => share, 
                    // {
                    //     {
                    //         let mut shares = shared_shares.lock().unwrap();
                    //         shares.push(share);
                    //     }

                    //     // thread count increment
                    //     let mut count = thread_count.lock().unwrap();
                    //     *count += 1;

                    //     println!(
                    //         "Received share: {} | Thread count: {}",
                    //         share, *count
                    //     );

                    //     // call add_numbers when thread count is 3
                    //     if *count >= 3 {
                    //         let sum = {
                    //             let shares = shared_shares.lock().unwrap();
                    //             add_numbers((*shares).clone())
                    //         };
                    //         println!("Sum of shares: {}", sum);

                    //         // reset count for next input
                    //         let mut shares = shared_shares.lock().unwrap();
                    //         shares.clear();
                    //         *count = 0;
                    //     }
                    // }
                    Err(_) => {
                        println!("Invalid input from {}: {}", stream.peer_addr().unwrap(), received);
                    }
                };

                {
                    let mut vec = shared_shares.lock().unwrap();
                    vec.push(parsed.clone());
                }

                let mut count = thread_count.lock().unwrap();
                *count += 1;

                println!("Received share: {:?}", parsed);

                if *count == 3 {
                    let final_ans = {
                        let vec = shared.lock().unwrap();
                        add_secret_shares(vec.clone())
                    };

                    println!("Output:");
                    println!("Sum = {}", final_ans.share);
                    println!("Policy = {:?}", final_ans.share_policy);
                    println!("\n");

                    shared.lock().unwrap().clear();
                    *c = 0;
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

    // how to communicate between servers? 
    // do we need to async anything? 
    // how do we retrieve answers from the client/ where do we output them?
    // I think we should have both servers print out result to cmd line 
    // we calculate manual for now

    Ok(())
}
