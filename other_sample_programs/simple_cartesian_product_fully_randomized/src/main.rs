use std::iter::repeat_with;
use itertools::Itertools;
use rand::distributions::{Distribution,Uniform};

fn main() {
    let mut rng1 = rand::thread_rng();
    let mut rng2 = rand::thread_rng();
    let die = Uniform::from(1..=6_i32);
    let s = 
        repeat_with(|| die.sample(&mut rng1))
        .take(10)
        .cartesian_product(
            repeat_with(|| die.sample(&mut rng2))
            .take(10)
            .collect::<Vec<i32>>()
            .iter()
        )
        .map(|(x,y)| x*y)
        .fold(0, |x,y| x+y);

    println!("The computation result is: {}", s);
}
