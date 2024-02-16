use ch17_oop::{AveragedCollection, Button, Screen, SelectBox};

fn main() {
    let mut collection = AveragedCollection::new(Vec::new());

    println!("Average: {}", collection.average());

    collection.add(5);
    collection.add(10);
    collection.add(25);

    println!("Average: {}", collection.average());

    if let Some(value) = collection.remove() {
        println!("Remove value: {}", value);
    } else {
        println!("Collection is empty.");
    }

    println!("Updated average: {}", collection.average());

    println!("============================================");

    let screen = Screen {
        components: vec![
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(Button {
                width: 50,
                height: 10,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}
