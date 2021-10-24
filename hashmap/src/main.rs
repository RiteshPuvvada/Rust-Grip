use std::collections::HashMap;

fn main() {
    let mut shape = HashMap::new();
    shape.insert(String::from("Triangle"),3); // key = triangle and value = 3
    shape.insert(String::from("Square"),4);

    println!("A square has {} sides", shape["Square".into()]);

    for (key,value) in &shape {
        println!("key = {}, value = {}",key,value);
    }

    shape.insert("Square".into(),5);
    println!("{:?}",shape);

    shape.entry("Circle".into()).or_insert(5); //It will check or else is add
    {
        let actual = shape
            .entry("Circle".into()).or_insert(2);
        *actual = 0;
        
    }
    println!("{:?}",shape);
}
