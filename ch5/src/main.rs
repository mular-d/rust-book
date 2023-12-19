struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let mut user1 = User {
        active: true,
        username: String::from("someusername123"),
        email: String::from("someone@example.com"),
        sign_in_count: 1,
    };

    user1.username = String::from("anotherusername123");
    println!("user1: {}", user1.username);

    // creating instances from function returns
    let user2 = build_user(
        String::from("something@example.com"),
        String::from("something"),
    );
    println!("user2: {}", user2.username);

    // creating instances from other instances
    let user3 = User {
        email: String::from("another@example.com"),
        ..user2
    };
    // println!("user2: {}", user2.username); // this gives error because user2.username is moved
    println!("user3: {}", user3.username);

    // struct tuples
    struct Color(i32, i32, i32);
    struct Point(i32, i32, i32);

    let _black = Color(0, 0, 0);
    let _origin = Point(0, 0, 0);

    struct AlwaysEqual; // unit-like structs
    let _subject = AlwaysEqual;
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
