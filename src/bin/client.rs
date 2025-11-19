use std::io:{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::sync::mpsc;
use stf::thread;
use std::env;

// make shares
fn share(data: i32, shares: i32) -> Vec<i32> {
    let mut split: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;

    let mut rng = rand::rng();

    for _ in 0..shares - 1 {

        let num: i32 = rng.random_range(0..=100);
        let mut sign: i32 = rng.random_range(-1..=1);
        
        if sign == 0 {
            sign = 1;
        }

        let value = num * sign;
        sum += value;
        split.push(value);
    }

    // Last share ensures sum of shares equals the secret
    split.push(data - sum);

    split
}

// how they connect to the servers, and send shares
fn connection(std::string hostName, usize share){
    match TcpStream::connect(hostName) {
        Ok(mut stream) => {
            println!("Successfully connected to server {}", hostName);

            stream.write(share).unwrap();
            println!("Sent share, awaiting reply...");

            let mut data = [0 as u8; 6]; // using 6 byte buffer
            match stream.read_exact(&mut data) {
                Ok(_) => {
                    if &data == msg {
                        println!("Reply is ok!");
                    } else {
                        let text = from_utf8(&data).unwrap();
                        println!("Unexpected reply: {}", text);
                    }
                },
                Err(e) => {
                    println!("Failed to receive data: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Failed to connect: {}", e);
        }
    }
    println!("Terminated connection from {}", hostName);
}

fn main() {
    // take in secret number from args
    let args: Vec<String> = env::args().collect();

    let mut secret: i32 = &args[1]; 
    let num_parties: i32 = 3; // given

    let shares = share(secret, num_parties);

    // server 1
    let server1 = String::from("localhost:3333");
    let share1 = usize shares[0];
    connection(server1, share1);

    // server 2
    let server2 = String::from("localhost:3333");
    let share2 = usize shares[1];
    connection(server2, share2);
}


