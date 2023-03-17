


fn main() {
    let mut canary : i64 = 0;
    let v1 : Option<i64> = None;

    match v1 {
        None => canary = 0,
        Some(_) => canary = 1,
    }
    
    let v2 : Option<i64> = Some(31);
    match v2 {
        None => canary = 0,
        Some(_) => canary = 1,
    }


    let v3 : Option<String> = None;
    match v3 {
        None => canary = 0,
        Some(_) => canary = 1,
    }

    let v4 : Option<String> = Some(String::from("thirty-two"));
    match v4 {
        None => canary = 0,
        Some(_) => canary = 1,
    }

    let v5 : Option<Vec<i64>> = None;
    match v5 {
        None => canary = 0,
        Some(_) => canary = 1,
    }

    let v6 : Option<Vec<i64>> = Some(vec![33,34,35,6]);
    match v6 {
        None => canary = 0,
        Some(_) => canary = 1,
    }

}
    
    
