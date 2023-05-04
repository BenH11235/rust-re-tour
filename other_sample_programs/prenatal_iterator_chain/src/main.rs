use itertools::iterate;
use rand::distributions::{Distribution,Uniform};

fn main() {
    let mut rng = rand::thread_rng();
    let die = Uniform::from(1..=6);
    let s = 
        iterate(1, |x| x + die.sample(&mut rng))
        .take_while(|x| x < &5000)
        .map(|x| x*17)
        .fold(0, |x,y| x+y);

    println!("The sum is: {}", s);
}
