#![allow(non_snake_case)]
#![allow(unused_imports)]
#![allow(unused_variables)]
#![allow(dead_code)]

fn main() {
    methods();
    functions(5);
    let mut z = 1;
    increase(&mut z);
    println!("z = {}",z);
    let a = 15;
    let b = 18;
    let p = product(a,b);
    println!("{} * {} = {}",a,b,p);

}

fn functions(x:i32){
    println!("Value of x = {}",x)
}

fn increase(A:&mut i32){
    *A += 1;
}

fn product(X:i32, Y:i32) -> i32{
    X * Y
}

/* Methods */

struct Point{
    x : f64, y : f64
}

struct Line{
    start : Point, end : Point
}

impl Line
{
    fn len(&self) -> f64
    {
        let dx = self.start.x - self.end.x;
        let dy = self.start.y - self.end.y;
        (dx*dx+dy*dy).sqrt()
    }
}

fn methods(){
    let p = Point{x:3.0,y:4.0};
    let p2 = Point{x:5.0,y:10.0};
    let myLine = Line{start:p, end:p2};

    println!("Length = {}",myLine.len());
}