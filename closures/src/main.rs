#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]

use std::mem;

fn main() {
    closures();
}

fn say_hello(){println!("hello")}

fn closures(){
    let sh = say_hello;
    sh();
    let plus_one = |x:i32| -> i32 {x + 1};
    let a = 3;
    println!("{} + 1 = {}",a,plus_one(a));
    
    let plus_two = |x| {
        let mut z = x;
        z += 2;
        z
    };
    println!("{} + 2 = {}",3,plus_two(3));

    // T : by value
    // T&
    // &mut &
    let plus_three = |x:&mut i32| *x += 3;
    let mut f = 12;
    plus_three(&mut f);
    println!("f = {}",f);



}