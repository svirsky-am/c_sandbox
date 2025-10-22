use std::cell::RefCell;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Display;
use std::fs;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader, BufWriter, Cursor};
use std::ops::AddAssign;
use std::ops::SubAssign;
use std::path::Path;

pub type Name = String;
// type Balance = (i64);

#[derive(Debug)]
enum OpKind {
    // пополнить/потратить счёт
    Deposit(u64),
    Withdraw(u64),
    // закрыть аккаунт - все средства выведены
    CloseAccount,
} // вот и всё, никаких посторонних операций и данных! 

// #[derive(PartialEq, PartialOrd)]
// #[derive(PartialEq)]
// #[derive(Copy, Clone)]
// #[derive(Copy)]
#[derive(Debug)]
pub struct Balance {
    pub result: u64,
    last_ops: Vec<OpKind>,
}

impl AddAssign for Balance {
    fn add_assign(&mut self, other: Self) {
        self.result += other.result;
        // self.y += other.y;
    }
}

impl SubAssign for Balance {
    fn sub_assign(&mut self, other: Self) {
        self.result -= other.result;
    }
}

impl Display for Balance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // todo!()
        write!(f, "MyStruct value: {}", self.result)?;
        // Return Ok(()) on success
        Ok(())
    }
}

impl PartialOrd for Balance {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // Delegate the comparison to the inner field's partial_cmp method
        self.result.partial_cmp(&other.result)
        // match self.x.partial_cmp(&other.x) {
        //     Some(Ordering::Equal) => self.y.partial_cmp(&other.y),
        //     ordering => ordering,
        // }
    }
}

impl PartialEq for Balance {
    fn eq(&self, other: &Self) -> bool {
        // Books are considered equal if their ISBNs match, regardless of title or author
        self.result == other.result
    }
}

// обернём баланс в новый тип, чтобы можно было реализовывать метод
// (не для целого числа ведь метод добавлять, верно? :) )
// и запретим балансу опускаться ниже нуля

impl Balance {
    // Можно пойти ещё дальше и в качестве аргумента принимать любой тип,
    // который может итерироваться по OpKind, с помощью дженерика:
    // fn process<'a>(&mut self, impl IntoIterator<Item=&'a OpKind>) -> Vec<&'a OpKind>
    // Пробуйте, дерзайте!
    fn process<'a>(&mut self, ops: &[&'a OpKind]) -> Vec<&'a OpKind> {
        let mut remaining = ops.into_iter();
        let mut bad_ops = Vec::new();
        for op in &mut remaining {
            match op {
                OpKind::Deposit(value) => {
                    self.result += *value as u64;
                }
                OpKind::Withdraw(value) if self.result > *value as u64 => {
                    self.result -= *value as u64;
                }
                other @ _ => {
                    bad_ops.push(*other);
                    break;
                }
            }
        }
        bad_ops.extend(remaining);
        bad_ops
    }
}

pub struct Storage {
    accounts: HashMap<Name, Balance>,
}

impl Storage {
    /// Создаёт новый пустой банк
    pub fn new() -> Self {
        Storage {
            accounts: HashMap::new(),
        }
    }

    pub fn add_user(&mut self, name: Name) -> Option<u64> {
        let empty_balance = Balance {
            result: 0,
            last_ops: vec![
                OpKind::Deposit(5000),
                OpKind::Withdraw(500),
                OpKind::Withdraw(1000),
                OpKind::Withdraw(700),
            ],
        };
        // if self.accounts.contains_key(&name) {
        //     None
        // } else {
        //     self.accounts.insert(name, empty_balance)
        //     // Some(emptyBalance)
        // }
        // let cur_name = Box::new(name);
        let cur_name = RefCell::new(name);

        if let std::collections::hash_map::Entry::Vacant(e) =
            self.accounts.entry(cur_name.borrow().to_string())
        {
            e.insert(empty_balance);
            // None
            // let cur_name2 = cur_name.borrow().as_str();
            // cur_name2
            let cur_balance = self.accounts.get(&cur_name.borrow().to_string());
            // cur_balance
            Some(cur_balance.unwrap().result)
            // Some(0)
        } else {
            None
        }
    }

    pub fn remove_user(&mut self, name: &Name) -> Option<u64> {
        self.accounts.remove(name).map(|account| account.result)
    }

    pub fn get_balance(&self, name: &Name) -> Option<u64> {
        let cur_balance = self.accounts.get(name);
        // if (cur_balance){
        //     Some(cur_balance.unwrap().result)
        // } else  {

        // }
        match cur_balance {
            Some(n) => return Some(cur_balance.unwrap().result),
            None => return None,
        }

        // if Some(cur_balance) {
        //     Some(cur_balance.unwrap().result)
        // } else {
        //     None
        // }
    }

    pub fn deposit(&mut self, name: &Name, amount: u64) -> Result<(), String> {
        if let Some(balance) = self.accounts.get_mut(name) {
            balance.result = balance.result + amount;
            Ok(())
        } else {
            Err("Пользователь не найден".into())
        }
    }

    pub fn withdraw(&mut self, name: &Name, amount: u64) -> Result<(), String> {
        if let Some(balance) = self.accounts.get_mut(name) {
            // dbg!("{}", balance);

            // Ok(())
            if &balance.result >= &amount {
                // balance.result = balance.result - amount;
                balance.result -= amount;
                Ok(())
            } else {
                Err("Недостаточно средств".into())
            }
        } else {
            Err("Пользователь не найден".into())
        }
    }

    /// Загружает данные из CSV-файла или создаёт хранилище с дефолтными пользователями
    pub fn load_data(file: &str) -> Storage {
        let mut storage = Storage::new();

        // Проверяем, существует ли файл
        if Path::new(file).exists() {
            // Открываем файл
            let file = File::open(file).unwrap();

            // Оборачиваем файл в BufReader
            // BufReader читает данные блоками и хранит их в буфере,
            // поэтому построчное чтение (lines()) работает быстрее, чем читать по байту
            let reader = io::BufReader::new(file);

            // Читаем файл построчно
            for line in reader.lines().flatten() {
                // Разделяем строку по запятой: "Name,Balance"
                let parts: Vec<&str> = line.trim().split(',').collect();
                if parts.len() == 2 {
                    let name = parts[0].to_string();
                    // Пробуем преобразовать баланс из строки в число
                    let balance: u64 = parts[1].parse().unwrap_or(0);
                    // let new_balance = Balance {
                    //     result: balance,
                    //     last_ops: vec![OpKind::Deposit(balance)],
                    // };
                    // Добавляем пользователя и выставляем баланс
                    storage.add_user(name.clone());
                    let _ = storage.deposit(&name, balance);
                }
            }
        } else {
            // если файла нет, создаём пользователей с нуля
            for u in ["John", "Alice", "Bob", "Vasya"] {
                storage.add_user(u.to_string());
            }
        }

        storage
    }

    fn get_all(&self) -> Vec<(Name, u64)> {
        self.accounts
            .iter()
            .map(|(n, b)| (n.clone(), b.result))
            .collect()
    }

    // fn get_all(&self) -> Vec<(Name, u64)> {
    //     self.accounts
    //         .iter()
    //         .map(|(n, b)| (n.clone(), *b.result))
    //         .collect()
    // }

    /// Сохраняет текущее состояние Storage в CSV-файл
    pub fn save(&self, file: &str) {
        let mut data = String::new();

        // Собираем все данные в одну строку формата "Name,Balance"
        for (name, balance) in self.get_all() {
            data.push_str(&format!("{},{}\n", name, balance));
        }

        // Записываем в файл
        // Здесь мы не используем BufWriter, потому что сразу пишем всю строку целиком.
        fs::write(file, data).expect("Не удалось записать файл");
    }
}

#[test]
fn test_add_user() {
    let mut storage = Storage::new();
    assert_eq!(storage.add_user("Alice".to_string()), Some(0)); // новый пользователь
    assert_eq!(storage.add_user("Alice".to_string()), None); // уже существует
}

#[test]
fn test_remove_user() {
    let mut storage = Storage::new();
    storage.add_user("Bob".to_string());
    storage.deposit(&"Bob".to_string(), 100).unwrap();

    assert_eq!(storage.remove_user(&"Bob".to_string()), Some(100)); // удаляем и получаем баланс
    assert_eq!(storage.remove_user(&"Bob".to_string()), None); // второй раз — не найден
}

#[test]
fn test_deposit_and_withdraw() {
    let mut storage = Storage::new();
    storage.add_user("Charlie".to_string());

    // Пополнение
    assert!(storage.deposit(&"Charlie".to_string(), 200).is_ok());
    assert_eq!(storage.get_balance(&"Charlie".to_string()), Some(200));

    // Успешное снятие
    assert!(storage.withdraw(&"Charlie".to_string(), 150).is_ok());
    assert_eq!(storage.get_balance(&"Charlie".to_string()), Some(50));

    // Ошибка: недостаточно средств
    assert!(storage.withdraw(&"Charlie".to_string(), 100).is_err());
    assert_eq!(storage.get_balance(&"Charlie".to_string()), Some(50));
}

#[test]
fn test_nonexistent_user() {
    let mut storage = Storage::new();

    // Депозит несуществующему пользователю
    assert!(storage.deposit(&"Dana".to_string(), 100).is_err());

    // Снятие у несуществующего пользователя
    assert!(storage.withdraw(&"Dana".to_string(), 50).is_err());

    // Баланс у несуществующего пользователя
    assert_eq!(storage.get_balance(&"Dana".to_string()), None);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs::{self, File};
    use std::io::Write;

    #[test]
    fn test_load_data_existing_file() {
        let file_path = "test_load.csv";

        // Создаём файл с исходными данными
        let mut f = File::create(file_path).unwrap();
        writeln!(f, "John,100").unwrap();
        writeln!(f, "Alice,200").unwrap();
        writeln!(f, "Bob,50").unwrap();

        // Загружаем Storage
        let storage = Storage::load_data(file_path);

        assert_eq!(storage.get_balance(&"John".to_string()), Some(100));
        assert_eq!(storage.get_balance(&"Alice".to_string()), Some(200));
        assert_eq!(storage.get_balance(&"Bob".to_string()), Some(50));
        // Пользователь Vasya не добавлен в файле, поэтому None
        assert_eq!(storage.get_balance(&"Vasya".to_string()), None);

        // Удаляем тестовый файл
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_save_creates_file_with_correct_data() {
        let file_path = "test_save.csv";

        // Создаём Storage и добавляем пользователей
        let mut storage = Storage::new();
        storage.add_user("John".to_string());
        storage.add_user("Alice".to_string());
        storage.deposit(&"John".to_string(), 150).unwrap();
        storage.deposit(&"Alice".to_string(), 300).unwrap();

        // Сохраняем в файл
        storage.save(file_path);

        // Читаем файл обратно и проверяем содержимое
        let contents = fs::read_to_string(file_path).unwrap();
        let mut lines: Vec<&str> = contents.lines().collect();
        lines.sort(); // сортируем, так как get_all() может возвращать в любом порядке

        assert_eq!(lines, vec!["Alice,300", "John,150"]);

        // Удаляем тестовый файл
        fs::remove_file(file_path).unwrap();
    }

    #[test]
    fn test_load_data_existing_cursor() {
        // Создаём данные в памяти, как будто это CSV-файл
        let data = b"John,100\nAlice,200\nBob,50\n";
        let mut cursor = Cursor::new(&data[..]);

        // Читаем данные из Cursor
        let mut storage = Storage::new();
        let reader = BufReader::new(&mut cursor);
        for line in reader.lines() {
            let line = line.unwrap();
            let parts: Vec<&str> = line.trim().split(',').collect();
            if parts.len() == 2 {
                let name = parts[0].to_string();
                let balance: u64 = parts[1].parse().unwrap_or(0);
                storage.add_user(name.clone());
                storage.deposit(&name, balance).unwrap();
            }
        }

        assert_eq!(storage.get_balance(&"John".to_string()), Some(100));
        assert_eq!(storage.get_balance(&"Alice".to_string()), Some(200));
        assert_eq!(storage.get_balance(&"Bob".to_string()), Some(50));
        assert_eq!(storage.get_balance(&"Vasya".to_string()), None); // нет в данных
    }

    #[test]
    fn test_save_writes_to_cursor_correctly() {
        // Создаём Storage и добавляем пользователей
        let mut storage = Storage::new();
        storage.add_user("John".to_string());
        storage.add_user("Alice".to_string());
        storage.deposit(&"John".to_string(), 150).unwrap();
        storage.deposit(&"Alice".to_string(), 300).unwrap();

        // Сохраняем в память через BufWriter
        let buffer = Vec::new();
        let mut cursor = Cursor::new(buffer);
        {
            let mut writer = BufWriter::new(&mut cursor);
            for (name, balance) in storage.get_all() {
                writeln!(writer, "{},{}", name, balance).unwrap();
            }
            writer.flush().unwrap();
        }

        // Читаем обратно из памяти
        cursor.set_position(0);
        let mut lines: Vec<String> = BufReader::new(cursor).lines().map(|l| l.unwrap()).collect();
        lines.sort(); // сортируем для сравнения

        assert_eq!(lines, vec!["Alice,300", "John,150"]);
    }

    #[test]
    fn test_extend_balance() {
        let ops = [
            &OpKind::Deposit(32),
            &OpKind::Withdraw(64),
            &OpKind::CloseAccount,
        ];
        // let bad_ops = Balance(0).process(&ops);
        // assert_eq!(bad_ops.len(), 2);
        // println!("{:?}", bad_ops);
    }
}
