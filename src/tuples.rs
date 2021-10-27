//tuples group together values of different types
//max 12 elements
pub fn learn() {
    let person: (&str, i32, i8) = ("hello", 4, 5);

    println!("{} next {} next {}", person.0, person.1, person.2);
}
