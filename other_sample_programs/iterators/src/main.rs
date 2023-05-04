use core::iter::repeat;
use itertools::iterate;

fn main() {
    let x1 = 
        (0..10)
        .sum::<i32>();

    let x2 = 
        (0..20)
        .map(|x| x*19)
        .sum::<i32>();
    
    (0..30)
    .for_each(|x| println!("{}",x));

    (0..40)
    .map(|x| x+7)
    .for_each(|x| println!("{}",x));

    let v1 : Vec<i32> = 
        repeat(6)
        .take(7)
        .collect();

    let v2 : Vec<i32> = 
        iterate(1, |x| (5*x)+9)
        .take(5)
        .chain(
            repeat(7)
            .take(8)
        )
        .collect();

    let v3 : Vec<i32> = 
        iterate(0, |x| x+2)
        .take_while(|&x| x<10)
        .cycle()
        .take(10)
        .collect();


}
