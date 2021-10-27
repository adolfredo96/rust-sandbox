//fixed list where elements are of the same type
use std::mem;
pub fn run() {
    let mut numbers: [i32; 5] = [1, 2, 3, 4, 5];

    println!("{:?}", numbers);

    //get single val
    println!("single value: {:?}", numbers[0]);

    //reasign a value
    numbers[2] = 20;

    //get array lenght
    println!("lenght: {}", numbers.len());

    //arrays are stacked allocated
    println!("Arrays ocuppies {} bytes", mem::size_of_val(&numbers));

    //Get slice
    let slice: &[i32] = &numbers[0..2];
    println!("Slice: {:?}", slice);

    println!("{:?}", numbers);
}
