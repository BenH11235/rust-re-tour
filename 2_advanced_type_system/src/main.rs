// Traits and types


pub mod quackslike {
    use std::ops::{Add,Sub,Mul,Div};
    use std::fmt::Display;

    pub trait QuacksLike<S> : 
        PartialEq<S>
        +Into<S>
        +Clone
        +Display {
            // no addditional requirements
    }
    
    impl<S,T> QuacksLike<S> for T 
    where T: 
        PartialEq<S>
        +Into<S>
        +Clone
        +Display {
        // no code necessary
    }

    pub trait Algebra<S>: 
        Add<S,Output=S>
        +Sub<S,Output=S>
        +Mul<S,Output=S>
        +Div<S,Output=S>
        +PartialOrd<S> {
            // no additional requirements
    }
    
    impl<S,T> Algebra<S> for T
    where T: 
        Add<S,Output=S>
        +Sub<S,Output=S>
        +Mul<S,Output=S>+Div<S,Output=S>+PartialOrd<S> {
        // no code necessary
    }

}


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
    use std::cmp::Ordering;
    
    pub enum Error {
        UnknownBeatle,
    }

    #[derive(Copy,Clone,PartialEq)]
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
    
    impl PartialOrd<Beatle> for Beatle {
        fn partial_cmp(&self, _: &Beatle) -> Option<Ordering> {
            Some(Ordering::Equal)
        }
    }


    //Toy trait and impl for tour of Box<dyn trait>


    pub trait Song {
        fn beatle(&self) -> Box<Option<Beatle>>;
        fn title(&self) -> Box<String>;
        fn release_year(&self) -> Box<u64>;
        fn text_repr(&self) -> Box<String>;

    }
    
    pub struct _Song {
        beatle: Option<Beatle>,
        title: &'static str,
        release_year: u64
    }
    
    

    impl Song for _Song {
        fn beatle(&self) -> Box<Option<Beatle>> {
            Box::new(self.beatle)
        }
        fn title(&self) -> Box<String> {
            Box::new(String::from(self.title))
        }
        fn release_year(&self) -> Box<u64> {
            Box::new(self.release_year)
        }
        fn text_repr(&self) -> Box<String> {
            Box::new(match *(self.beatle()) {
                Some(b) =>
                    format!("Artist: {}, Title: {}, Release Year: {}", b, self.title(), self.release_year()),
                None => 
                    format!("Not by a Beatle, Title: {}, Release Year: {}",  self.title(), self.release_year()),
            })
        }
    }

    const SONGS : [_Song; 5] = [
        _Song {
            beatle: Some(Beatle::John), 
            title: song_name::IMAGINE, 
            release_year: 1971
        },
        _Song {
            beatle: Some(Beatle::Paul), 
            title: song_name::YESTERDAY, 
            release_year: 1965
        },
        _Song {
            beatle: Some(Beatle::George), 
            title: song_name::HERE_COMES_THE_SUN, 
            release_year: 1969
        },
        _Song {
            beatle: Some(Beatle::Ringo), 
            title: song_name::DONT_PASS_ME_BY, 
            release_year: 1968
        },
        _Song {
            beatle: None, 
            title: song_name::MACARENA, 
            release_year: 1993
        }
    ];
    
    pub fn implied_favorite_song(beatle: Option<Beatle>) -> Result<Box<dyn Song>,Error> {
        Ok(Box::new(
            SONGS
            .into_iter()
            .find(|s| beatle == s.beatle)
            .ok_or(Error::UnknownBeatle)?)
        )
    }
    
}



pub mod person {
    use crate::quackslike::{QuacksLike,Algebra};
    use crate::beatle::Beatle;

    #[derive(Debug)]
    pub enum Error {
        EmptyName,
        NegativeYear,
        FavoriteSongFail,
        NoSuchPerson
    }

    //interfaces
    pub trait Person : Sized  {
        fn name(&self) -> String;
        fn age_in_future<T>(&self, years: &T) -> Result<i64, Error> 
            where T: QuacksLike<i64>+Algebra<i64>+Copy; 
        fn favorite_beatle(&self) -> Option<Beatle>;
    }
    //implementations
    pub struct _Person {
        name: String,
        age: i64,
        favorite_beatle: Option<Beatle>
    }

    pub fn new<T1,T2,T3>(name: &T1, age: &T2, favorite_beatle: Option<&T3>) -> Result<_Person, Error>
    where 
        T1: QuacksLike<String>,
        T2: QuacksLike<i64>+Algebra<i64>+Copy,
        T3: QuacksLike<Beatle>+Copy {
            [
                ((*name)!=String::from(""), Err(Error::EmptyName)),
                (age >= &0, Err(Error::NegativeYear))
            ]
            .into_iter()
            .find(|(cond,_)| cond == &false)
            .map_or_else( || Ok(()), |(_,e)| e)?;
                
            Ok(_Person {
                name: (*name).clone().into(),
                age: (*age).into(),
                favorite_beatle: favorite_beatle.map(|b| (*b).into())
            })
        }
    

    impl Person for _Person {
        
        fn name(&self) -> String {
            self.name.clone()
        }
        fn age_in_future<T>(&self, years: &T) -> Result<i64, Error> 
        where T: QuacksLike<i64>+Algebra<i64>+Copy { 
            [
                (*years >= 0, Err(Error::NegativeYear)),
            ]
            .into_iter()
            .find(|(cond,_)| cond == &false)
            .map_or_else( || Ok(()), |(_,e)| e)?;

            Ok(*years + self.age)
        }

        fn favorite_beatle(&self) -> Option<Beatle> {
            self.favorite_beatle
        }
    }

    pub mod state_verbose {
        use crate::quackslike::{QuacksLike,Algebra};
        use std::fmt::Display;
        use crate::person::{Person,Error};
        use crate::beatle;

        pub fn future_age<T1,T2>(p: &T1, years: &T2) -> Result<impl Display, Error> 
        where 
            T1: Person,
            T2: QuacksLike<i64>+Algebra<i64>+Copy {
                Ok(format!("In {} years, {} will be {}", years, p.name(), p.age_in_future(years)?))
        }
        
        pub fn favorite_song<T>(p: &T) -> Result<impl Display, Error> 
        where T: Person {
            let song : Box<dyn beatle::Song> = beatle::implied_favorite_song(p.favorite_beatle()).map_err(|_| Error::FavoriteSongFail)?;
            Ok(format!("{}'s favorite song -- {}", p.name(), song.text_repr()))
        }
    }
}

pub mod misc {
    use crate::quackslike::{QuacksLike,Algebra};
    pub fn far_enough_in_future<T>(years: &T) -> bool
    where T: QuacksLike<i64>+Algebra<i64> {
        years > &5
    }
}

use rayon::prelude::*;
use crate::beatle::Beatle;
use itertools::Itertools;
use crate::person::{Person,state_verbose,Error};
use crate::misc::far_enough_in_future;
use std::collections::HashMap;

fn main() -> Result<(), Error> {
    let people =
        [
            (&"Alice",   &_const::age::ALICE,    Some(&Beatle::George)),
            (&"Bob",     &_const::age::BOB,      Some(&Beatle::Ringo)),
            (&"Carol",   &_const::age::CAROL,    Some(&Beatle::Paul)),
            (&"David",   &_const::age::DAVID,    None::<&Beatle>)
        ]
        .into_par_iter()
        .map(|t| 
             person::new(t.0, t.1, t.2)
             .map(|p| (p.name(), p))
        )
        .collect::<Result<HashMap<_,_>,Error>>()?;
    
    //state everyone's future ages, starting 6 years in future
    people
    .values()
    .cartesian_product(0..10)
    .filter(|(_,y)| far_enough_in_future(y))
    .map(|(p,y)| state_verbose::future_age(p, &y))
    .collect::<Result<Vec<_>,Error>>()?
    .iter()
    .for_each(|s| println!("{}",s));

    //print Carol's favorite song
    println!(
        "{}",
        state_verbose::favorite_song(
            people
            .get("Carol")
            .ok_or(Error::NoSuchPerson)?
        )?
    );
    
    Ok(())
}
