
// Variables hold primitive data or reference to data
// Variables are immutable by default - by default you cant reassign them
// Rust is a block-scoped language

pub fn run() {
    let name = "Tush";
    let mut age = 21;

    println!("My name is {} and I am {}", name, age);

    age = 22;

    println!("My name is {} and I am {}", name, age);

    //Define constants
    //You have to use uppercase and add a type while using const eg below bit32
    const ID: i32 = 001;
    println!("ID: {}", ID);

    //Assign multiple variables
    let ( my_name, my_age ) = ( "Tush", 22 );
    println!("{} is {}", my_name, my_age);

}