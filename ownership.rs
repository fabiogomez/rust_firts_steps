fn main(){
    let hello = String::from("hello");
    saludar(hello);
    //x is not valid here, because it was declared after the variable declaration
    let x = 5;

    let h = 10;
    let y = h;

    println!("{}, {}", h, y);
    println!("{:p}, {:p}", &h, &y);

    let x = String::from("Hello world");
    let y = x.clone();

    println!("{}, {}", x, y);
    println!("{:p}, {:p}", &x, &y);

    let j = 10;
    let k = 20;

    sumar(j, k);
    println!("{}, {}", j, k);

    let saludo = recive_hello();
    println!("{}", saludo);



}//x is not valid, because it is out of scope

fn saludar(nombre: String){
    println!("Hola {}", nombre);
}
fn sumar(j: i32, k: i32){
    println!("{}", j + k);
}

fn recive_hello() -> String{
    let saludar = String::from("hello");
    saludar
}