use std::{
    cell::RefCell,
    ops::Deref,
    rc::{Rc, Weak},
};

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

    println!("======================================");

    #[derive(Debug)]
    enum ListR {
        Cons(i32, RefCell<Rc<ListR>>),
        Nil,
    }

    impl ListR {
        fn tail(&self) -> Option<&RefCell<Rc<ListR>>> {
            match self {
                ListR::Cons(_, item) => Some(item),
                ListR::Nil => None,
            }
        }
    }

    let a = Rc::new(ListR::Cons(5, RefCell::new(Rc::new(ListR::Nil))));

    println!("a initial rc count = {}", Rc::strong_count(&a));
    println!("a next item = {:?}", a.tail());

    let b = Rc::new(ListR::Cons(10, RefCell::new(Rc::clone(&a))));

    println!("a rc count after b creation = {}", Rc::strong_count(&a));
    println!("b initial rc count = {}", Rc::strong_count(&b));
    println!("b next item = {:?}", b.tail());

    if let Some(link) = a.tail() {
        *link.borrow_mut() = Rc::clone(&b);
    }

    println!("b rc count after changing a = {}", Rc::strong_count(&b));
    println!("a rc count after changing a = {}", Rc::strong_count(&a));

    // Uncomment the next line to see that we have a cycle
    // it will overflow the stack
    // println!("a next item = {:?}", a.tail());

    println!("======================================");

    #[derive(Debug)]
    struct Node {
        value: i32,
        parent: RefCell<Weak<Node>>,
        children: RefCell<Vec<Rc<Node>>>,
    }

    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaft strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    {
        let branch = Rc::new(Node {
            value: 5,
            parent: RefCell::new(Weak::new()),
            children: RefCell::new(vec![Rc::clone(&leaf)]),
        });

        *leaf.parent.borrow_mut() = Rc::downgrade(&branch);

        println!(
            "branch strong = {}, weak = {}",
            Rc::strong_count(&branch),
            Rc::weak_count(&branch)
        );
        println!(
            "leaf strong = {}, weak = {}",
            Rc::strong_count(&leaf),
            Rc::weak_count(&leaf)
        );
    }
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );
}

fn hello(name: &str) {
    println!("Hello, {name}");
}
