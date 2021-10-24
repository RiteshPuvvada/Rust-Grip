#![allow(unused_variables)]
#![allow(dead_code)]
fn main() {
    generics();
}

//option T
struct Point<T>{
    x : T,
    y : T
}
struct line<T>{
    start : Point<T>,
    end : Point<T>
}
fn generics(){
    /*we can write also
    let a:Point<i32,u32> = Point{ x:0, y:0 };
    let b:Point<f32,i32> = Point{ x:1.4, y:2.4 };
    */
    let a:Point<f64> = Point{ x:1.0, y:0.0 };
    let b = Point{ x:1.4, y:2.4 };
    
    let myline = line{start : a, end : b};
    

}
