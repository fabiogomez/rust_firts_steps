use std::collections::HashMap;

fn main(){
    let mut map = HashMap::new();
    map.insert(1, "a");
    map.insert(2, "b");
    map.insert(3, "c");
    println!("{:?}", map);

    let mut map = HashMap::new();
    map.insert("rust", 10);
    println!("{:?}", map.get("rust"));
    map.insert("javascript", 30);

    //add value if not exists
    map.entry("rust").or_insert(20);

    for (key, value) in &map {
        println!("{}: {}", key, value);
    }

    let key = "rust";
    let value = 10;
    let mut map = HashMap::new();
    map.insert(key, value);
    println!("{:?}", map);

    println!("{} {}", key, value);
}