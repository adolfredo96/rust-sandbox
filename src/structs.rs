//used to create custom data types

//Tradicional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}
//Tuple struct
struct Colors(u8, u8, u8);

//Person struct
struct Person {
    first_name: String,
    last_name: String,
}
impl Person {
    fn new(first: &str, last: &str) -> Person {
        return Person {
            first_name: first.to_string(),
            last_name: last.to_string(),
        };
    }

    fn get_full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    //name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    c.red = 200;
    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut d = Colors(255, 0, 0);

    println!("Color: {} {} {}", d.0, d.1, d.2);

    let mut p = Person::new("john", "doe");
    println!("Firstname: {} Lastname: {}", p.first_name, p.last_name);
    println!("Fullname: {}", p.get_full_name());
    p.set_last_name("Malave");
    println!("Fullname: {}", p.get_full_name());
    println!("Person Tuple: {:?}", p.to_tuple());
}
