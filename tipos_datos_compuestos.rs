fn main(){
    let tupla = (1,2.3,"Hola");
    println!("{:?}",tupla);
    println!("{}",tupla.0);
    
    let tupla2: (i32, f32, &str) = (1,2.3,"Hola");
    println!("{:?}",tupla2);
    
    let (x,y,z) = tupla2;
    println!("{}",x);
    println!("{}",y);
    println!("{}",z);

    let h = tupla2.2;
    println!("{}",h);

    //mutable tuple
    let mut tupla3 = (1,2.3,"Hola");
    tupla3.0 = 2;   
    println!("{:?}",tupla3);

    //array
    let array = [1,2,3,4,5];
    println!("{:?}",array);

    //array with months names
    let months = ["Enero","Febrero","Marzo","Abril","Mayo","Junio","Julio","Agosto","Septiembre","Octubre","Noviembre","Diciembre"];

    //array
    let array2: [i32; 5] = [1,2,3,4,5];
    println!("{:?}",array2);

     //array with zero
    let array3: [i32; 5] = [0; 5];
    println!("{:?}",array3);

     //array with five strings
    let array4: [&str; 5] = ["Hola","Mundo","Hola","Mundo","Hola"];
    println!("{:?}",array4);

    //array with five caracteres
    let mut array5: [char; 5] = ['H','o','l','a','M'];
    println!("{:?}",array5);

    //print value of array
    println!("{}",array5[0]);

    //change second value of array
    array5[1] = 'Z';
    println!("{:?}",array5);

    //out range
    println!("{}",array5[5]);

}

