//variables hold primitive data or references to data
//variables are immutable by default
//rust is a blocked scoped language
//the convention for naming variables is _ not camelCase
pub fn run() {
    let name = "brad";
    let mut age = 24; // mut indicates that the variable can be reassigned stands for mutable
    println!("My name is {} and im {} years old", name, age);
    age = 25;
    println!("My name is {} and im {} years old", name, age);

    //define constants
    const ID: i32 = 4; //name them in all uppercases, you must explicitly define the data type
    println!("The id is: {}", ID);

    //assing multiple variables
    let (my_name, my_age) = ("brad", 15);
    println!("{} is {}", my_name, my_age);
}
