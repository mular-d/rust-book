fn main() {
    let _s = "hello"; // immutable string literal
    let mut s1 = String::from("hello"); // mutable string

    s1.push_str(", world!"); // push_str() appends a literal to a string
    println!("{}", s1); // This will print `hello, world!`

    // let s2 = s1; // s1 moved
    // println!("{}", s1); // This will fail to compile`

    let s2 = s1.clone(); // s1 cloned
    println!("s1 = {}, s2 = {}", s1, s2); // This will print the values

    let s = String::from("hello");
    takes_ownership(s);

    // println!("Value of s: {}", s); // This will fail to compile

    let x = 5;
    makes_copy(x);
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}
