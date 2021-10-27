//vectors are resizable arrays
pub fn run() {
    let mut numbers: Vec<i32> = vec![1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    //get single val
    println!("single value: {:?}", numbers[0]);

    //reasign a value
    numbers[2] = 20;

    //add onto vector
    numbers.push(6);
    numbers.push(7);

    //remove last value
    numbers.pop();

    //get vector lenght
    println!("lenght: {}", numbers.len());

    //vectors are stacked allocated
    println!("vector ocuppies {} bytes", std::mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    //loop through vector values
    for x in numbers.iter() {
        println!("Number: {}", x)
    }

    //loop and mutate values
    for x in numbers.iter_mut() {
        *x *= 2;
    }

    println!("{:?}", numbers);
}
