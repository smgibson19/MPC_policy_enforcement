use std::io:{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::sync::mpsc;
use stf::thread;

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

fn client_process() {
    // since number of parties is given, split can happen before connection

    let secret1: i32 = 15; // set this secret as an argument from the commandline

    let num_parties: i32 = 3; // here is given

    let shares1 = share(secret1, num_parties);
    println!("Shares from party 1: {:?}", shares1);

    // 2) connect to servers
    
    // 3) send data to both servers
}


