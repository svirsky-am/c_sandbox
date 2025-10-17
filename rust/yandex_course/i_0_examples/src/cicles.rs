#![allow(dead_code)]
#![allow(unused_variables)]

#[derive(Debug, PartialEq)]
enum Command {
    Move,
    Attack,
    Defend,
    Wait,
    End,
}

pub fn fake_main() {
    // loop {
    //     println!("Этот цикл будет выполняться бесконечно...");

    //     // Условие, которое никогда не наступит
    //     if false {
    //         break;
    //     }
    // }

    let mut counter = 0;

    // loop может возвращать значения при завершении с помощью выражения break.
    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    let mut counter = 5;

    while counter > 0 {
        println!("{counter}");
        counter -= 1;
    }

    println!("Счётчик достиг нуля.");

    //#Пример while-let:
    let commands = [
        Command::Move,
        Command::Attack,
        Command::Wait,
        Command::Defend,
    ];
    let mut index = 0;

    while let Some(command) = commands.get(index) {
        println!("Выполняется команда: {:?}", command);
        index += 1;
    }

    println!("Все команды выполнены");

    for i in 1..10 {
        println!("Число: {}", i);
    }

    // Метки циклов в Rust

    let matrix = [[1, 3, 5], [2, 7, 4], [9, 0, 6]];
    let needle = 9;

    let mut i = 0;

    'search: loop {
        if i >= matrix.len() {
            println!("{} не найдено.", needle);
            break;
        }

        let mut j = 0;
        loop {
            if j >= matrix[i].len() {
                break;
            }

            if matrix[i][j] == needle {
                println!("{} найдено в позиции ({}, {})!", needle, i, j);

                // Выход сразу из внешнего цикла!
                break 'search;
            }

            j += 1;
        }

        i += 1;
    }

    println!("Конец.");
}
