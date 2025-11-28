use rand::Rng; // Import Rng trait
use std::io::{Read, Write};
use std::net::{TcpStream};
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
fn connection(host_name: String, share: i32){
    match TcpStream::connect(&host_name) {
        Ok(mut stream) => {
            println!("Successfully connected to server {}", host_name);

            // writing all share to stream 

            let share_to_string: [u8; _] = share.to_be_bytes();
            stream.write(&share_to_string).unwrap();
            println!("Sent share, awaiting reply...");

            let mut buffer = [0u8; 50]; 
            match stream.read(&mut buffer) {
                Ok(_) => {
                    println!("Reply is ok!");
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
    println!("Terminated connection from {}", host_name);
}

fn main() {
    // take in secret number from args
    let args: Vec<String> = env::args().collect();

    let secret = &args[1]; 
    let secret: i32 = secret.parse::<i32>().unwrap();
    let num_parties: i32 = 2; // given

    let shares = share(secret, num_parties);

    // need the server names in a list
    let server_names = vec!["localhost:3333", "localhost:3334"];

    for x in 0..shares.len() {
        connection(String::from(server_names[x]), shares[x]);
    }

    // client decides their policy? sent over the server request 
}