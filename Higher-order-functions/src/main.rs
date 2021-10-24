fn is_even(x : u32) -> bool
{
    x % 2 == 0
}
fn greater_than(limit : u32) -> impl Fn(u32) -> bool
{
    move |y| y > limit;
}

fn main() {
    //Functions that take functions
    //f(g) {let x = g();}

    //functions that returns functions
    //generators
    // f() -> g

    /* Example : SUM of all even squares < 500*/
    let limit = 500;
    let mut sum = 0;
    //let above_limit = |y| y > limit;
    let above_limit = greater_than(limit);

    for i in 0..{
        let isq = i*i;

        // if isq > limit
        if above_limit(isq){
            break;
        }else if is_even(isq){
            sum += isq;
        }
    }
    println!("Loop sum = {}",sum);
}
