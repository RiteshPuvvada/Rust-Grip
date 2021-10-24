#![allow(ellipsis_inclusive_range_patterns)]
fn main() {
    pattern_match();
}

fn how(x:i32) -> &'static str{
    match x{
        0 => "NO",
        1 | 2 => "one or two",
        z @ 9...11 => "lots of",
        12 => "A dozen",
        _ if(x%2 == 0) => "Some",
        _ => "A few"
    }
}

fn pattern_match(){
    for x in 0..13
    {
        println!("{}: I have {} oranges",x, how(x))
    }

    let point = (3,7);
    match(point)
    {
        (0,0) => println!("origin"),
        (0,y) => println!("x axis, y = {}",y),
        (x,0) => println!("y axis, x = {}",x),
        //(x,y) => println!("x = {}, y = {}",x,y),
        (_,y) => println!("(?,{})",y)
    }
/* !!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!!! */
    enum Color{
        Red,
        Blue,
        Green,
        RgbColor(u8,u8,u8), //tuple
        CmykColor{cyan:u8, magenta:u8, yellow:u8, black:u8} //struct
    }

    let c:Color = Color::CmykColor{cyan:0, magenta:128, yellow:0, black:255};
    match c{
        Color::Red => println!("!RED"),
        Color::Blue => println!("!BLUE"),
        Color::Green => println!("!GREEN"),
        //_ => println!("Some other color"),
        Color::RgbColor(0,0,0)
        | Color::CmykColor{black:255,..} => println!("!BLACK"), //.. is we need to care only that black one 
        Color::RgbColor(r,g,b) => println!("RGB({},{},{})",r,g,b),
        _ => ()
        
    }
}
