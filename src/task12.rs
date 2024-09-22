pub fn mutate_string() {
    let mut s1 = String::from("Yogesh");
    print_str(&mut s1);
    println!("{}", s1);
}
fn print_str(s2: &mut String) {
    s2.push_str("Aryal");
    println!("{}", s2);
}
