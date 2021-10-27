//String = growable, heap-allocated data structure - use it when you need to modify or own string data

pub fn run() {
    let mut name = String::from("Adolf");

    //get length
    println!("Length: {}", name.len());

    //push a char
    name.push('r');

    //push a string
    name.push_str(" edo");

    //capacity in bytes
    println!("Capacity: {}", name.capacity());

    //check if its empty
    println!("Is empty: {}", name.is_empty());

    //contains
    println!("Contains edolf {}", name.contains("edolf"));

    //Replace
    println!("Replace: {}", name.replace("Adolf", "ADOLF"));

    //loop through string by whitespace
    for name in name.split_whitespace() {
        println!("{}", name);
    }

    //Create string with capacity
    let mut s = String::with_capacity(10);
    s.push('a');
    s.push('b');

    //assertion testing
    assert_eq!(2, s.len()); //only shows error if it fails, if it passes it wont show anything
    assert_eq!(10, s.capacity());
    println!("{}", s);
}
