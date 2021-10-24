fn main() {
    let country_code = 998;
    
    let country = match country_code{
        44 => "Uk",
        46 => "Sweden",
        76 => "London",
        //1..=1000 => "unknown",//The RangeToInclusive ..=end contains all values with x <= end. It cannot serve as an Iterator because it doesn't have a starting point.
        //1..41 => "unknown", it prints 1 - 40
        1...999 => "unknown", //it includes 1-999 
        // 1..9(gives us values from 1 to 8)
        // 1...9(gives us values from 1 to 9)
        _=>"Invalid"
    };
    println!("The country code with {} is {}",country,country_code);
}
