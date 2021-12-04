fn main(){
    my_function(10,20);
    let x = get_10();
    println!("{}",x);
    // variables with same name
    let x = 10;
    let y = {
        let x = 3;
        x + 1
    };
    println!("value of x {}",x);
    println!("value of y {}",y);

}

fn my_function(x:i32, y:i32){
    println!("Value of x is : {}", x);
    println!("Value of y is : {}", y);
}

fn get_10() -> i32{
    println!("I am in get_10");    
    10 //return 10;
}