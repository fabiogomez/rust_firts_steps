fn main(){
    let mut s = String::from("hello");
    let s1 = "hello";
    println!("{}", s);
    println!("{}", s1);

    s.push_str(", world!");
    println!("{}", s);

    println!("len {}", s.len());
    println!("len {}", s1.len());

    println!("capacity {}", s.capacity());
    saludar(&s);
    saludar2(s);
    
}

fn saludar(s: &str){
    println!("{}", s);
}
fn saludar2(s: String){
    println!("{}", s);
}