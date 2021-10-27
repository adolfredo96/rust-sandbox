pub fn run() {
    greeting("hello", "adolfredo");
    println!("result: {}", add(2, 2));

    //bind function result to variables
    let result = add(5, 5);

    println!("result: {}", result);

    //closure
    let n3: i32 = 10;
    let add_nums = |n1: i32, n2: i32| n1 + n2 + n3;
    println!("C sum: {}", add_nums(4, 4));
}

fn greeting(greet: &str, name: &str) {
    println!("{} {} to meet you", greet, name)
}

fn add(n1: i32, n2: i32) -> i32 {
    return n1 + n2;
}
