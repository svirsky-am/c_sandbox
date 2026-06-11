use sha2::{Sha256, Digest};

// Концептуальный пример (псевдокод)

// Каждый "тик" PoH — это хеш от предыдущего хеша
struct Poh {
    current_hash: [u8; 32],
    tick_count: u64,
}

impl Poh {
    fn tick(&mut self) {
        // Новый хеш = SHA256(предыдущий_хеш + счётчик)
        let mut data = Vec::new();
        data.extend_from_slice(&self.current_hash);
        data.extend_from_slice(&self.tick_count.to_le_bytes());
        
        self.current_hash =  Sha256::digest(&data).into();
        self.tick_count += 1;
    }
}

// Пример работы
fn main() {
    let mut poh = Poh {
        current_hash: [0; 32], // Начальный хеш
        tick_count: 0,
    };
    
    // Создаём временную шкалу
    for i in 0..5 {
        poh.tick();
        println!("Tick {}: {:?}", i, &poh.current_hash[..4]);
    }
}