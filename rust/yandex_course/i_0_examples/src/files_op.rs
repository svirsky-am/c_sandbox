use std::fs::File;
// use std::io::Write;
use std::io::{self, BufRead, BufWriter, Cursor, Read, Write};
// use std::io::{;

fn input_example() {
    // Stdin/Stdout
    // Stdin — стандартный ввод

    let stdin = io::stdin();
    let mut input = String::new();

    println!("Введите ваше имя:");
    stdin.read_line(&mut input).unwrap(); // читаем строку
    let name = input.trim(); // убираем лишние пробелы и переносы
    println!("Привет, {}!", name);

    // Stdout — стандартный вывод
    print!("Введите число: ");
    io::stdout().flush().unwrap(); // показываем приглашение сразу

    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let number: i32 = input.trim().parse().unwrap();

    println!("Вы ввели число {}", number);
}

pub fn fake_main() {
    let mut f = File::create("data.csv").unwrap();
    writeln!(f, "John,100").unwrap();
    writeln!(f, "Alice,200").unwrap();
    // Каждая строка — отдельный системный вызов write.

    // BufWriter (BufWrite)
    let f = File::create("data_buff.csv").unwrap();
    let mut writer = BufWriter::new(f);

    writeln!(writer, "John,100").unwrap(); // пока в буфере
    writeln!(writer, "Alice,200").unwrap(); // пока в буфере
    writer.flush().unwrap(); // всё записано в файл однойCursor операцией 
    // input_example();

    // Использование Cursor для работы с памятью
    // Создаём буфер в памяти
    let mut buffer = Cursor::new(Vec::new());

    // Записываем данные
    write!(buffer, "Hello, world!").unwrap();

    // Возвращаем курсор в начало, чтобы читать
    buffer.set_position(0);

    // Читаем данные обратно
    let mut output = String::new();
    // buffer.read_to_string(&mut output).unwrap();
    buffer.read_to_string(&mut output).unwrap();

    println!("Считано из памяти: {}", output);
}
