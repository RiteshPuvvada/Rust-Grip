use std::mem;
fn main() {
    //u = unsigned (0 +) 0-255(range) ,  i = signed (0 -) -128-127(range)
    let a:u8 = 123; // 8bits or 1 byte
    println!("The value of A = {} ,size = {} bytes ", a,mem::size_of_val(&a));

    //a = 456; Cannot assign twice to immutable variable
    //To assign and change variable it should be mutable

    let mut b:i8 = 12;
    println!("The value of b = {}",b);
    b = 32;
    println!("The value of b = {}",b);
    
    let mut c:u32 = 123456789; //32-bit signed i32 4 bytes
    println!("C = {}, size = {} bytes ", c, mem::size_of_val(&c));
    c = 1235;
    println!("C = {}, size = {} bytes ", c, mem::size_of_val(&c));
    // i8 u8 i16 u16 i32 u32 i64 u64 i128 u128 
    /*
    u8 = 2^8 = 256 (0-255)
    i8 = -ve = 256 (-128 to 127 )
    u16 = 2^16 = 65536 (0-65535)
    i16 = -ve = 65536 (-32768 to 32767)
    u32 = 2^32 = 4294967296 (0 - 4294967295)
    i32 = -ve = 4294967296 (-2147483648 to 2147483647)
    u64 = 2^64 = 18446744073709551616 (0 - 18446744073709551615 )
    i64 = -ve = 18446744073709551616 (-9223372036854775808 to 9223372036854775807)
    u128 = 2^128 = 340282366920938463463374607431768211456 (0-340282366920938463463374607431768211455)
    i128 = -ve = 340282366920938463463374607431768211456 (-170141183460469231731687303715884105728 to 170141183460469231731687303715884105727)
    */ 


    //isize,usize
    let z:isize = 123;
    let size_of_z = mem::size_of_val(&z);
    println!("The value of Z = {} , takes up {} bytes, {}-bit os",z,size_of_z,size_of_z*8);

    let d:char = 'r';
    println!("C = {}, size = {} bytes ", d, mem::size_of_val(&d));

    let e:f32 = 2.5; // Double-precision, 8bytes or 64bit, f64
    println!("C = {}, size = {} bytes ", e, mem::size_of_val(&e));

    // true or false
    let g = false;
    println!("C = {}, size = {} bytes ", c, mem::size_of_val(&g));
    let f = 4>0;
    println!("The value of F = {}",f);
}
