#[derive(Debug)]
struct HappyStruct {
    a: String,
    b: i64,
    c: String
}

#[derive(Debug)]
struct SadStruct {
    a: i64,
    b: i64
}

enum SumType0 {
    JustOneOption(i64)
}

enum SumType1 {
    OneOption(i64),
    AnotherOption(i64)
}

enum SumType2 {
    OneOption(i64),
    AnotherOption(String)
}

enum SumType3 {
    JustANumber(i64),
    Happy(HappyStruct)
}

enum SumType4 {
    Happy(HappyStruct),
    Sad(SadStruct),
    JustANumber(i64)
}



fn main() {
    let sumtype_0_0 = SumType0::JustOneOption(10);
    let sumtype_1_0 = SumType1::OneOption(11);
    let sumtype_1_1 = SumType1::AnotherOption(12);
    let sumtype_2_0 = SumType2::OneOption(13);
    let sumtype_2_1 = SumType2::AnotherOption(String::from("Fourteen"));
    let sumtype_3_0 = SumType3::JustANumber(14);
    let sumtype_3_1 = SumType3::Happy(HappyStruct {a: String::from("Fifteen"), b: 16, c: String::from("Seventeen")});
    let sumtype_4_0 = SumType4::Happy(HappyStruct {a: String::from("Eighteen"), b: 19, c: String::from("Twenty")});
    let sumtype_4_1 = SumType4::Sad(SadStruct {a: 21, b: 22});
    let sumtype_4_2 = SumType4::JustANumber(23);

    match sumtype_0_0 {
        SumType0::JustOneOption(x) => println!("0_0: JustOneOption, {}", x),
    }
    match sumtype_1_0 {
        SumType1::OneOption(x) => println!("1_0: OneOption, {}", x),
        SumType1::AnotherOption(x) => println!("1_0: AnotherOption, {}", x),
    }
    match sumtype_1_1 {
        SumType1::OneOption(x) => println!("1_1: OneOption, {}", x),
        SumType1::AnotherOption(x) => println!("1_1: AnotherOption, {}", x),
    }
    match sumtype_2_0 {
        SumType2::OneOption(x) => println!("2_0: OneOption, {}", x),
        SumType2::AnotherOption(x) => println!("2_0: AnotherOption, {}", x),
    }
    match sumtype_2_1 {
        SumType2::OneOption(x) => println!("2_1: OneOption, {}", x),
        SumType2::AnotherOption(x) => println!("2_1: AnotherOption, {}", x),
    }
    match sumtype_3_0 {
        SumType3::JustANumber(x) => println!("3_0: JustANumber, {}", x),
        SumType3::Happy(x) => println!("3_0: Happy, {:?}", x),
    }
    match sumtype_3_1 {
        SumType3::JustANumber(x) => println!("3_1: JustANumber, {}", x),
        SumType3::Happy(x) => println!("3_1: Happy, {:?}", x),
    }
}
    
    
