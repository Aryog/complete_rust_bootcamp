// src/main.rs

mod task1;
mod task10;
mod task11;
mod task12;
mod task13;
mod task14;
mod task15;
mod task16;
mod task17;
mod task2;
mod task3;
mod task4;
mod task5;
mod task6;
mod task7;
mod task8;
mod task9;

use task10::check;
use task11::borrow_string;
use task12::mutate_string;
use task13::vector_imp;
use task14::hashmap;
use task15::get_hm_from_vec;
use task16::type_iter;
use task17::iterator_adaptor;
use task4::User;
use task5::Rect;
use task6::use_shape;
use task7::find_first_a;
use task7::CustomOption;
use task8::check_read;
use task9::fetch_date_time;

fn main() {
    println!("Hello, world!");

    // First task
    println!("{}", task1::is_even(21));

    // Second task
    println!("{}", task2::fibonacci(8));

    // Third task
    let my_string = String::from("Hello, world!");
    let length = task3::get_str_length(my_string);
    println!("The number of characters in the string is: {}", length);

    // Fourth task
    let user1 = User {
        active: true,
        username: String::from("Yogesh"),
        email: String::from("some@example.com"),
        sign_in_count: 1,
    };
    println!("User1 username: {:?}", user1.username);

    // Fifth task
    let rect = Rect {
        width: 30,
        height: 50,
    };
    println!("The area of the rectangle is {}", rect.area());
    println!("The debug data is {}", Rect::debug());

    // Sixth task
    use_shape();

    // Seventh task
    let index = find_first_a(String::from("Yogendra"));
    match index {
        CustomOption::Some(value) => println!("index is {}", value),
        CustomOption::None => println!("a not found"),
    }

    // Eight task
    let ans = check_read(String::from("file.txt"));
    match ans {
        Ok(file_content) => println!("File read successfully: {:?}", file_content),
        Err(error) => println!("Failed to read file: {:?}", error),
    }

    // Ninth task
    fetch_date_time();

    // Tenth task
    check();

    // Eleventh task
    borrow_string();

    // Twelth task
    mutate_string();

    // Thirteenth task
    vector_imp();

    // Fourteenth task
    hashmap();

    // Fifteenth task
    get_hm_from_vec();

    // Sixteenth task
    type_iter();

    // Seventeenth task
    iterator_adaptor();
}
