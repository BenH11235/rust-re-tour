// Traits and types

enum Beatle {
    John,
    Paul,
    George,
    Ringo
}

struct Person {
    name: String,
    age: i64,
    favorite_beatle: Beatle
}

mod constants {
    pub const ALICE_AGE: i64 = 27;
}

fn age_in_future(p: &Person, years: i64) -> i64 {
    p.age + years
}

fn main() {
    let alice = Person {
        name: String::from("Alice"),
        age: constants::ALICE_AGE,
        favorite_beatle: Beatle::George
    };

    let bob = Person {
        name: String::from("Bob"),
        age: 71,
        favorite_beatle: Beatle::Ringo
    };
    let carol = Person {
        name: String::from("Carol"),
        age: 45,
        favorite_beatle:  Beatle::Paul
    };
    for i in 0..10 {
        if i>5 {
            println!(
                "In {} years, Alice will be {}", i, age_in_future(&alice,i)
            );
        }
    }
    let song = match carol.favorite_beatle {
        Beatle::John => "Imagine",
        Beatle::Paul => "Yesterday",
        Beatle::George => "Here Comes The Sun",
        Beatle::Ringo => "Don't Pass Me By"
    }; // should evaluate to "Yesterday"
    println!("Carol's favorite song is {}", song);
}

