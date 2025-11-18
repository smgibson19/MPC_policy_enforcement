use rand::Rng; // Import Rng trait
use std::io:{ErrorKind, Read, Write};
use std::net::{TcpListener, TcpStream, Shutdown};
use std::sync::mpsc;
use stf::thread;

mod mpcClient;
mod mpcServer;

// /// Function that creates secret shares of a given integer
// /// in client
// fn share(data: i32, shares: i32) -> Vec<i32> {
//     let mut split: Vec<i32> = Vec::new();
//     let mut sum: i32 = 0;

//     let mut rng = rand::rng();

//     for _ in 0..shares - 1 {

//         let num: i32 = rng.random_range(0..=100);
//         let mut sign: i32 = rng.random_range(-1..=1);
        
//         if sign == 0 {
//             sign = 1;
//         }

//         let value = num * sign;
//         sum += value;
//         split.push(value);
//     }

//     // Last share ensures sum of shares equals the secret
//     split.push(data - sum);

//     split
// }

fn main() {
   let secret1: i32 = 15;
    let secret2: i32 = 23;

    let num_parties: i32 = 3;

    let shares1 = share(secret1, num_parties);
    let shares2 = share(secret2, num_parties);

    println!("Shares from party 1: {:?}", shares1);
    println!("Shares from party 2: {:?}", shares2);

    let mut compute_sums: Vec<i32> = Vec::new();
    for i in 0..num_parties as usize {
        let sum = shares1[i] + shares2[i];
        compute_sums.push(sum);
    }

    println!("Compute party sums: {:?}", compute_sums);

    let total_sum: i32 = compute_sums.iter().sum();
    println!("Reconstructed total sum: {}", total_sum);
}
