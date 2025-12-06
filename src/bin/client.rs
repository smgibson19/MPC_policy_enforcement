use rand::Rng;
use std::io::{Read, Write};
use std::net::{TcpStream};
use std::{env};
use std::collections::HashSet;
use std::fs::{File, write};
use serde::{Serialize, Deserialize};
use bincode;

#[derive(Serialize, Deserialize, Debug)]
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
        let new_policy: HashSet<String> = (self.share_policy).intersection(&other.share_policy).cloned().collect();
        SecretShare::new(new_share, new_policy)
    }    
    pub fn reveal(self, name: String) {
        let mut _file = File::create(&name).expect("File creation failed");

        // write to file based on permission
        if self.share_policy.contains(&name) {
            let piece = self.share;
            let results = format!("Sum of values :{piece}"); 
            write(&name, results).expect("Write failed");
        } else {
            write(&name, "Access Denied :O").expect("Write failed");
        }
    }
}

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

            // serialize it to send
            let serialized = bincode::serialize(&private_share).unwrap();

            // writing all share to stream 
            stream.write(&serialized).unwrap();
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
    // assuming user inputs args as follows: num file.txt file2.txt file3.txt
    let args: Vec<String> = env::args().collect();
    println!("{:?}", args);

    let secret = &args[1]; 
    let secret: i32 = secret.parse::<i32>().unwrap();

    // split share
    let num_parties  = 2; // given number, len 2 for everything
    let shares = share(secret, num_parties);

    // policy hashsets
    let mut policy = HashSet::new();
    let (_first, rest) = args.split_first().unwrap();

    for x in rest{
        policy.insert(String::from(x));
    }
    
    // give each piece the same policy that user input
    let num_parties: usize = num_parties as usize;
    let policies = vec![policy; num_parties];

    // need the server names in a list
    // let server_names: Vec<&str> = vec!["localhost:3333", "localhost:3334"];
    let server_names: Vec<&str> = vec!["localhost:3333"];


    // sends one secret to each server
    for x in 0..server_names.len() {
        let private_share = SecretShare{share: shares[x], share_policy: (policies[x]).clone()};

        connection(String::from(server_names[x]), private_share);
    }

    // client decides their policy? sent over the server request 
}