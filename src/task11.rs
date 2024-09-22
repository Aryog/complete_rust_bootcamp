// Example of Borrwing

pub fn borrow_string() {
    let s1 = String::from("hello borrow");
    print_str(&s1);
    println!("{}", s1);
}
fn print_str(s2: &String) {
    println!("{}", s2);
}
