#![allow(dead_code)]
#![allow(unused_variables)]

fn swap<T>(a: T, b: T) -> (T, T) {
    (b, a)
}

trait Active {
    // Ассоциированный тип
    // Каждый тип, реализующий трейт, может сам решить,
    // как представлять своё состояние:
    // как строку, символ, число и т. д.
    type Status;

    // Ассоциированная константа
    // Тип устройства или сущности
    const ENTITY_TYPE: &'static str;

    // Методы

    // Возвращает текущее состояние, используя ассоциированный тип
    // (например, true, '🔴', "Active")

    fn status(&self) -> Self::Status;

    // Метод-флаг активности
    fn is_active(&self) -> bool;

    // Метод активации
    fn activate(&mut self);

    // Метод деактивации
    fn deactivate(&mut self);

    // Метод по умолчанию
    fn toggle(&mut self) {
        if self.is_active() {
            self.deactivate();
        } else {
            self.activate();
        }
    }
}

struct Sensor {
    active: bool,
}

impl Active for Sensor {
    type Status = char;

    const ENTITY_TYPE: &'static str = "SmartDevice";

    fn status(&self) -> Self::Status {
        if self.active { '🟢' } else { '🔴' }
    }

    fn is_active(&self) -> bool {
        self.active
    }

    fn activate(&mut self) {
        self.active = true;
    }

    fn deactivate(&mut self) {
        self.active = false;
    }
}

struct Feature {
    enabled: bool,
    active: bool,
}

impl Active for Feature {
    type Status = &'static str;

    const ENTITY_TYPE: &'static str = "Feature";

    fn status(&self) -> Self::Status {
        if self.active { "enabled" } else { "disabled" }
    }

    fn is_active(&self) -> bool {
        self.enabled
    }

    fn activate(&mut self) {
        self.enabled = true;
    }

    fn deactivate(&mut self) {
        self.enabled = false;
    }
}

// тоде работает
// fn start(component: &mut impl Active) {
fn start<T: Active>(component: &mut T) {
    component.activate();
}

#[derive(Default)]
struct Field<T> {
    value: T,
    is_valid: bool,
}

// alternative fof #[derive(Default)]

// impl<T: Default> Default for Field<T> {
//     fn default() -> Self {
//         Self {
//             value: T::default(),
//             is_valid: bool::default(),
//         }
//     }
// }

pub fn fake_main() {
    let field: Field<i32> = Field::default();

    // false (значение по умолчанию для bool)
    println!("{}", field.is_valid);
}
