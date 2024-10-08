fn filter_and_map(v: Vec<i32>) -> Vec<i32> {
    let new_iter = v.iter().filter(|x| *x % 2 == 1).map(|x| x + 1);
    let new_vec: Vec<i32> = new_iter.collect();
    return new_vec;
}
pub fn iterator_adaptor() {
    let v1: Vec<i32> = vec![1, 2, 3];
    let ans = filter_and_map(v1);
    println!("{:?}", ans);
}
