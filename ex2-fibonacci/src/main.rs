use std::io;

fn main() {
    println!("Enter the number: ");
    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Failed to read line");

    let input: i32 = input.trim().parse().expect("Invalid input.");
    println!("Fibonacci of {input} is: {}", fibonacci(input));
}

fn fibonacci(input: i32) -> i32 {
    return if input <= 2 {
        1
    } else {
        fibonacci(input - 1) + fibonacci(input - 2)
    }
}
