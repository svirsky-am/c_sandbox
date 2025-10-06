// Структура с именованными полями
struct Person {
    age: u8,
    is_active: bool,
}

// Структура-кортежа
struct Point(i32, i32);

// Пример юнит-подобной структуры
struct Logger;

enum UserAction {
    SignOut,                       // без данных
    MoveCursor { x: i32, y: i32 }, // с именованными полями
    SendMessage(String),           // с одним значением
    ChangeTheme(u8, u8, u8),       // с кортежем
}

pub fn example_data_types() {
    // Объявление мутабельной переменной
    // и присвоение начального значения
    let mut x = 1;

    // Объявление неизменяемой переменной
    // и присвоение начального значения
    let name = "Боб";

    println!("Привет {} !", name);

    println!("Ты пользователь № {}", x);

    // Изменение мутабельной переменной
    x = 2;

    // "Затенение" переменной
    let name = "Алиса";

    println!("Привет {} !", name);

    println!("Ты пользователь № {}", x);

    // Тип по умолчанию - `i32`
    let default_int = -10;

    // Пример явной аннотации - `u8`
    let unsigned_int: u8 = 5;

    // Пример суффиксной аннотации - `i16`
    let signed_int = -10i16;

    // ---

    // Пример выведения типа из контекста.

    // несмотря на значение при инициализации, ...
    let mut inferred_int = 30;

    // ... тип `i64` фактически выводится здесь
    inferred_int = 3_000_000_000i64;

    // ---

    // Пример преобразования типа через `as`
    let casted_int = 10u8 as i32;

    // Тип по умолчанию - `f64`
    let default_float = -10.5;

    // Пример явной аннотации - `f32`
    let float: f32 = 5.0;

    // Пример выведения типа из контекста - `f32`.
    let mut inferred_float = 10.0;
    inferred_float = 1_000.0f32;

    // Пример преобразования типа через `as`
    let casted_float = 10f32 as f64;

    // Пример экспоненциальной записи - `f64` (1_000_000.0)
    let exp_float = 1e6;

    // Пример неявной аннотации - `char`
    let default_char = 'a';

    // Пример явной аннотации - `char`
    let ecliptic_char: char = 'ℤ';

    // Вариант используемого значения
    let emoji_char = '🦀';

    // Пример преобразования через `as`
    let casted_char = '🦀' as u32;

    let person = (30, 1.85, true);

    // Пример явной аннотации - `(i32, bool, char)`
    let team: (i32, bool, char) = (10, true, '🦀');

    // ---

    // ВАЖНО: кортеж из одного элемента требует запятую!
    let single_tuple = (10,);
    println!("single_tuple: {} (тип (i32,))", single_tuple.0);

    // // А это НЕ кортеж! это скалярный тип `i32`
    // let single_number = (10);
    // println!("single_number: {} (тип i32)", single_number);

    // ---

    // Пример деструктуризации кортежа
    let (age, height, is_active) = person;
    println!("Возраст: {}, рост: {}, активен: {}", age, height, is_active);

    // Пример доступа по индексу
    let team_icon = team.2;
    println!("Символ команды: {}", team_icon);

    // Пример неявной аннотации - `[i32; 7]`
    let week_temperatures = [20, 22, 19, 24, 21, 22, 25];

    // Пример явной аннотации - `[char; 4]`
    let weather_icons: [char; 4] = ['🌤', '🌧', '☀', '⚡'];

    // Пример инициализации с повторением - `[bool; 7]`
    let day_monitoring_flags = [true; 7];

    // Пример доступа по индексу
    println!(
        "Понедельник: {}°C {}",
        week_temperatures[0], weather_icons[0]
    );

    let person = Person {
        age: 20,
        is_active: true,
    };

    println!("Возраст: {}", person.age);

    let action = UserAction::SignOut;
}
