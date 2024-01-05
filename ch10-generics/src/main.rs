// generic struct
struct Point<T> {
    x: T,
    y: T,
}

// generic impl
impl<T> Point<T> {
    fn x(&self) -> &T {
        &self.x
    }
}

// concrete impl
impl Point<f32> {
    fn distance_from_origin(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
}

fn main() {
    let number_list = vec![34, 50, 25, 100, 65];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8];
    let result = largest(&number_list);
    println!("The largest number is {}", result);

    let p = Point { x: 5, y: 4 };
    println!("{}", p.x());
    // let wont_work = Point { x: 5, y: 4.9 }; with Point<T> { x: T, y: T }
    // let both_integer = Point { x: 5, y: 4 }; will work with Point<T> { x: T, y: U }
    // let both_float = Point { x: 5.5, y: 4.4 }; will work with Point<T> { x: T, y: U }
    // let integer_and_float = Point { x: 5, y: 4.9 }; will work with Point<T> { x: T, y: U }
}

fn largest<T: PartialOrd>(list: &[T]) -> &T {
    let mut largest = &list[0];

    for item in list {
        if item > largest {
            largest = item
        }
    }

    largest
}
