use rand::Rng;
// Import Rng trait
use std::io::{Read, Write};
use std::net::{TcpStream};
use std::{env, primitive};
use std::collections::HashSet;

pub struct SecretShare{
    share: i32,
    share_policy: HashSet<String>
}
impl SecretShare {
    pub fn new(piece: i32, policy: HashSet<String>) -> SecretShare {
        SecretShare{ share: piece, share_policy: policy }
    }
    pub fn add(self, other: SecretShare) -> SecretShare {
        let new_share: i32 = self.share + other.share;

        // add policy conservatively; intersection of two
        let new_policy: HashSet<String> = (self.share_policy).intersection(&other.share_policy).collect();
        SecretShare::new(new_share, new_policy)
    }    
    pub fn reveal(self) {
        // target of reveal is allowed by policy
        // "server1" is in policy = ["server1"]
        if true {
            println!("{}", self.share);
        } else {
            println!("NOT ALLOWED");
        }
    }
}

// add function to access, new struct share = share 1 + share 2/ concat share policy
// reveal function: prints function 

// serializeable/ deserializable
// send over socket easier :)

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
fn connection(host_name: String, private_share: SecretShare){
    match TcpStream::connect(&host_name) {
        Ok(mut stream) => {
            println!("Successfully connected to server {}", host_name);

            // check private_share policy? 
            // if(private_share.share_policy == "all"){

            // }
            // else{

            // }

            // writing all share to stream 
            let share_to_string: [u8; _] = private_share.share.to_be_bytes();
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
    // take arg for list of permissions 
    let secret: i32 = secret.parse::<i32>().unwrap();

    // given variables: 
    let num_parties: i32 = 2;

    // need the server names in a list
    // change when we get list
    let server_names: Vec<&str> = vec!["localhost:3333", "localhost:3334"];
    let policies: Vec<&str> = vec!["all", "none"];

    // split secret up
    let shares = share(secret, num_parties);

    // sends one secret to each server
    for x in 0..server_names.len() {
        let private_share = SecretShare{share: shares[x], share_policy: policies[x].to_string()};
        connection(String::from(server_names[x]), private_share);
    }

    // client decides their policy? sent over the server request 
}