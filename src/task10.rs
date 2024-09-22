pub fn check() {
    let a1 = String::from("yogesh");
    do_something(a1);

    // Below line gives error Move feature
    // println!("number is {}", a2);
}

fn do_something(a2: String) {
    println!("{}", a2);
}
