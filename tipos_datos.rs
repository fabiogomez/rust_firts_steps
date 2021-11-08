fn main(){
    let x: u32 = 10; //data type unsigned 32 bits, may be u62, u128, etc.
    println!("{}", x);

    let x: u32 = "10".parse().expect("No es un numero"); // parse string to u32, if not possible, return error. They need to be the same type
    println!("{}", x);

    println!("{}",i8::MIN); // -128
    println!("{}",i8::MAX); // 127
    println!("{}",i16::MIN); // -32768
    println!("{}",i16::MAX); // 32767
    println!("{}",i32::MIN); // -2147483648
    println!("{}",i32::MAX); // 2147483647
    println!("{}",i64::MIN); // -9223372036854775808
    println!("{}",i64::MAX); // 9223372036854775807

    // example float in rust
    let x: f32 = 3.14;
    let y: f32 = 5.14;
    println!("{}", x + y);

    let x = true;
    println!("{}", x);
    
    // single quotes for one caracteres
    let x = 'a';
    println!("{}", x);

    // double quotation marks for string
    let x = "Casa";
    println!("{}", x);
    


}