use std::io;

fn main() {
    // variables
    let mut x = 5;
    println!("The value of x is: {x}");
    x = 6;
    println!("The value of x is: {x}");

    // constants
    const THREE_HOURS_IN_SECONDS: u32 = 60 * 60 * 3;
    println!("{THREE_HOURS_IN_SECONDS}");

    // shadowing
    let y = 5;
    let y = y + 1;
    {
        let y = y * 2;
        println!("The value of y in the inner loop is: {y}");
    }
    println!("The value of y is {y}");

    // floating point types
    let _x = 2.0;
    let _y: f32 = 3.0;

    // boolean type
    let _t = true;
    let _f: bool = false;

    // character type
    let _c = 'z';
    let _z: char = 'Z';
    let _heart_eyed_cat = '😻';

    // typle
    let tup: (i32, f64, u8) = (500, 6.4, 1);
    let (_x, y, _z) = tup;
    println!("The value of y is: {y}");
    println!("The first value in tup is: {}", tup.0);

    // array
    let _a: [i32; 5] = [1, 2, 3, 4, 5];
    let a = [3; 5];
    let _first = a[0];
    let _second = a[1];

    // out of index at run time
    let a = [1, 2, 3, 4, 5];
    println!("Please enter an array index.");
    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Failed to read line.");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}
