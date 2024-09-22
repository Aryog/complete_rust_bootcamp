pub fn vector_imp() {
    let mut vec = Vec::new();
    vec.push(1);
    vec.push(2);
    vec.push(3);
    let ans = even_val_filter(&mut vec);
    println!("{:?}", vec);
    println!("{:?}", ans);
}

// Using the vew vector
// fn even_val_filter(vec: &Vec<i32>) -> Vec<i32> {
//     let mut new_vec = Vec::new();
//     for val in vec {
//         if val % 2 == 0 {
//             new_vec.push(*val);
//         }
//     }
//     return new_vec;
// }

fn even_val_filter(v: &mut Vec<i32>) {
    let mut i = 0;
    while i < v.len() {
        if v[i] % 2 != 0 {
            v.remove(i);
        }
        i += 1;
    }
}
