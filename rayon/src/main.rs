//rayon
use rayon::prelude::*;

fn sum_of_squares(input: &[i32]) -> i32 {
    input.par_iter() // <-- just change that!
         .map(|&i| i * i)
         .sum()
}

fn main() {
    let mut v = vec![];
    for i in 1..1000{
        v.push(i);
    }
    println!("{}",sum_of_squares(&v))
}

