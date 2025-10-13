// Декларативная обработка ошибок в Rust

//Императивный стиль

fn parse_and_double_imp(s: &str) -> Result<i32, std::num::ParseIntError> {
    // Пытаемся распарсить строку в число
    let n = match s.parse::<i32>() {
        Ok(num) => num,
        Err(e) => return Err(e), // явно выходим при ошибке
    };

    // Умножаем результат на 2 и возвращаем
    Ok(n * 2)
}

// Функциональный стиль

fn parse_and_double(s: &str) -> Result<i32, std::num::ParseIntError> {
    s.parse::<i32>().map(|n| n * 2)
}

// 2. map_err. То же самое, но применяется к ошибке. Удобно для преобразования типа ошибки.

fn parse_number(s: &str) -> Result<i32, String> {
    s.parse::<i32>()
        .map_err(|e| format!("Ошибка парсинга: {}", e))
}

// 3. and_then. Позволяет продолжить цепочку, если результат был успешный (Ok / Some).

// императивно
fn square_root(num: f64) -> Result<f64, String> {
    if num >= 0.0 {
        Ok(num.sqrt())
    } else {
        Err("Отрицательное число".into())
    }
}
// функционально
fn parse_and_sqrt(s: &str) -> Result<f64, String> {
    s.parse::<f64>()
        .map_err(|_| "Не удалось распарсить".into())
        .and_then(square_root)
}

// 4. or_else. Аналог and_then, но для ошибки. Позволяет заменить ошибку на другой результат.

// Читаем значение из файла, и если получаем ошибку, то
// читаем значение из переменной окружения

// fn get_config_value(key: &str) -> Result<String, String> {
//     read_from_file(key).or_else(|_| read_from_env(key))
// }

// 5. unwrap_or, unwrap_or_else, unwrap_or_default. Достаём значение или берём значение по умолчанию, если не получилось взять значение.
// Пытаемся взять переменную окружения PORT, в случае неудачи ставим по умолчанию 8080

// Цепочки значений

// Вспомогательная функция для возведения числа в корень
fn square_root_v3(x: f64) -> Result<f64, String> {
    if x >= 0.0 {
        Ok(x.sqrt())
    } else {
        Err("Отрицательное число".into())
    }
}

fn process(s: &str) -> Result<f64, String> {
    s.parse::<f64>() // Парсим строку в число
        .map_err(|_| "Парсинг не удался".into()) // Преобразуем ошибку
        .and_then(square_root) // Берём корень.
        .map(|r| r * 10.0) // Умножаем результат на 10 
}

// Вспомогательная функция для возведения числа в корень
fn square_root_v2(x: f64) -> Result<f64, String> {
    if x >= 0.0 {
        Ok(x.sqrt())
    } else {
        Err("Отрицательное число".into())
    }
}

fn process_vec(data: Vec<&str>) -> Result<Vec<f64>, String> {
    data.into_iter()
        .map(|s| {
            s.parse::<f64>() // Применяет парсер к каждому элементу вектора
                .map_err(|_| format!("Не удалось распарсить '{}'", s)) // Преобразуем ошибку
                .and_then(square_root) // Корень каждого элемента вектора
                .map(|r| r * 10.0) // Умножаем каждый элемент на 10
        })
        .collect() // Собираем финальный вектор после всех преобразований
}

fn get_port_config(env_var: Option<String>) -> u16 {
    // 1. Если env_var есть, попробуйте распарсить в u16
    // 2. Если парсинг неудачен или значения нет, верните 8080
    // Используйте функциональный стиль!
    let result = env_var.and_then(|s| s.parse::<u16>().ok()).unwrap_or(8080);
    result
    // Some(result)
}

// Макрос panic!
use std::fs;

fn load_game_config(path: &str) -> Result<String, String> {
    let content =
        fs::read_to_string(path).map_err(|_| format!("Не удалось прочитать конфиг: {}", path))?;
    Ok(content)
}

fn get_item(vec: &Vec<i32>, index: usize) -> i32 {
    vec[index] // если index >= vec.len(), вызовется panic!
}

fn only_positive(x: i32) {
    if x < 0 {
        panic!("Ожидалось положительное число, а пришло {}", x);
    }
}
// Программа завершит выполнение и напечатает в консоль причину

#[derive(Debug)]
enum AuthError {
    InvalidPassword,
    UserNotFound(String),
    TokenExpired,
}

use std::error::Error;

impl Error for AuthError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            AuthError::UserNotFound(err) => None,
            AuthError::InvalidPassword => None,
            AuthError::TokenExpired => None,
        }
    }
}

use std::fmt;

impl fmt::Display for AuthError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AuthError::UserNotFound(username) => {
                // write!(f, "Пользователь 'john' не найден: {}", err)
                write!(f, "Пользователь '{}' не найден", username)
            }
            AuthError::InvalidPassword => {
                write!(f, "Неверный пароль")
            }
            AuthError::TokenExpired => {
                write!(f, "Токен истёк")
            }
        }
    }
}

// Box<dyn Error>
// Динамическая диспетчерезация ошибок

fn read_and_parse_file(path: &str) -> Result<i32, Box<dyn Error>> {
    let content = std::fs::read_to_string(path)?; // std::io::Error
    let number = content.trim().parse::<i32>()?; // ParseIntError
    Ok(number)
}

// Трейты From и Into для преобразования ошибок

// Объявляем перечисление с возможными ошибками
#[derive(Debug)]
enum MyError {
    NotFound,
    Parse(ParseIntError),
}

// Реализуем трейт Display для красивого отображения
impl fmt::Display for MyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyError::NotFound => write!(f, "Элемент не найден"),
            MyError::Parse(e) => write!(f, "Ошибка парсинга числа: {}", e),
        }
    }
}

// Реализуем трейт Error
// В большинстве простых случаев методы трейта реализовывать не обязательно
impl Error for MyError {}

use std::num::ParseIntError;
// Реализуем трейт From
impl From<ParseIntError> for MyError {
    fn from(err: ParseIntError) -> Self {
        MyError::Parse(err)
    }
}

// Используем наш enum с ошибкой и автоматическим преобразованием ошибки
fn read_number_from_vec(data: Vec<&str>, index: usize) -> Result<i32, MyError> {
    let s = data.get(index).ok_or(MyError::NotFound)?; // тут может вернуться ошибка
    let number = s.parse::<i32>()?; // тут ошибка преобразуется автоматически через From
    Ok(number)
}

pub fn fake_main() {
    let port: i32 = std::env::var("PORT")
        .ok()
        .and_then(|s| s.parse::<i32>().ok())
        .unwrap_or(8080);

    let nums = vec!["9.0", "4.0", "16.0", "4.0"];

    // Попробуем преобразовать всё в Vec<f64>
    match process_vec(nums) {
        Ok(results) => println!("Результаты: {:?}", results),
        Err(err) => println!("Ошибка: {}", err),
    }

    // Практическое задание 1: функциональная обработка

    // Тесты
    assert_eq!(get_port_config(Some("3000".to_string())), 3000);
    assert_eq!(get_port_config(Some("abc".to_string())), 8080);
    assert_eq!(get_port_config(None), 8080);
    println!("Тесты прошли");

    // // Макрос panic!
    // let config = load_game_config("game_config.txt")
    //     .expect("Конфигурация игры не найдена, программа не может продолжить!");

    // println!("Конфиг загружен: {}", config);
    // // Если нет файла настроек, то .expect() вызовет panic

    // Практическое задание 2: создание собственной ошибки
    let err = AuthError::UserNotFound("john".to_string());
    assert_eq!(err.to_string(), "Пользователь 'john' не найден");

    let err = AuthError::InvalidPassword;
    assert_eq!(err.to_string(), "Неверный пароль");

    let err = AuthError::TokenExpired;
    assert_eq!(err.to_string(), "Токен истёк");

    println!("Тесты прошли");

    // Трейты From и Into для преобразования ошибок

    let data = vec!["10", "20", "oops", "40"];

    match read_number_from_vec(data.clone(), 1) {
        Ok(n) => println!("Нашли число: {}", n),
        Err(e) => println!("Ошибка: {}", e),
    }

    match read_number_from_vec(data.clone(), 2) {
        Ok(n) => println!("Нашли число: {}", n),
        Err(e) => println!("Ошибка: {}", e),
    }

    match read_number_from_vec(data.clone(), 10) {
        Ok(n) => println!("Нашли число: {}", n),
        Err(e) => println!("Ошибка: {}", e),
    }
    // Вывод в консоль
    // Нашли число: 20
    // Ошибка: Ошибка парсинга числа: invalid digit found in string
    // Ошибка: Элемент не найден
}
