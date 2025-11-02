use rand::Rng;

// function that creates secret shares
// data is what is being shared
// shares is how many parties the data is shared between
fn share(data: i32, shares: i32) -> Vec<i32> {
    
    let mut split: Vec<i32> = Vec::new(); 
    let mut sum: i32 = 0;


    // calc polynomials
    for _x in 0..shares-1{
        let num: i32 = rand::thread_rng().gen_range(0..=100);  
        let mut sign: i32 = rand::thread_rng().gen_range(-1..=1);

        if sign == 0{
            sign += 1;
        }

        sum += num* sign;
        split.push(sum);

    }

    // add x- sum to shares
    split.push(data-sum);

    return split;

}

// function for operation on data

fn main(){
    let foo:i32 = 23;
    let shares = 3;

    let results: Vec<i32> = share(foo, shares);

    println!("{:?}", results);
}