//32-bits
#![allow(dead_code)]
#![allow(unused_variables)]
union IntOrFloat
{
    i : i32,
    f : f32
}
fn process(iof: IntOrFloat){
    unsafe{
        match iof{
            IntOrFloat {i:42} => {
                println!("Meaning of life value");
            }
            IntOrFloat { f } =>{
                println!("value = {}",f)
            }
        }
    }
}
fn main() {
    let mut iof = IntOrFloat{i : 123};
    iof.i = 234;
    let value = unsafe{iof.i};
    println!("iof.i = {}",value);
    process(IntOrFloat{f:42.0});
    process(IntOrFloat{i:42});
    process(IntOrFloat{i:2})
}