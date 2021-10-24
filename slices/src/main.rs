fn main() {
    Slices();
}

fn Slices(){
    let mut data = [1,2,3,4,5,6,4];
    //use_slice(&mut data[1..4]); //index 1 to 3 .. means not taking the value of index 4
    use_slice(&mut data); //Entire array into slice
    println!("{:?}",data);
}
fn use_slice(slice :&mut[i32]){
    println!("First Element = {}, len = {} ",slice[0],slice.len());
    slice[0] = 4321;
}