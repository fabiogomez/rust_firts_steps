struct Circle {
    radius: f64,
}
trait HasArea {
    fn area(&self) -> f64;
}

struct Square {
    side: f64,
}

impl HasArea for Circle {
    fn area(&self) -> f64 {
        std::f64::consts::PI * (self.radius * self.radius)
    }
    
}

impl HasArea for Square {
    fn area(&self) -> f64 {
        self.side * self.side
    }
}
fn main() {
    let c = Circle { radius: 2.0 };
    println!("{}", c.area());

    let s = Square { side: 3.0 };
    println!("{}", s.area());
}