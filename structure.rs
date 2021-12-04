#[derive(Debug)]
struct User{
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool,
}
struct Color(i32, i32, i32);

struct Rectangle{
    width: u32,
    height: u32,
}
impl Rectangle{
    fn rectangle(width: u32, height: u32) -> Rectangle{
        Rectangle{
            width,
            height,
        }
    }
    fn area(&self) -> u32{
        self.width * self.height
    }
    fn can_hold(&self, other: &Rectangle) -> bool{
        self.width > other.width && self.height > other.height
    }
    fn square(size: u32) -> Rectangle{
        Rectangle{width: size, height: size}
    }
    fn compare(&self, other: &Rectangle) -> i32{
        if self.area() > other.area(){
            1
        }else if self.area() < other.area(){
            -1
        }else{
            0
        }
    }
    
}

fn main() {
    let  mut user1 = User{
        email: String::from("fabiogomez28@gmail.com"),
        username: String::from("fabiogomez28"),
        active: true,
        sign_in_count: 1
    };

    let user3 = User{
        email: String::from("hola@mundo.com"),
        username: String::from("user 3"),
        ..user1
    };

    println!("{:?}", user1);
    println!("{}", user1.email);
    println!("{:?}", user3);

    user1.sign_in_count = 2;

    println!("{}", user1.sign_in_count);

    let user2 = make_user(String::from("fabiogomez28@hotmail.com"), String::from("fabiogomez282"));
    println!("{:?}", user2);

    let black = Color(0, 0, 0);
    println!("{:?}", black.2);

    let width = 30;
    let height = 50;

    println!("The area of the rectangle is {} square pixels.", area(width, height));

    let rectangle = Rectangle {
        width: 30,
        height: 50,
    };

    let rectangle2 = Rectangle {
        width: 80,
        height: 50,
    };

    let rectangle3 = Rectangle::rectangle(200, 100);
    println!("Nuestro rectangulo es {:?}", rectangle3);

    println!("The area of the rectangle is {} square pixels.", area2(&rectangle));
    println!("The area of the rectangle is {} square pixels.", rectangle.area());
    println!("Compare areas of  two rectangles {}", rectangle.compare(&rectangle2));
}

fn make_user(email: String, username: String) -> User {
    User {
        email,
        username,
        active: true,
        sign_in_count: 1,
    }
}

fn area(width: u32, height: u32) -> u32 {
    width * height
}

fn area2(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.height
}
