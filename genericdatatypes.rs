use std::fmt::Debug;

struct Info<T> {
    name: T,
}

impl<T: Debug> Info<T> {
    fn show_name(&self) {
        println!("the value is {:?}", self.name);
    }
    fn show_other_value<U: Debug>(&self, x: U) {
        println!("the param value is {:?}", x);
    }
}

fn main(){

    let x: Info<String> = Info { 
        name: "John".to_string(),
    };

    println!("{}", x.name);

    let y: Info<i32> = Info {
        name: 30,
    };

    println!("{}", y.name);

    let z: Info<f64> = Info {
        name: 30.0,
    };

    println!("{}", z.name);


    any_param(190.0);

    let t1: Info<i32> = Info {
        name: 30,
    };
    t1.show_name();
    t1.show_other_value("Hola".to_string());
}

fn any_param<T: Debug>(x: T) {
    println!("{:?}", x);
}