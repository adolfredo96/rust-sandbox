pub fn run() {
    //default is i32
    let x = 20;
    //default is f64
    let y = 30.5;
    //add explicit type
    let z: i64 = 45454545445;

    //find max size
    println!("Max i32: {}", std::i32::MAX);
    println!("Max i64: {}", std::i64::MAX);

    //boolean
    let is_active = true;

    //get boolean from expression
    let is_greater = 10 < 5;

    //chars
    let a1 = 'a'; //just one character must be sorrounded by ''
    let face = '\u{1F600}'; // can assign unicode, like an emoji
    println!("{:?}", (x, y, z, is_active, is_greater, a1, face));
}
