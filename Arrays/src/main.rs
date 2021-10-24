use std::mem;
fn main() {
    Arrays();
    multiArray()
}

fn Arrays(){
    let mut a:[i32;5] = [1,2,3,4,5];
    println!("A has {} elements, first is {}",a.len(),a[0]);
    a[0] = 321;
    println!("A has {} elements, first is {}",a.len(),a[0]);

    println!("{:?}",a);

    if a != [1,2,3,4,5]{
        println!("Does not match")
    }

    //other way
    let b = [1u16; 10]; //b.len() == 10
    for i in 0..b.len(){
        println!("{}",b[i]);
    }
    println!("B to took this many {} bytes",mem::size_of_val(&b));
}

fn multiArray(){
    let mtx : [[f32;3];2] = [
        [1.0,0.0,0.0],
        [0.0,2.0,0.0]
    ];
    println!("MTX = {:?} ",mtx);

    //Diagonal values 
    for i in 0..mtx.len(){
        for j in 0..mtx[i].len(){
            if i == j{
                println!("mtx[{}][{}] = {}",i,j,mtx[i][j]);
            }
        }
    }
}
