enum TypeContact {
    Email(String),
    Phone(u64),
    Telegram(String),
}
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
fn main(){

    let x = 1;
    match x {
        1 => println!("one"),
        2 => println!("two"),
        3 => println!("three"),
        _ => println!("anything"),
    }
    use_match(0);
    use_match(1);
    use_match(6);
    use_match(3);
    use_match(25);

    let boolean = true;

    let menssage = match boolean {
        false => "false",
        true => "true",
    };
    println!("{} -> {}", boolean, menssage);   
    show_type_contact(TypeContact::Email(String::from("Hola@hola.com")));

    let color1 = Color {
        red: 255,
        green: 0,
        blue: 0
    };
    clasified_color(color1);

}
fn show_type_contact(contact: TypeContact) {
    match contact {
        TypeContact::Email(email) => println!("Email: {}", email),
        TypeContact::Phone(phone) => println!("Phone: {}", phone),
        TypeContact::Telegram(telegram) => println!("Telegram: {}", telegram),
    }
}
fn use_match(x: i32){
    match x {
        0 => {
            let x = 10;  
            println!("{}", x + 10);
        },
        1 => println!("one"),
        2 | 4 | 6 | 8 => println!("pair"),
        3 | 5 | 7 | 9 => println!("triple"),
        20..=30 => println!("twenty to thirty"),
        _ => println!("anything"),
    }
}
fn clasified_color(c: Color) {
    match c {
        Color { red: 0, green: 0, blue: 0 } => println!("black"),
        Color { red: 255, green: 255, blue: 255 } => println!("withe"),
        Color { red: _, green: 0, blue: 0 } => println!("red"),
        Color { red: _, green: _, blue: _ } => println!("any color"),
    }
}