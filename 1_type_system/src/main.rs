// Traits and types

pub mod _const {
    pub mod age {
        pub const ALICE : i64 = 27;
        pub const BOB : i64   = 71;
        pub const CAROL : i64 = 45;
        pub const DAVID : i64 =  2;
    }
    pub mod song_name {
        pub const IMAGINE : &str = "Imagine";
        pub const YESTERDAY : &str = "Yesterday";
        pub const HERE_COMES_THE_SUN : &str = "Here Comes The Sun";
        pub const DONT_PASS_ME_BY : &str = "Don't Pass Me By";
        pub const MACARENA : &str = "Los Del Rio's 'Macarena'";
    }
}
pub mod beatle {
    use crate::_const::song_name;
    use std::fmt;
   
    #[derive(PartialEq,Copy,Clone)]
    pub enum Beatle {
        John,
        Paul,
        George,
        Ringo
    }
    impl fmt::Display for Beatle {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
           match self {
               Beatle::John => write!(f, "John"),
               Beatle::Paul => write!(f, "Paul"),
               Beatle::George => write!(f, "George"),
               Beatle::Ringo => write!(f, "Ringo"),
           }
        }
    }

    pub mod quackslike {
        use std::fmt;
        pub trait Beatle : PartialEq<crate::beatle::Beatle>+Into<crate::beatle::Beatle>+Copy+fmt::Display {}
    }

    impl quackslike::Beatle for Beatle {}

    //This function is not written in idiomatic style. It was written this way to
    //Support "impl trait" arguments without resorting to more advanced features
    //like map_or, closures, etc
    pub fn implied_favorite_song(beatle: Option<impl quackslike::Beatle>) -> String {
        if beatle.is_none() {
            return String::from(song_name::MACARENA);
        } else {
            let _beatle = beatle.unwrap();
            if _beatle==Beatle::John {
                String::from(song_name::IMAGINE)
            } else if _beatle==Beatle::Paul {
                String::from(song_name::YESTERDAY)
            } else if _beatle==Beatle::George {
                String::from(song_name::HERE_COMES_THE_SUN)
            } else if _beatle==Beatle::Ringo {
                String::from(song_name::DONT_PASS_ME_BY)
            } else {
                String::from(song_name::MACARENA)
            }
        }
    }
}

pub mod quackslike {
    use std::fmt::Display;
    use std::ops::Add;
    pub trait _i64 : Add<i64,Output=i64>+PartialOrd<i64>+Into<i64>+Copy+Display {}
    pub trait _String : PartialEq<String>+Into<String>+Clone+Display {}

    impl _i64 for i64 {}
    impl _String for String {}
}

pub mod person {
    use crate::beatle;
    use crate::beatle::Beatle;
    use crate::quackslike;

    #[derive(Debug)]
    pub enum Error {
        EmptyName,
        NegativeYear
    }

    //interfaces
    pub trait Person : Sized {
        fn name(&self) -> String;
        fn age_in_future(
            &self, 
            years: &impl quackslike::_i64
        ) -> Result<i64, Error>;
        fn favorite_beatle(&self) -> Option<Beatle>;
    }
    //implementations
    pub struct _Person {
        name: String,
        age: i64,
        favorite_beatle: Option<Beatle>
    }

    pub fn new(
            name: &impl quackslike::_String, 
            age: &impl quackslike::_i64, 
            favorite_beatle: Option<&impl beatle::quackslike::Beatle>
        ) -> Result<_Person, Error> {
            if (*name)==String::from("") {
                Err(Error::EmptyName)
            }
            else if age < &0 {
                Err(Error::NegativeYear)
            } else {
                Ok(_Person {
                    name: (*name).clone().into(),
                    age: (*age).into(),
                    favorite_beatle: favorite_beatle.map(|b| (*b).into())
                    }
                )
            }
    }

    impl Person for _Person {
        
        fn name(&self) -> String {
            self.name.clone()
        }

        fn age_in_future(&self, years: &impl quackslike::_i64) -> Result<i64, Error> {
            if *years < 0 {
                Err(Error::NegativeYear)
            } else {
                Ok(*years + self.age)
            }
        }

        fn favorite_beatle(&self) -> Option<Beatle> {
            self.favorite_beatle
        }
    }
    pub mod state_verbose {
        use std::fmt::Display;
        use crate::person::{Person,Error,quackslike};
        use crate::beatle;

        pub fn future_age(p: &impl Person, years: &impl quackslike::_i64) -> Result<impl Display, Error> {
            Ok(format!("In {} years, {} will be {}", years, p.name(), p.age_in_future(years)?))
        }
        
        pub fn favorite_song(p: &impl Person) -> impl Display {
            let song = beatle::implied_favorite_song(p.favorite_beatle());
            format!("{}'s favorite song is {}", p.name(), song)
        }
    }
}

pub mod misc {
    use crate::quackslike;
    pub fn far_enough_in_future(years: &impl quackslike::_i64) -> bool {
        years > &5
    }
}

use crate::beatle::Beatle;
use crate::person::{Person,state_verbose,Error};
use crate::misc::far_enough_in_future;
use std::collections::{HashMap};

fn main() -> Result<(), Error> {
    let alice  = person::new(&String::from("Alice"), &_const::age::ALICE,    Some(&Beatle::George))?;  
    let bob    = person::new(&String::from("Bob"),   &_const::age::BOB,      Some(&Beatle::Ringo))?;  
    let carol  = person::new(&String::from("Carol"), &_const::age::CAROL,    Some(&Beatle::Paul))?;
    let david  = person::new(&String::from("David"), &_const::age::DAVID,    None::<&Beatle>)?;  

    let mut age_by_name : HashMap<String,i64> = HashMap::new();
    for person in vec![&alice, &bob, &carol, &david] {
        age_by_name.insert(
            person.name(),
            person.age_in_future(&0)
        ?);
    }
    println!("David's age is {}", age_by_name[&String::from("David")]);

    let mut future_years : Vec<i64> = Vec::new();
    for i in 0..10 {
        future_years.push(i);
    }

    for year in future_years {
        if far_enough_in_future(&year) {
            println!("{}",state_verbose::future_age(&carol, &year)?);
        }
    }
    
    println!("{}",state_verbose::favorite_song(&carol));

    Ok(())
}
