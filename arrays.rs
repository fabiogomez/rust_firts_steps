#[derive(Debug)]
enum DiefrentDataTypes {
    Int(i32),
    Float(f64),
    Text(String),
}
fn main(){
    let mut array: Vec<i32> = Vec::new();
    array.push(1);
    array.push(1000);
    println!("{:?}", array);
    array.pop(); 
    println!("{:?}", array);

    let mut vector = vec![1, 2, 3];
    vector.push(4);
    vector.push(5);
    println!("{:?}", vector);

    let mut vector = vec![0;100];
    println!("{:?}", vector);

    let mut vector = vec![1, 2, 3, 4, 5];
    let element = &vector[2];
    let element2 = &vector[2..4];
    println!("{:p} {:p}", element, element2);
    println!("{:?} {:?}", element, element2);


    /* let mut vector = vec![1, 2, 3, 4, 5];
    let element = &vector[4];
    vector.push(5);
    println!("{:?} ", element); */

    let mut vector = vec![1, 2, 3, 4, 5];
    let element = vector.get(4);
    let element2 = vector.get(1000);
    println!("{:?} ", element);

    for i in &vector {
        println!("a reference {}", i);
    }

    for i in &mut vector {
        *i += 10;
        println!("a mutable refrence {}", i);
    }

    println!("{:?}", vector);

    for i in vector{
        println!("a value {}", i);
    }
    // vector don't exists after this line

    let vector = vec![
        DiefrentDataTypes::Int(1),
        DiefrentDataTypes::Float(2.0),
        DiefrentDataTypes::Text(String::from("Hello")),
    ];
    
    println!("{:?}", vector);
}