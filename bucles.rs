fn main(){
    loop {
        println!("Hello, world!");
        break;
    }

    let x = loop {
        break 10;
    };
    println!("The value of x is: {}", x);

    let mut counter = 0;
    let result = loop {
        counter += 1;
        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);

    //while
    counter = 10;
    while counter > 0 {
        println!("{}!", counter);
        counter -= 1;
    }
    println!("{}", counter);

    //for cycle
    let a = [10, 20, 30, 40, 50];
    
    for element in a.iter() {
        println!("the value is: {}", element);
    }
    for number in (1..4).rev() {
        println!("{}!", number);
    }

    


}