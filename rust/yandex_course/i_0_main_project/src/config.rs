use std::fmt;

pub const DEFAULT_COURSE_NAME: &str = "Rust для действующих разработчиков";

pub enum CourseCohort {
  Start,
  Base,
  Blockchain,
}

impl Default for CourseCohort {
  fn default() -> Self {
    // Когорта по умолчанию
    CourseCohort::Start
  }
}

impl fmt::Display for CourseCohort {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    let name = match self {
      CourseCohort::Start => "Переход с C/С++/Python",
      CourseCohort::Base => "Базовый курс",
      CourseCohort::Blockchain => "Погружение в блокчейн",
    };
    write!(f, "{}", name)
  }
}

pub struct CourseConfig {
  pub cohort: CourseCohort,
}

impl fmt::Display for CourseConfig {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{}. Когорта: {}", DEFAULT_COURSE_NAME, self.cohort)
  }
}

impl CourseConfig {
  pub fn new(cohort: CourseCohort) -> Self {
    Self { cohort }
  }

  pub fn duration(&self) -> u8 {
    match self.cohort {
      CourseCohort::Start => 16,
      CourseCohort::Base => 12,
      CourseCohort::Blockchain => 20,
    }
  }

  pub fn upgrade(&mut self) -> bool {
    match self.cohort {
      CourseCohort::Blockchain => false,
      _ => {
        self.cohort = CourseCohort::Blockchain;
        true
      }
    }
  }
}
