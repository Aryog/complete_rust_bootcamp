// src/task2.rs
pub fn fibonacci(num: u32) -> u32 {
    let mut first = 0;
    let mut second = 1;
    if num == 0 {
        return first;
    }

    for _ in 1..num {
        let temp = second;
        second = second + first;
        first = temp;
    }
    second
}
