// /// Модуль с функциями-помощниками
// mod helpers {

//      /// Публичная функция для "приветствия"
//      pub fn greet() {
//           println!("Hello, world!");
//      }
// }

mod config;
mod utils;

use config::CourseCohort;
use config::CourseConfig;
use config::DEFAULT_COURSE_NAME;
use time::OffsetDateTime;
use utils::helpers::greet;
use utils::helpers::show_progress;


fn main() {
  // Здороваемся
  greet();

  // Дополнительно сообщаем сегодняшнюю дату
  println!("Сегодня: {}", OffsetDateTime::now_utc().date());
  println!("Сегодня: {}", OffsetDateTime::now_utc().time());
  println!("Я прохожу курс: {}!", DEFAULT_COURSE_NAME);

  println!("Мой прогресс в текущем модуле:");
  show_progress(5, 15);

  // let course_config = CourseConfig::new(CourseCohort::Start);
  // let course = CourseConfig::default();
  let course_config = CourseConfig::new(CourseCohort::default());

  let course_duration = course_config.duration();
  // course_config::Display();
  println!("course_duration: {}!", &course_duration);
  println!("course_duration: {}!", &course_config);
  show_progress(9, 14);
}
