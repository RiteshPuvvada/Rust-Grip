#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use std::thread;
use std::time;

fn main() {
    let name = "Ritesh";
    let ntg = format!("hi, I'm {}, nice to meet you",name);
    println!("{}",ntg);

    let hello = "Hello";
    let rust = "Rust";
    let hello_rust = format!("{},{}!",hello,rust);
    println!("{}",hello_rust);

    let run = "Run";
    let forest = "Forest";
    let rfr = format!("{0}, {1}, {0}!",run,forest);
    println!("{}",rfr);

    let info = format!(
        "my name is {last}. {first} {last}.",
        first = "James",last = "bond");
        println!("{}",info);

    let mixed = format!(
        "{1} {} {0} {} {data}",
        "Alpha",
        "Beta",
        data = "Delta"
    );
    println!("{}",mixed)
}
