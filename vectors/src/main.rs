fn main() {
    vector();
}

fn vector(){
    let mut a = Vec::new();
    a.push(1);
    a.push(2);
    a.push(3);
    println!("a = {:?}",a);
    a.push(4);
    println!("a = {:?}",a);
    // usize isize

    let idx:usize = 0;
    a[idx] = 321;
    println!("a[0] = {}",a[idx]);
    println!("a[0] = {:?}",a);
    // option
    match a.get(3){
        Some(x) => println!("a[3] = {}", x),
        None => println!("Error, no such element")
    }
    // iteration
    for x in &a { println!("{}",x)}

    a.push(44);
    a.push(78);
    println!("{:?}",a);
     //some(77)
    let last_ele = a.pop(); // Option
    println!("last ele is {:?}, a = {:?}",last_ele,a);

    while let Some(x) = a.pop()
    {
        println!("{}",x);
    }
}
