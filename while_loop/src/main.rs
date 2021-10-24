fn main() {
    while_loop();
}

fn while_loop(){
    let mut x = 1;
    while x < 1000
    {
        x *= 2;
        if x == 64{ continue; }
        println!("X = {}",x);
    }

    let mut y = 1;
    loop //while true
    {
        y *= 2;
        println!("Y = {}", y);

        if y == 1<<10 {break;} // 1x2^10
    }

}