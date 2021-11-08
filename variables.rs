fn main(){
    let mut x = 10;
    println!("{}", x);
    x=20;
    println!("{}", x);

    //constantes
    const MAX_POINTS: u32 = 100_000;
    println!("{}", MAX_POINTS);

    //sombreado de variables
    let x = 10;
    println!("{}", x);
    let x = x * 2;

    let x = "Hello World";
    println!("{}", x);
    let x = x.len();
    println!("{}", x);

    let mut x = "Hello world";
    println!("{}", x);
    x = "Cambio a un size mayor de string";
    println!("{}", x);

    // let mut x = "Hello world";
    // println!("{}", x);
    // x = x.len();
    // println!("{}", x);
}