#[derive(Debug)]
enum Gender{
    Male,
    Female

}
enum Speed {
    Slow = 10,
    Half = 20,
    Fast = 50
}
enum Dificult{
    Easy = 1,
    Medium,
    Hard
}
#[derive(Debug)]
enum EnumWithLargeName {
    Sum,
    Sub

}
type Operations = EnumWithLargeName;

#[derive(Debug)]
enum Value{
    Number(f64),
    Str(String),
    Bool(bool,bool),
    Struct{
        x:i32,
        y:i32
    }
}
enum Option<T>{
    Some(T),
    None
}


#[derive(Debug)]
struct Person { 
    name: String,
    gender: Gender
}

fn main()
{
    let man = Gender::Male;
    let woman = Gender::Female;

    let p1 = Person {
        name: String::from("Juan"),
        gender: Gender::Male
    };

    let p2 = Person {
        name: String::from("Ana"),
        gender: Gender::Female
    };

    let slow = Speed::Slow;
    let slow = slow as u32;

    let medium = Dificult::Medium;
    let medium = medium as u32;

    let sum = Operations::Sum;

    let n = Value::Number(2.3);
    let string = Value::Str(String::from("Hello"));
    let bool = Value::Bool(true,false);
    let struct_ = Value::Struct{x:1,y:2};

    let some = Option::Some(1);
    let none = Option::None;
    let some2 = Option::Some("Hello");
    let some2 = Option::Some(String::from("Hello"));
    let some3 = Option::Some(true);

    println!("n: {:?} string: {:?} bool: {:?} struct_: {:?}  ", n, string, bool, struct_);


    println!("{:?}", man);
    println!("{:?}", woman);

    println!("{:?}", p1);
    println!("{:?}", p2);

    println!("{:?}", slow);
    println!("{:?}", medium);
    println!("{:?}", sum);
}