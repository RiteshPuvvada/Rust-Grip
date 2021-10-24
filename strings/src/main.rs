fn main() {
    strings();
}

fn strings()
{
    //utf - 8
    let s:&'static str = "Hello there!"; //&str = string slice
    // s = "ABC" won't work
    // let h = s[0] won't work
    for c in s.chars(){
        println!("{}",c);
    }
    for c in s.chars().rev(){
        println!("{}",c);
    }
    if let Some(first_char) = s.chars().nth(0){
        println!("First letter is {}", first_char);
    }

    //String
    //Heap alloc

    let mut letters = String::new();
    let mut a = 'a' as u8;
    while a<= ('z' as u8){
        letters.push(a as char);
        letters.push_str(",");
        a += 1;
    }
    println!("{}",letters);
    // &str <> String
    let u:&str = &letters;

    //concatenation 
    //String + str

    //let z = letters + &letters;
    let mut abc = "Hello world".to_string();
    abc.remove(0);
    abc.push_str("!!!!!!!!!!");
    println!("{}",abc.replace("ello", "goodbye"));
}
