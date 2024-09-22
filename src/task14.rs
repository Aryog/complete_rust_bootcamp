use std::collections::HashMap;

pub fn hashmap() {
    let mut users: HashMap<String, i32> = HashMap::new();
    users.insert(String::from("Yogesh"), 21);
    users.insert(String::from("Yug"), 32);

    let age1 = users.get("Yogesh");
    match age1 {
        Some(age1) => println!("{}", age1),
        None => println!("User not found in the db"),
    }
}
