// Do return error
use std::fs::read_to_string;

pub fn check_read(file: String) -> Result<String, String> {
    let result = read_to_string(file);

    match result {
        Ok(data) => Ok(data),
        Err(_err) => Err(String::from("File not read")),
    }
}
