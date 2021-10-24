fn main() {
    for_loop();
}

fn for_loop(){
    for x in 1..11
    {
        if x == 3{continue;}

        if x == 8 {break;}
        
        println!("X = {}",x);
    }

    for (pos,y) in (30..41).enumerate()
    {
        println!("{}: {}",pos,y);
    }
}
