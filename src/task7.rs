pub enum CustomOption {
    Some(i32),
    None,
}
pub fn find_first_a(s: String) -> CustomOption {
    for (index, char) in s.chars().enumerate() {
        if (char == 'a') {
            return CustomOption::Some(index as i32);
        }
    }
    return CustomOption::None;
}
