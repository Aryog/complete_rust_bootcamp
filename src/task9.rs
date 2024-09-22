use chrono::Local;
pub fn fetch_date_time() {
    let now = Local::now();
    println!("current time is {}", now);
}
