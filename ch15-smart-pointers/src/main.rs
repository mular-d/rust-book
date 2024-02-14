use std::{ops::Deref, rc::Rc};

fn main() {
    let b = Box::new(5);
    println!("b = {}", b);

    enum List {
        Cons(i32, Box<List>),
        Nil,
    }

    let _list = List::Cons(
        1,
        Box::new(List::Cons(2, Box::new(List::Cons(3, Box::new(List::Nil))))),
    );

    println!("======================================");

    let x = 5;
    let y = &x;

    assert_eq!(5, x);
    // assert_eq!(5, y); // Error: cannot compare {integer} with &{integer}
    assert_eq!(5, *y);

    println!("======================================");

    let x = 5;
    let y = Box::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("======================================");

    struct MyBox<T>(T);

    impl<T> MyBox<T> {
        fn new(x: T) -> MyBox<T> {
            MyBox(x)
        }
    }

    impl<T> Deref for MyBox<T> {
        type Target = T;

        fn deref(&self) -> &Self::Target {
            &self.0
        }
    }

    let x = 5;
    let y = MyBox::new(x);

    assert_eq!(5, x);
    assert_eq!(5, *y);

    println!("======================================");

    let m = MyBox::new(String::from("Rust"));
    hello(&m);

    println!("======================================");

    struct CustomSmartPointer {
        data: String,
    }

    impl Drop for CustomSmartPointer {
        fn drop(&mut self) {
            println!("Dropping CustomSmartPointer with data `{}`", self.data);
        }
    }

    let _c = CustomSmartPointer {
        data: String::from("my stuff"),
    };
    let _d = CustomSmartPointer {
        data: String::from("other stuff"),
    };
    println!("CustomSmartPointers created.");
    drop(_c);
    println!("CustomSmartPointer `c` dropped before the end of main");

    println!("======================================");

    let a = List::Cons(5, Box::new(List::Cons(10, Box::new(List::Nil))));
    let _b = List::Cons(3, Box::new(a));
    // let c = List::Cons(4, Box::new(a)); // value used after move

    println!("======================================");

    enum RcList {
        Cons(i32, Rc<RcList>),
        Nil,
    }

    let a = Rc::new(RcList::Cons(
        5,
        Rc::new(RcList::Cons(10, Rc::new(RcList::Nil))),
    ));
    println!("count after creating a = {}", Rc::strong_count(&a));
    let _b = RcList::Cons(3, Rc::clone(&a));
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let _c = RcList::Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}

fn hello(name: &str) {
    println!("Hello, {name}");
}
