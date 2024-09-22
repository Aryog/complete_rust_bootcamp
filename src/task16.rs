pub fn type_iter() {
    let v1 = vec![1, 2, 3];

    // Uses borrowing not ownership also default immutable
    let v1_iter = v1.iter();

    for val in v1_iter {
        println!("Got: {val}");
    }
    println!("{:?}", v1);
    // For mutability here is the function
    mut_type_iter();
    some_iter();
}

fn mut_type_iter() {
    let mut v1 = vec![1, 2, 3];
    let v1_iter = v1.iter_mut();
    for val in v1_iter {
        *val = *val + 1;
    }
    println!("{:?}", v1);
}

fn some_iter() {
    let mut v1 = vec![1, 2, 3];
    let mut v1_iter = v1.iter_mut();
    while let Some(val) = v1_iter.next() {
        println!("{}", val);
    }

    println!("{:?}", v1);
}
