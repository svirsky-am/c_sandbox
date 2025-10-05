fn heart_rate_status(bpm: u8) -> char {
    // Пример присваивания значения
    let result = if bpm < 50 {
        '🐢'
    } else if bpm < 100 {
        '🙂'
    } else {
        '🔥'
    };

    return result;
}

enum Command {
    Quit,
    Move { x: i32, y: i32 },
}

enum UserRole {
    Guest,
    Moderator,
    Admin,
}

struct User {
    name: String,
    role: UserRole,
    active: bool,
}

pub fn fake_main() {
    let cmd = Command::Move { x: 5, y: 10 };

    if let Command::Move { x, y } = cmd {
        println!("Координаты перемещения: ({x},{y})");
    }

    // Пример использования с кортежем
    let transaction = (1500.0, "USD", 200);

    // Явно указываем, каким должен быть 2-й и 3-й элемент ...
    // ... только `USD` и `status_code == 200`
    if let (amount, "USD", 200) = transaction {
        println!("Успешный платёж: ${} USD", amount);
    }

    // Пример использования с массивами
    let download_progress = [0, 25, 50, 75, 100];

    // Загрузка должна начинаться с 0% !
    if let [0, .., completion] = download_progress {
        if completion == 100 {
            println!("Загрузка успешно завершена!");
        }
    };

    // Пример использования со структурами
    let new_user = User {
        name: "Алиса".to_string(),
        role: UserRole::Admin,
        active: true,
    };

    // Только активный Admin !
    if let User {
        name,
        role: UserRole::Admin,
        active: true,
    } = new_user
    {
        println!("У нас новый Администратор: {}", name);
    }

    // Имитация значения в виде кортежа
    let config = ("localhost", 8080);

    // Ранняя проверка
    let ("localhost", port) = config else {
        // Возврат `never` (паника)
        panic!("Ожидался хост 'localhost', получен: {}", config.0);
    };

    println!("Сервер запущен на localhost:{}", port);

    // Имитация значения в виде массива
    let upload_progress = [0, 25, 50, 75, 100];

    // Ранняя проверка
    let [0, .., 100] = upload_progress else {
        println!("Загрузка не завершилась");
        // Возврат `never`
        return;
    };

    println!("Загрузка успешно завершена!");

    let dice_roll = 9;

    match dice_roll {
        3 => println!("Получен бонус!"),
        7 => println!("Потеря хода."),
        _ => println!("Продолжайте игру."),
    }

    let age = 25;

    match age {
        0..=12 => println!("Ребёнок"),
        13..=19 => println!("Подросток"),
        20..=64 => println!("Взрослый"),
        _ => println!("Пожилой"),
    }

    let some_point = (1, 20);

    match some_point {
        (0, 0) => println!("Начало координат"),
        (x, 0) => println!("На оси X: x = {}", x),
        (0, y) => println!("На оси Y: y = {}", y),
        (x, y) => println!("Точка: ({}, {})", x, y),
    } 



}
