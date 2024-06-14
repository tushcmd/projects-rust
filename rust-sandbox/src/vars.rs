
// Variables hold primitive data or reference to data
// Variables are immutable by default - by default you cant reassign them
// Rust is a block-scoped language

pub fn run() {
    let name = "Tush";
    let mut age = 21;

    println!("My name is {} and I am {}", name, age);

    age = 22;

    println!("My name is {} and I am {}", name, age);

}