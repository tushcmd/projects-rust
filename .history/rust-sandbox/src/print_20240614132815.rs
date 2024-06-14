pub fn run() {
    // Print "to console
    println!("Hello from the print.rs file");

    // Basic Formatting
    println!("Number: {}", 1);

    println!("{} is from {}", "Tush", "Mars");

    //Positional Arguments
    println!("{0} is from {1} and {0} likes to {2} in Rust", "Tush", "Mars", "code");

    //Named Arguments
    println!("{name} likes to play {activity}", name = "Gyan", activity = "football");

    // Placeholder Traits
    println!("Binary: {:b} Hex: {:x} Octal: {:o}", 10, 10, 10);

    // Placeholder for debug traits - print array
    println!("{:?}", (15, true, "Tush"));
}
