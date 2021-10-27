pub fn run() {
    //print to console
    println!("hello from the print.rs file!");
    //basic formatting
    println!("{} is from {}", "Adolfredo", "Venezuela");
    //positional parameters
    println!(
        "{0} is from {1} and {0} likes to {2}",
        "Adolfredo", "Puerto ordaz", "Guitar"
    );
    //named arguments
    println!(
        "Adolfredo likes to play {name} with a {friend}",
        name = "warzone",
        friend = "fulano"
    );
    //placeholder traits
    println!("Binary: {:b} Hex: {:x} Octal {:o}", 10, 10, 10);

    //placeholder for debug trait
    println!("{:?}", (12, "hello", true));

    //basic math
    println!("10+10={}", 10 + 10);
}
