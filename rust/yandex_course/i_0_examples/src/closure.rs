#![allow(dead_code)]
#![allow(unused_variables)]

fn take_ownership(s: String) {
    println!("{} practicum", s);
} // <- `s` выходит из области видимости и освобождается (drop) 

fn take_int_copy(num: i32) {
    println!("{}", num);
}

fn give_ownership() -> String {
    let s = String::from("yandex");
    s
} // <- `s` не освобождается - владение возвращается вызывающей стороне 

fn take_and_give_back(s: String) -> String {
    println!("Обработано: {}", s);
    s
}

pub fn fake_main() {
    let x = 1;

    {
        // `y` получает владение
        let y = x;

        println!("y = {}", y);
    }

    println!("x = {}", x);

    // Владение и работа с данными кучи

    // `s1` получет владение
    let s1 = String::from("yandex");
    // `s2` получает во владение полную копию `s1`
    let s2 = s1.clone();

    // Обе переменные валидны
    println!("{} и {}", s1, s2);

    {
        // Передача владения (move):
        // `s1` теряет владение
        // `s2` получет владение
        let s2 = s1;

        println!("{s2}");
    }

    // ОШИБКА: `s1` - больше не валидна
    // println!("{s1}");

    // Владение и функции
    let s4 = String::from("yandex");

    // Владение передаётся аргументу функции
    take_ownership(s4);

    // `x` получает владение
    let z = 10;

    // `x` побитово копируется в функцию
    take_int_copy(z);

    // `x` сохранил владение
    println!("{}", z);

    // `s5` получает владение
    let s5 = give_ownership();

    println!("{}", s5);

    // `s6` получает владение
    let s6 = String::from("yandex");

    // Владение передаётся функции
    // `s6` больше невалидна
    // `s7` получает владение
    let s7 = take_and_give_back(s6);

    // `s6` не может использоваться, `s7` — новый владелец
    println!("{}", s7);
}
