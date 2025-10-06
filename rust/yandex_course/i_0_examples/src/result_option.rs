// enum Option<T> {
//     Some(T),
//     None,
// }

// Найти первое слово длиннее 5 символов
fn find_long_word(words: &[String]) -> Option<&String> {
    for word in words {
        if word.len() > 5 {
            return Some(word); // нашли подходящее слово
        }
    }
    None // ничего не нашли
}

fn print_first_even(numbers: &[i32]) {
    // Попытка взять первое чётное число
    let Some(n) = numbers.iter().find(|&&x| x % 2 == 0) else {
        println!("Чётных чисел нет");
        return;
    };

    println!("Первое чётное число: {}", n);
}

fn find_first_even(numbers: &[i32]) -> Option<i32> {
    for &num in numbers {
        if num % 2 == 0 {
            return Some(num); // нашли подходящее слово
        }
    }
    None // ничего не нашли
}

// enum Result<T, E> {
//     Ok(T),  // успешный результат (значение типа T)
//     Err(E), // ошибка (значение типа E)
// }

// T — это тип успешного значения.
// E — это тип ошибки.

fn divide(a: i32, b: i32) -> Result<i32, String> {
    if b == 0 {
        Err("деление на ноль".to_string())
    } else {
        Ok(a / b)
    }
}

//divide(10, 5) -> Ok(2)
//divide(10, 0) -> Err("деление на ноль")

// use std::num::ParseIntError;

// fn parse_and_double(s: &str) -> Result<i32, ParseIntError> {
//     match s.parse::<i32>() {
//         Ok(n) => Ok(n * 2),
//         Err(e) => Err(e),
//     }
// }

use std::num::ParseIntError;

fn parse_and_double(input: &str) -> Result<i32, ParseIntError> {
    // dbg!(&s);
    // let number = input.parse::<i32>()?;
    // Ok(number * 2)

    match input.parse::<i32>() {
        Ok(n) => Ok(n * 2),
        // Err(n) => Err("деление на ноль".to_string()), // Err(n) => Err(std::num::ParseIntError), // Err(e) => Err(e),
        Err(n) => Err(n), // Err(n) => Err(std::num::ParseIntError), // Err(e) => Err(e),
    }
}

#[derive(Debug, PartialEq)]
enum MathError {
    DivisionByZero,
}

fn safe_divide(a: f64, b: f64) -> Result<f64, MathError> {
    // Ваш код здесь
    if (b == 0.0) {
        return Err(MathError::DivisionByZero);
    }
    Ok(a / b)
}

// Оператор ? (Question Mark)

// //Вместо явного match

// fn read_file(path: &str) -> std::io::Result<String> {
//     match std::fs::read_to_string(path) {
//         Ok(content) => Ok(content),
//         Err(e) => return Err(e),
//     }
// }

fn read_file(path: &str) -> std::io::Result<String> {
    match std::fs::read_to_string(path) {
        Ok(content) => Ok(content),
        Err(e) => return Err(e),
    }
}

pub fn fake_main() {
    let x = Some(5); // значение есть
    let y: Option<i32> = None; // значения нет

    // 1. match. Полная и безопасная обработка всех вариантов:
    let number = Some(10);

    match number {
        Some(n) => println!("Нашли число: {}", n),
        None => println!("Ничего не нашли"),
    } // Вывод в консоль: Нашли число: 10

    // 2. if let. Удобный синтаксис, если интересует только Some или только None:

    let number: Option<i32> = Some(7);

    if let Some(n) = number {
        println!("Нашли число: {}", n);
    } else {
        println!("Ничего нет");
    }

    // Вывод в консоль: Нашли число: 7

    let number: Option<i32> = None;

    if let None = number {
        println!("Числа нет");
    } else {
        println!("Нашли число!");
    } // Вывод в консоль: Числа нет

    // 3. unwrap. Берёт значение из Some, но упадёт с паникой, если там None:
    let a = Some(42);
    println!("Значение: {}", a.unwrap()); // 42

    let b: Option<i32> = None;
    // println!("{}", b.unwrap()); // 💥 panic

    // 4. expect. То же самое, что unwrap, но можно указать сообщение:
    let a = Some("Rust");
    println!("Значение: {}", a.expect("Ожидали строку"));

    let b: Option<&str> = None;
    // println!("{}", b.expect("Значение должно быть!"));
    // panic с этим сообщением (аварийное завершение программы)

    // 5. let-else. Удобный синтаксис, который позволяет сразу извлекать значение из Option, а если его нет — выходить из функции или делать return, break, continue.
    let nums1 = vec![1, 3, 5, 7];
    let nums2 = vec![1, 4, 5, 6];

    print_first_even(&nums1); // Чётных чисел нет
    print_first_even(&nums2); // Первое чётное число: 4

    // Практическое задание 1: поиск в массиве
    // Тесты
    assert_eq!(find_first_even(&[1, 3, 4, 7, 8]), Some(4));
    assert_eq!(find_first_even(&[1, 3, 5, 7]), None);
    println!("Все тесты прошли!");

    // # Тип Result<T, E>

    let inputs = ["50", "abc"];
    dbg!(&inputs);

    for s in &inputs {
        match parse_and_double(s) {
            Ok(v) => println!("Удвоенное число: {}", v),
            Err(e) => println!("Ошибка парсинга: {}", e),
        }
    }

    // Успешный вариант
    if let Ok(value) = divide(10, 2) {
        println!("Результат: {}", value);
    }

    // Ловим ошибку
    if let Err(error) = divide(10, 0) {
        println!("Ошибка: {}", error);
    }

    // Вывод в консоль: Ошибка: деление на ноль
    // Вывод в консоль: Результат: 5

    if let Ok(value) = divide(10, 2) {
        println!("Результат: {}", value);
    } else {
        println!("Ошибка"); // Нет доступа к тексту ошибки 
    }

    if let Err(error) = divide(10, 0) {
        println!("Ошибка: {}", error);
    } else {
        println!("Операция прошла успешно"); // Нет доступа к значению результата операции
    }

    // Практическое задание 2: безопасное деление
    // Тесты
    assert_eq!(safe_divide(10.0, 2.0), Ok(5.0));
    assert_eq!(safe_divide(10.0, 0.0), Err(MathError::DivisionByZero));
    println!("Все тесты прошли!");
}
