// https://practicum.yandex.ru/learn/middle-rust-blockchain/courses/0c7c693a-6212-46be-90a7-4a9f947ad137/sprints/736337/topics/9be69bb7-feae-4f38-ab93-82f83f56e277/lessons/c1fe7786-c477-4afd-b039-a307db457c45/

use std::rc::Rc;

use std::cell::Cell;

use std::cell::RefCell;

#[derive(Debug, PartialEq)]
enum ListBox {
    ConsBox(i32, Box<ListBox>),
    Nil,
}

// use ListBox::{ConsBox, Nil};

#[derive(Debug)]
enum ListRc {
    ConsRc(i32, Rc<ListRc>),
    Nil,
}

// Практическое задание 1: использование RefCell для изменения данных во внутренней структуре
fn add_value(cell: &RefCell<Vec<i32>>, value: i32) {
    let mut borrowed = cell.borrow_mut();
    borrowed.push(value);
}

#[derive(Debug)]
enum List {
    Cons(Rc<RefCell<i32>>, Rc<List>),
    Nil,
}

use List::{Cons, Nil};

pub fn fake_main() {
    println!("Начало main");

    {
        println!("Вход во внутреннюю область видимости");
        let b = Box::new(5);
        println!("b = {}", b);
        println!("Выход из внутренней области видимости");
    } // <- Здесь Box<T> автоматически освобождает память

    println!("Продолжение main - память уже освобождена");

    // Также можно использовать Box для рекурсивных типов данных. Посмотрите на этот код:
    let list = ListBox::ConsBox(
        1,
        Box::new(ListBox::ConsBox(
            2,
            Box::new(ListBox::ConsBox(3, Box::new(ListBox::Nil))),
        )),
    );

    assert_eq!(
        list,
        ListBox::ConsBox(
            1,
            Box::new(ListBox::ConsBox(
                2,
                Box::new(ListBox::ConsBox(3, Box::new(ListBox::Nil)))
            ))
        )
    );
    println!("{:?}", list);

    // Rc<T> ― совместное владение
    let a = Rc::new(ListRc::ConsRc(
        5,
        Rc::new(ListRc::ConsRc(10, Rc::new(ListRc::Nil))),
    ));
    println!("Счётчик ссылок после создания a: {}", Rc::strong_count(&a)); // 1

    let b = ListRc::ConsRc(3, Rc::clone(&a));
    println!("Счётчик ссылок после создания b: {}", Rc::strong_count(&a)); // 2

    {
        let c = ListRc::ConsRc(4, Rc::clone(&a));
        println!("Счётчик ссылок после создания c: {}", Rc::strong_count(&a)); // 3
    }

    println!(
        "Счётчик ссылок после выхода c из области видимости: {}",
        Rc::strong_count(&a)
    ); // 2

    // Cell<T> — простая внутренняя изменяемость

    let c = Cell::new(5);
    let five = c.get();
    c.set(10);
    let ten = c.get();

    println!("Было: {}, стало: {}", five, ten); // Было: 5, стало: 10

    // RefCell<T> — безопасная внутренняя изменяемость

    let value = RefCell::new(5);

    // Неизменяемое заимствование
    {
        let borrowed = value.borrow();
        println!("Значение: {}", *borrowed); // 5
    } // borrowed выходит из области видимости

    // Изменяемое заимствование
    {
        let mut borrowed_mut = value.borrow_mut();
        *borrowed_mut += 10;
    } // borrowed_mut выходит из области видимости

    println!("Новое значение: {}", *value.borrow()); // 15

    // Практическое задание 1: использование RefCell для изменения данных во внутренней структуре

    let numbers = RefCell::new(Vec::new());

    add_value(&numbers, 10);
    add_value(&numbers, 20);
    add_value(&numbers, 30);

    // Проверка корректности
    let borrowed = numbers.borrow();
    assert_eq!(&*borrowed, &[10, 20, 30]);

    println!("Все проверки прошли! numbers = {:?}", *borrowed);

    // Rc<RefCell<T>> — совместное владение с изменяемостью
    let value = Rc::new(RefCell::new(5));

    let a = Rc::new(Cons(Rc::clone(&value), Rc::new(Nil)));
    let b = Cons(Rc::new(RefCell::new(3)), Rc::clone(&a));
    let c = Cons(Rc::new(RefCell::new(4)), Rc::clone(&a));

    println!("a до изменения = {:?}", a);
    // Изменяем значение
    *value.borrow_mut() += 10;

    println!("a после изменения = {:?}", a);
    println!("b после изменения = {:?}", b);
    println!("c после изменения = {:?}", c);
    // a после изменения = Cons(RefCell { value: 15 }, Nil)
    // b после изменения = Cons(RefCell { value: 3 }, Cons(RefCell { value: 15 }, Nil))
    // c после изменения = Cons(RefCell { value: 4 }, Cons(RefCell { value: 15 }, Nil))
}
