#![allow(dead_code)]
use std::mem;

fn main() {
    stack_and_heap();
}

struct point{
    x: f64,y: f64 //f64 = 8bytes
}
fn origin() -> point{
    point{x:0.0, y:0.0}
}
pub fn stack_and_heap(){
    let p1 = origin();
    let p2 = Box::new(origin());

    println!("P1 takes {} bytes",mem::size_of_val(&p1)); //16bytes
    println!("P2 takes {} bytes",mem::size_of_val(&p2)); //8bytes

    let p3 =*p2;//follow the box actually leaves 
    println!("P3 {}",p3.x);//relocating back to the stack
}