use std::fmt;
use std::fs;
use std::num::ParseIntError;

#[derive(Debug)]
pub enum MyConfigError {
    FileRead(std::io::Error),
    Parse(ParseIntError),
    InvalidValue {
        field: String,
        value: String,
        expected: String,
    },
    Custom(String),
    MissingField(String),
}

impl fmt::Display for MyConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MyConfigError::FileRead(err) => {
                write!(f, "Не удалось прочитать файл конфигурации: {}", err)
            }
            MyConfigError::Parse(err) => {
                write!(f, "Ошибка парсинга числа в конфиге: {}", err)
            }
            MyConfigError::Custom(err) => {
                write!(f, "Ошибка: {}", err)
            }
            MyConfigError::MissingField(field) => {
                write!(f, "Отсутствует обязательное поле: {}", field)
            }
            MyConfigError::InvalidValue {
                field,
                value,
                expected,
            } => {
                write!(
                    f,
                    "Неверное значение '{}' для поля '{}', ожидается: {}",
                    value, field, expected
                )
            }
        }
    }
}

use std::error::Error;

impl Error for MyConfigError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            MyConfigError::FileRead(err) => Some(err),
            MyConfigError::Parse(err) => Some(err),
            MyConfigError::Custom(_) => None,
            MyConfigError::MissingField(_) => None,
            MyConfigError::InvalidValue { .. } => None,
        }
    }
}

// Структура конфигурации
#[derive(Debug)]
struct Config {
    port: u16,
    host: String,
    debug: bool,
}

// Функция загрузки конфига с нашей ошибкой MyConfigError
fn load_config(filename: &str) -> Result<Config, MyConfigError> {
    // Читаем файл
    let content = fs::read_to_string(filename).map_err(MyConfigError::FileRead)?;

    let mut port = None;
    let mut host = None;
    let mut debug = None;

    // Простой парсер key=value
    for line in content.lines() {
        let line = line.trim();
        // Игнорируем пустые линии и комментарии
        if line.is_empty() || line.starts_with('#') {
            continue;
        }

        let parts: Vec<&str> = line.split('=').collect();
        if parts.len() != 2 {
            continue;
        }

        let key = parts[0].trim();
        let value = parts[1].trim();

        match key {
            "port" => {
                port = Some(value.parse::<u16>().map_err(MyConfigError::Parse)?);
            }
            "host" => {
                host = Some(value.to_string());
            }
            "debug" => {
                debug = Some(match value {
                    "true" => true,
                    "false" => false,
                    _ => {
                        return Err(MyConfigError::InvalidValue {
                            field: "debug".to_string(),
                            value: value.to_string(),
                            expected: "true или false".to_string(),
                        });
                    }
                });
            }
            _ => {} // Игнорируем неизвестные поля
        }
    }

    // Проверяем обязательные поля
    let port = port.ok_or_else(|| MyConfigError::MissingField("port".to_string()))?;
    let host = host.ok_or_else(|| MyConfigError::MissingField("host".to_string()))?;

    // Проверяем необязательные поля
    let debug = debug.unwrap_or(false); // если None(отсутствует), то значение debug = false 

    Ok(Config { port, host, debug })
}

fn main() {
    match load_config("app.conf") {
        Ok(config) => {
            println!("Конфиг загружен: {:?}", config);
        }
        Err(err) => {
            eprintln!("Ошибка: {}", err);

            // Показываем цепочку ошибок
            let mut source = err.source();
            while let Some(err) = source {
                eprintln!("  Вызвано: {}", err);
                source = err.source();
            }
        }
    }
}
