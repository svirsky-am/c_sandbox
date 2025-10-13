#![allow(dead_code)]
#![allow(unused_variables)]

use std::fmt;

#[derive(Debug)]
struct Student {
    name: String,
    age: u8,
}

impl fmt::Display for Student {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} ({})", self.name, self.age)
    }
}

// Альтернатива для #[derive(Debug)]

// impl fmt::Debug for Student {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         // Один из простых примеров кастомизации
//         f.debug_struct("Student")
//             .field("name", &self.name)
//             .field("age", &format_args!("{} лет", self.age))
//             .finish()
//     }
// }

pub fn fake_main() {
    let s1 = "Яндекс";
    let s2 = "Практикум";

    // Форматирование и возврат `String`
    let s: String = format!("{} {}", s1, s2);

    // Вывод в консоль
    println!("{}", s);

    let student = Student {
        name: String::from("Алиса"),
        age: 25,
    };

    // Вывод: Алиса (25)
    println!("Студент: {}", student);

    // let pi2 = 3.1415;
    let pi2 = std::f64::consts::PI;

    let motivator = '🦀';

    // Пример форматирования чисел с плавающей точкой
    // Вывод: `pi2 = 3.14`
    println!("pi2 = {:.2}", pi2);

    // ---

    // Примеры форматирования с указанием ширины заполнения и выравнивания

    // Ширина заполнения: 5 символов (пробел по умолчанию)
    // Вывод: `🦀    `
    println!("`{:5}`", motivator);

    // Эквивалентно варианту выше
    // Ширина заполнения: 5 символов
    // Выравнивание: слева (`<`)
    // Вывод: `🦀    `
    println!("`{:<5}`", motivator);

    // `<` - выравнивание справа
    // Вывод: `    🦀`
    println!("`{motivator:>5}`");

    // `^` - выравнивание по центру
    // Вывод: `  🦀  `
    println!("`{0:^5}`", motivator);

    // Кастомный символ заполнения (`-`)
    // Вывод: `--🦀--`
    println!("`{:-^5}`", motivator);

    // ---

    // Пример форматирования с добавлением лидирующих нулей
    // Вывод: `000031`
    println!("{:06}", 31);

    // Вывод: Student { name: "Алиса", age: 25 }
    println!("{:?}", student);
    // Вывод:
    // Student {
    //     name: "Алиса",
    //     age: 25,
    // }
    println!("{:#?}", student);

    let x = 10;
    let y = dbg!(x * 2) + 5;
}
