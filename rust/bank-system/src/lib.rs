pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

// fn main() {
//     let x = 0.1f64 + 0.2f64;
//     println!("{}", x); // Вывод: 0.30000000000000004
// } 

use std::collections::HashMap;

type Name = String;
type Balance = i64;

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
    
    pub fn add_user(&mut self, name: Name) -> Option<Balance> {
        if self.accounts.contains_key(&name) {
            None
        } else {
            self.accounts.insert(name, 0);
            Some(0)
        }
    }

    pub fn remove_user(&mut self, name: &Name) -> Option<Balance> {
        self.accounts.remove(name)
    }

    pub fn get_balance(&self, name: &Name) -> Option<Balance> {
        self.accounts.get(name).copied()
    }

    pub fn deposit(&mut self, name: &Name, amount: Balance) -> Result<(), String> {
        if let Some(balance) = self.accounts.get_mut(name) {
            *balance += amount;
            Ok(())
        } else {
            Err("Пользователь не найден".into())
        }
    }

    pub fn withdraw(&mut self, name: &Name, amount: Balance) -> Result<(), String> {
        if let Some(balance) = self.accounts.get_mut(name) {
            if *balance >= amount {
                *balance -= amount;
                Ok(())
            } else {
                Err("Недостаточно средств".into())
            }
        } else {
            Err("Пользователь не найден".into())
        }
    }
} 

#[cfg(test)]
mod tests {
    use super::*; // подключаем всё из родительского модуля

    #[test]
    fn test_new_storage_is_empty() {
        let bank = Storage::new();
        assert_eq!(bank.accounts.len(), 0);
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
} 



