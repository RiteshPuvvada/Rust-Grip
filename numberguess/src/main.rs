#![allow(dead_code)]
#![allow(unused_imports)]
#![allow(unused_must_use)]

use rand::Rng;
use std::io::stdin;

fn main() {
    let number = rand::thread_rng().gen_range(1..101);
    loop {
        println!("Enter your guess number : ");
        let mut buffer = String::new();
        match stdin().read_line(&mut buffer){
            Ok(_) => {
                let parsed = buffer.trim_end().parse::<i64>();
                match parsed{
                    Ok(guess) => {
                        if guess < 1 || guess > 101{
                            println!("Your guess is out of range");
                        }else if guess < number{
                            println!("Your guess is too low :(");
                        }else if guess > number{
                            println!("Your guess is to high :(");
                        }else {
                            println!("Correct !!! :)");
                            break;
                        }
                    },Err(e) => {
                        println!("Could not read your input. {}. Try again!",e);
                    }
                }
            }, Err(_) => continue,
        }
    }
}
