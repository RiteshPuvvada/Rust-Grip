#![allow(dead_code)]
#![allow(unused_variables)]
#![allow(unused_imports)]
#![allow(unreachable_patterns)]

use std::mem;

fn main() {
    enumeration();
}


enum Color{
    Red,
    Blue,
    Green,
    RgbColor(u8,u8,u8), //tuple
    CmykColor{cyan:u8, magenta:u8, yellow:u8, black:u8} //struct
}

fn enumeration(){
    let c:Color = Color::CmykColor{cyan:0, magenta:128, yellow:0, black:255};
    match c{
        Color::Red => println!("!RED"),
        Color::Blue => println!("!BLUE"),
        Color::Green => println!("!GREEN"),
        //_ => println!("Some other color"),
        Color::RgbColor(0,0,0)
        | Color::CmykColor{cyan:_, magenta:_, yellow:_, black:255} => println!("!BLACK"),
        Color::RgbColor(r,g,b) => println!("RGB({},{},{})",r,g,b),
        _ => ()
        
    }
}