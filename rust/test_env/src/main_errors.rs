use std::num::ParseIntError;

fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
    match s.parse::<i32>() {
        Ok(n) => Ok(n * 2),
        Err(e) => Err(e),
    }
}

fn main() {
    let inputs = ["50", "abc"];

    for s in &inputs {
        match parse_and_double(s) {
            Ok(v) => println!("Удвоенное число: {}", v),
            Err(e) => println!("Ошибка парсинга: {}", e),
        }
    }
}