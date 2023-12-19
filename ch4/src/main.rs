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

    // References and Borrowing
    let s1 = String::from("hello");
    let len = calculate_length(&s1);

    println!("The length of '{}' is {}.", s1, len); // This is valid

    // let s = String::from("hello");
    // change(s); // This will fail to compile

    let mut s = String::from("hello");
    change(&mut s); // This is valid

    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // let r2 = &mut s; // cannot borrow mutable reference more than one
    // println!("{}, {}", r1, r2)

    // let mut s = String::from("hello");
    // let r1 = &s;
    // let r2 = &s;
    // let r3 = &mut s; // cannot borrow as mutable as it is borrowed as immutable already
    // println!("{}, {}, and {}", r1, r2, r3);

    let mut s = String::from("hello");
    let r1 = &s;
    let r2 = &s;
    println!("{} and {}", r1, r2);
    let r3 = &mut s;
    println!("{}", r3); // This works fine because the last usage of the immutable occurs before the mutable

    // let reference_to_nothing = dangle();

    let mut s = String::from("hello world");
    let word = first_word(&s); // word will get the value of 5
    s.clear(); // this empties the String, making it equal to "" word still has the value 5 here, but it is invalid!

    let mut s = String::from("hello world");
    let word = first_word_slice_type(&s);
    // s.clear(); // error!
    println!("the first word is: {}", word);
}

// fn dangle() -> &String { // make the return type `String` to fix
//     let s = String::from("hello");

//     &s // and return the string `s` directly
// }

fn change(some_string: &mut String) {
    some_string.push_str(", world!");
}

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn takes_ownership(some_string: String) {
    println!("{}", some_string);
}

fn makes_copy(some_integer: i32) {
    println!("{}", some_integer);
}

fn first_word(s: &String) -> usize {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return i;
        }
    }

    s.len()
}

fn first_word_slice_type(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
       if item == b' ' {
           return &s[0..i];
       }
    }

    &s[..]
}