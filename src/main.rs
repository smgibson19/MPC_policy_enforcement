use rand::Rng; // Import Rng trait

/// Function that creates secret shares of a given integer.
/// 
/// # Arguments
/// * `data` - The secret integer to split.
/// * `shares` - Number of shares to generate.
/// 
/// # Returns
/// A vector of integers representing the shares.
fn share(data: i32, shares: i32) -> Vec<i32> {
    let mut split: Vec<i32> = Vec::new();
    let mut sum: i32 = 0;

    // Create the random number generator once, outside the loop
    let mut rng = rand::rng();

    for _ in 0..shares - 1 {
        // Generate a random number between 0 and 100
        let num: i32 = rng.random_range(0..=100);

        // Randomly assign a sign: -1 or 1
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

fn main() {
    let foo: i32 = 23;
    let shares: i32 = 3;

    let results: Vec<i32> = share(foo, shares);
    println!("Shares: {:?}", results);

    let reconstructed: i32 = results.iter().sum();
    println!("Reconstructed: {}", reconstructed);
}
