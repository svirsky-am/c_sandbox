// /// Модуль с функциями-помощниками
// mod helpers {

//      /// Публичная функция для "приветствия"
//      pub fn greet() {
//           println!("Hello, world!");
//      }
// }

mod config;
mod utils;

use config::DEFAULT_COURSE_NAME;
use time::OffsetDateTime;
use utils::helpers::greet;

fn main() {
  // Здороваемся
  greet();

  // Дополнительно сообщаем сегодняшнюю дату
  println!("Сегодня: {}", OffsetDateTime::now_utc().date());
  println!("Сегодня: {}", OffsetDateTime::now_utc().time());
  println!("Я прохожу курс: {}!", DEFAULT_COURSE_NAME);
}
