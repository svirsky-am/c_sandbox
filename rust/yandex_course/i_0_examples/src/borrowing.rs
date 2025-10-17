#![allow(dead_code)]
#![allow(unused_variables)]

fn calculate_length(s: &String) -> usize {
    s.len()
}

fn change(s: &mut String) {
    s.push_str(" Практикум");

    // Понижение изменяемости - `&mut String` → `&str`
    log(s);
}

fn log(data: &str) {
    println!("[LOG]: {}", data);
}

pub fn fake_main() {
    let s1 = String::from("Яндекс");

    let len = calculate_length(&s1);

    println!("Длина строки '{s1}': {len}.");
    // Изменяемые ссылки
    let mut s2 = String::from("Яндекс");

    change(&mut s2);
    let len2 = calculate_length(&s2);
    println!("Длина строки '{s2}': {len2}.");
}
