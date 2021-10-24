fn main() {
    if_statement();
}

fn if_statement(){
    //control flow
    let temp = 35;
    if temp > 30{
        println!("Really hot outside")
    }
    else if temp < 10 {
        println!("Really cold outside")
    }
    else {
        println!("temperature is perfect");
    }
    let day = if temp > 20 {"Sunny"} else {"Cloudy"};
    println!("Today is {}",day);
    //other type
    println!("Is it {}",
    if temp > 20 {"Hot"} else if temp < 10{"Cold"} else {"ok"});
    //Nest
    println!("It is {}",
        if temp > 20 {
            if temp > 30 {"Very hot"} else {"Hot"}
        } else if temp < 10 {"Cold"} else {"Ok"});
}