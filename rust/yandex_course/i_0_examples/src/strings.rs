use std::collections::HashSet;
use std::collections::hash_map::Entry;
use std::collections::{BTreeSet, HashMap};

// Для использования своего типа нужно реализовать Hash
#[derive(Hash, PartialEq, Eq)]
struct Transaction {
    country: &'static str,
    id: u32,
}

fn make_key(word: &str) -> [usize; 128] {
    let mut key = [0usize; 128];
    for c in word.chars() {
        key[c as usize] += 1;
    }
    return key;
}

pub fn fake_main() {
    // (код - вставлять в main())
    // Перевод литерала времени компиляции str в контейнер String
    let mut s1 = "hello, world!".to_string();
    // Вставка в _байтовую_ позицию 0 новой строчки
    s1.insert_str(0, "Super ");
    s1.pop();
    assert_eq!(s1, "Super hello, world");
    // 22 - в байтах! Будет паника, если попасть на 'середину' UTF-8 символа
    s1.replace_range(13.., "student");
    s1.push_str(" of Practicum");
    assert_eq!(s1, "Super hello, student of Practicum");

    let mut s2 = String::from("Добрый день!");
    assert_eq!(s2.len(), 22); // длина - в байтах!
    // Когда речь заходит о не английском тексте - полагаться на индексы нельзя
    // Мы воспользуемся методом str find(), который вернёт позицию в байтах для символа
    // Под капотом find() ходит по итератору chars(). Итераторы рассмотрим в следующем уроке
    s2.truncate(s2.find(' ').unwrap());

    // Хеш-таблицы: HashSet и HashMap
    let mut countries = HashSet::new();
    countries.insert(Transaction {
        country: "ru",
        id: 1,
    });
    countries.insert(Transaction {
        country: "kz",
        id: 1,
    });
    assert_eq!(countries.len(), 2);

    let mut ballances: HashMap<_, _> = [("user0", 0), ("user1", 500)].into();
    match ballances.entry("user3") {
        Entry::Vacant(v) => {
            v.insert(10000);
        }
        Entry::Occupied(mut o) => {
            *o.get_mut() = 300;
        }
    }
    *ballances.get_mut("user1").unwrap() = 200;
    assert_eq!(ballances.get("user1").unwrap(), &200);

    // BinaryHeap
    let mut heap = std::collections::BinaryHeap::new();
    heap.push(3);
    heap.push(4);
    heap.push(1);
    heap.push(5);
    heap.push(2);
    heap.push(4);
    assert_eq!(heap.pop(), Some(5));
    assert_eq!(heap.pop(), Some(4));

    // Практическое задание: анаграммы
    let input = vec!["aba", "cab", "baa", "aab", "acb", "abc"];

    let mut groups: HashMap<[usize; 128], BTreeSet<&str>> = Default::default();

    for word in input.iter() {
        // word имеет тип &str

        let key = make_key(word);
        match groups.entry(key) {
            std::collections::hash_map::Entry::Vacant(v) => {
                let mut group = BTreeSet::new();
                group.insert(*word);
                v.insert(group);
            }
            std::collections::hash_map::Entry::Occupied(mut o) => {
                o.get_mut().insert(*word);
            }
        }
    }

    let mut count = 0;
    for (_, words) in groups.iter() {
        println!("{}:", count);
        for word in words.iter() {
            println!("    {}", word);
        }
        count += 1;
    }
}
