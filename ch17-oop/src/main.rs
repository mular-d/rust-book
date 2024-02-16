use ch17_oop::AveragedCollection;

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
}
