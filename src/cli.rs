use std::env;
pub fn run() {
    let args: Vec<String> = env::args().collect();
    let name = "brad";
    let status = "100%";
    let command = args[1].clone();
    //println!("command: {:?}", command);

    if command == "hello" {
        println!("Hello {}, how are you", name);
    } else if command == "status" {
        println!("Status is {}", status);
    } else {
        println!("that is not a valid command");
    }
}
