use std::collections::HashMap;

pub fn get_hm_from_vec() {
    let input_vec = vec![(String::from("yogesh"), 22), (String::from("yug"), 32)];
    let hm = group_values_by_keys(input_vec);
    println!("{:?}", hm);
}

fn group_values_by_keys(vec: Vec<(String, i32)>) -> HashMap<String, i32> {
    let mut hm = HashMap::new();
    for (key, value) in vec {
        hm.insert(key, value);
    }
    return hm;
}
