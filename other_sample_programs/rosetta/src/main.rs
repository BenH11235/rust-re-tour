//Containers
//Iterators
//Panics
use std::collections::{HashMap, HashSet};
use std::fs;
use std::net::{Shutdown,TcpStream};
use std::io::prelude::*;
use std::process::{Command,Stdio};
use std::thread;
use std::time::{Duration, Instant};

fn vec() {
    let mut v1 : Vec<i32> = Default::default();
    let mut v2: Vec<i32> = Default::default();
    v1.push(4);
    v1.capacity();
    v1.pop();
    v1.append(&mut v2);
    v1.drain(2..4);
    v1.insert(4,3);
    v1.remove(1);
    v1.dedup();
    v1.clear();
    v1.splice(0..3,v2);
    let _ = v1 == v1;
    println!("{:?}",v1);
}


fn hashmap() {
    let mut m  : HashMap<i32,i32> = Default::default();
    m.keys();
    m.values();
    m.iter();
    m.len();
    m.is_empty();
    m.drain();
    m.clear();
    m.get(&1);
    m.contains_key(&2);
    m.insert(4,4);
    m.remove(&4);
    m.clone().into_iter();
    m.extend(HashMap::from([]).iter());
    let _  =  m==m;
    println!("{:?}", m);
}

fn slice() {
    let mut s : [i32;4] = [1,2,3,4];
    s.iter();
    s.windows(4);
    s.chunks(4);
    s.first();
    s.last();
    s.get(2);
    s.len();
    s.is_empty();
    s.reverse();
    s.swap(2,3);
    s.iter();
    s.clone().into_iter();
}

fn hashset() {
    let mut s : HashSet<i32> = Default::default();
    s.capacity();
    s.clear();
    s.contains(&3);
    s.difference(&s);
    s.symmetric_difference(&s);
    s.intersection(&s);
    s.union(&s);
    s.contains(&3);
    s.is_disjoint(&s);
    s.is_subset(&s);
    s.is_superset(&s);
    s.replace(5);
    s.remove(&5);
    s.take(&5);
    s.drain();
    s.get(&5);
    s.clone();
    s.extend(HashSet::from([1,2,3]).iter());
    s.iter();
    s.clone().into_iter();
    let _ = s==s;
    println!("{:?}",s);
}

fn float64() {
    let x : f64 = 4.0;
    x.sqrt();
    x.abs();
    x.cos();
    x.sin();
    x.ceil();
    x.floor();
    x.exp();
    x.ln();
    x.log2();
    x.round();
}

 fn float32() {
    let x : f32 = 4.0;
    x.sqrt();
    x.abs();
    x.cos();
    x.sin();
    x.ceil();
    x.floor();
    x.exp();
    x.ln();
    x.log2();
    x.round();
}

fn file() {
    fs::canonicalize("test.ext");
    fs::copy("test1.ext","test2.ext");
    fs::create_dir("/fictitious/path/");
    fs::read("test.ext");
    fs::remove_dir("/fictitious/path/");
    fs::remove_file("test.ext");
    fs::rename("test1.ext","test2.ext");
    fs::write("test.ext",b"AAAAAAAA");
}

fn string() {
    let mut s : String = Default::default();
    s.insert(1,'b');
    s.push('w');
    s.is_empty();
    s.len();
    s.remove(2);
    s.chars();
    s.contains("test");
    let _ = s==s;
    println!("{:?}",s);
    println!("{}",s);
    s.is_ascii();
    s.lines();
    s.replace("a","b");
    s.split("/");
    s.split_at(5);
    s.starts_with("one thing I don't know why");
    s.trim();
    s.to_lowercase();
    s.to_uppercase();
    let _ = s.clone()+&s;

}

fn tcp() {
    let mut stream = TcpStream::connect("127.0.0.1:32768").unwrap();
    stream.write(&[1]);
    stream.read(&mut [0; 128]);
    stream.peer_addr();
    stream.local_addr();
    stream.shutdown(Shutdown::Both);
    stream.peek(&mut [0; 128]);
    stream.set_ttl(1024);
    stream.take_error();
}

fn command() {
    let cmd_out = 
        Command::new("echo")
        .arg("Hello world")
        .current_dir("/")
        .stdin(Stdio::null())
        .stdout(Stdio::null())
        .env_remove("PATH")
        .env("Test1", "Test2")
        .output();
}
  
fn thread() {
    let t = thread::Builder::new().name("thread1".to_string()).spawn(move || {}).unwrap();
    let cur = thread::current();
    cur.id();
    cur.name();
    println!("{:?}", t);
}

fn time() {
    let t1 = Instant::now();
    let t2 = Instant::now();
    t2.clone();
    let d = t2 - t1;
    t2.duration_since(t1);
    let t3 = t1+2*d;
    let t4 = t2 - 5*d;
    println!("{:?}", t3);
    println!("{:?}",d);
    let _ = t1 == t1;
    let _ = t1 < t2;
    t2.elapsed();
}

fn iterator() {
    let mut v = vec![1,2,3];
    v.iter().map(|x| x+1);
    v.iter().chain(v.iter());
    v.iter().cycle();
    v.iter().enumerate();
    v.iter().rev();
    v.iter().scan(1, |_,_| Some(1));
    v.iter().skip(19);
    v.iter().step_by(20);
    v.iter().take(40);
    v.iter().skip_while(|x| true);
    v.iter().zip(v.iter());
    v.iter().fold(0, |x,y| x+y);
    v.iter().nth(4);
    v.iter().find(|x| true);
    v.iter().last();
    v.iter().for_each(|_| {} );
    v.iter().reduce(|a,i| i);
    v.iter().all(|_| true);
    v.iter().any(|_| false);
    v.iter().position(|x| true);
    v.iter().sum::<i32>();
    v.iter().product::<i32>();
    println!("{:?}",v.iter());

}

    



fn main() {
    vec();
    hashmap();
    slice();
    hashset();
    float64();
    float32();
    file();
    string();
    tcp();
    command();
    thread();
    time();
    iterator();
}
