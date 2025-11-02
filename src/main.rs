use rand::Rng;

// function that creates secret shares
// data is what is being shared
// shares is how many parties the data is shared between
fn share(data: usize, shares: usize) -> Vec<usize> {
    
    let mut split: Vec<usize> = Vec::with_capacity(shares); 
    let mut sum = 0;


    // calc polynomials
    for x in 0..shares{
        // generate random number
        // add it to sum
        // push it to shares
        let mut rng = rand::thread_rng();      
        let num: usize = rng.gen();

        sum += num * 100;
        split.push(sum);

    }

    // add x- sum to shares
    split.push(data-sum);

    return split;

}

// function for operation on data

fn main(){
    println!("Hey!");
}