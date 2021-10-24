use std::mem;

//Global variable
const Meaning_of_Ritesh:u8 = 23; //no fixed address

static mut Z:i32 = -124; 

fn main (){
    //It should be
    operators();
    scope_and_shadowing();
    println!("Global variable {}",Meaning_of_Ritesh);
    unsafe{
    println!("Global variable {}",Z); //Its a promise that it's my mistake to do this static does not be mutable to avoid address flaws it have no address
    }
}

fn operators() {
    let mut a = 2+3*4; // +-*
    println!("The value of A = {}",a);
    a += 1; // -- +
    println!("The value of A after = {}",a);
    // -= += *= /= %=
    println!("Remainder of {} / {} = {}",a,3,(a%3));
    

    let a_cube = i32::pow(a,3);
    println!("{} cubed is {}",a,a_cube);

    let b = 2.5;
    let b_cubed = f64::powi(b,3);
    let b_to_pt = f64::powf(b,std::f64::consts::PI);
    println!("{} is cubed = {} and {}^pi = {}",b, b_cubed, b, b_to_pt);


    //bitwise

    let c = 1 | 2; // | = OR, & = AND, ^ = XOR, ! = NOR
    // 01 OR 10 = 11 == 3_10
    println!("1 | 2 = {} and bytes = {}",c ,mem::size_of_val(&c));

    let two_to_10 = 1 << 10; // multiplying x with 2^y
    println!("2^10 = {}" ,two_to_10);
    let right_shift = 5 >> 1; // dividing x with 2^y
    println!("{}",right_shift);

    //Logical
    let pi_less_4 = std::f64::consts::PI < 4.0; //true
    println!("Boolean = {}",pi_less_4);
    //> < <= >= ==
    let x = 5;
    let x_is_5 = x == 5;
    println!("Boolean = {}",x_is_5);
    
}


//Scope and Shadowing 

fn scope_and_shadowing() {
    let var = 123;
    println!("Outside {}",var );
    {
        let var = 12; //shadow
        println!("Inside {}",var );
        let var1 = 124;
        println!("{}",var1);
    }
}