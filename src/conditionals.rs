pub fn run() {
    let age = 18;
    let check_id = true;
    let knows_person_of_age = true;

    if age >= 21 && check_id || knows_person_of_age {
        println!("bartender what would you like to drink");
    } else if age < 21 && check_id {
        println!("sorry cant");
    } else {
        println!("i need to see your id");
    }

    let is_of_age = if age >= 21 { true } else { false };

    println!("Is of age: {}", is_of_age);
}
