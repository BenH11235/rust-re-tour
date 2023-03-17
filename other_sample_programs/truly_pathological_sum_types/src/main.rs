enum Degenerate {
    Member()
}

enum AlsoDegenerate {
    Member0(),
    Member1(),
    Member2(),
}



fn main() {
    let mut canary : i64 = 0;
    
    let v1 : Option<&str> = Some("eighty-seven");
    match v1 {
        None => canary = 0,
        Some(_) => canary = 1,
    }
    
    let v2 : Option<&str> = None;
    match v2 {
        None => canary = 0,
        Some(_) => canary = 1,
    }


    let v3 : Degenerate = Degenerate::Member();
    match v3 {
        Degenerate::Member() => canary = 0,
        _ => unreachable!()
    }

    let v4 : AlsoDegenerate = AlsoDegenerate::Member2();

    match v4 {
        AlsoDegenerate::Member0() => canary = 40,
        AlsoDegenerate::Member1() => canary = 41,
        AlsoDegenerate::Member2() => canary = 42,
    }

}

