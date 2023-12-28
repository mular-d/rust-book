use std::collections::HashMap;

pub fn hm_examples() {
    println!("===============================HashMap===============================");

    let mut scores = HashMap::new();
    scores.insert(String::from("Blue"), 10);
    scores.insert(String::from("Yellow"), 50);

    for (key, value) in &scores {
        println!("{key}: {value}")
    }

    let team_name = String::from("Blue");
    let _score = scores.get(&team_name).copied().unwrap_or(0);

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(field_name, field_value);
    // This gives compiler error due to the values are moved
    // println!("{field_name}: {field_value}");

    let mut scores = HashMap::new();

    // This overwrite the value
    scores.insert(String::from("Blue"), 10);
    // scores.insert(String::from("Blue"), 50);

    println!("{:?}", scores);

    // Adding a key and value if a key isn't present
    scores.insert(String::from("Blue"), 10);
    scores.entry(String::from("Yellow")).or_insert(50);
    scores.entry(String::from("Blue")).or_insert(50);

    println!("{:?}", scores);

    // Updating a value based on the old value
    let text = "hello world wonderful world";
    let mut map = HashMap::new();
    for word in text.split_whitespace() {
        let count = map.entry(word).or_insert(0);
        *count += 1;
    }

    println!("{:?}", map);
}
