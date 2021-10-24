fn main() {
    tuples();
}

fn tuples(){
    let x = 5;
    let y = 3;
    let sp = sum_and_product(x,y);
    println!("Sp = {:?}",sp);
    println!("{0} + {1} = {2}, {0} * {1} = {3}", x, y, sp.0, sp.1);
    
    // destructuring 
    let (a,b) = sp;
    println!("a = {}, b = {}", a, b);

    //tuple of tuple
    let sp2 = sum_and_product(4,7);
    let combined = (sp , sp2);
    println!("{:?}",combined);
    println!("last element = {}", (combined.1).1);

    let ((c,d),(e,f)) = combined;
    println!("{:?}",combined);
    
}

fn sum_and_product(x:i32, y:i32) -> (i32, i32){
    (x+y, x*y)
}