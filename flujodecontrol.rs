fn main(){
    let x = 10;
    if x < 10 {
        println!("x is less than 10");
    } else if x < 100 {
        println!("x is less than 100");
    } else {
        println!("x is greater than or equal to 100");
    }

    let x = 10;
    let menssage = if x < 10 { "x is less than 10" } else { "x is greater than or equal to 10" };

    println!("{}", menssage);
}

