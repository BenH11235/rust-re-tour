use std::iter::repeat_with;
use itertools::Itertools;
use rand::distributions::{Distribution,Uniform};

fn main() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..=6_i32);
    let s = 
        repeat_with(|| die.sample(&mut rng))
        .take(10)
        .cartesian_product(1..4)
        .map(|(x,y)| x*y)
        .fold(0, |x,y| x+y);

    println!("The computation result is: {}", s);
}
