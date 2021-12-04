fn main() {
    let mut sub = String::from("Suscribete");
    let long = calculate_length(&mut sub);

    println!("The length of '{}' is {}.", sub, long);

    let mut s = String::from("hello");

    let r1 = &s;
    let r2 = &s;
    println!("{}, {}", r1, r2);

    let r3 = &mut s;
    println!("{}", r3);

    let null_reference = danger();


}

fn calculate_length(s: &mut String) -> usize {
    s.push_str(" y dale like.");
    s.len()
    
}
//dont return a reference to a local variable
fn danger() -> /*&String*/String {
    let s = String::from("hello");
    /*&s*/s
}
