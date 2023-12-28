pub fn vec_examples() {
    println!("===============================Vector===============================");

    let _v: Vec<i32> = Vec::new();
    let mut v = vec![1, 2, 3];
    v.push(4);
    v.push(5);
    v.push(6);
    // v.push("Seven"); // Invalid data type only expects i32

    let third = &v[2];
    println!("The third element is {third}");

    let third = v.get(2);
    match third {
        Some(third) => println!("The third element is {third}"),
        None => println!("There is no third element."),
    }

    let first = &v[0];
    // v.push(7); // mutable borrow occurs here
    println!("The first element is: {first}");

    let v = vec![100, 32, 57];
    for n_ref in &v {
        // n_ref has type &i32
        let n_plus_one = *n_ref + 1;
        println!("{n_plus_one}");
    }

    let mut v = vec![100, 32, 57];
    for n_ref in &mut v {
        *n_ref += 50;
        println!("{n_ref}");
    }
}
