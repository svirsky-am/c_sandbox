use std::collections::BTreeSet;
// Самобалансирующиеся деревья со своими структурами - не проблема
// Но свой тип должен реализовывать Eq. А Eq зависит от PartialEq.
// Eq - про равенство. Для порядка в дереве аналогично нужны PartialOrd и Ord
#[derive(Ord, PartialOrd, PartialEq, Eq)]
struct Subscription {
    user: &'static str,
    service: &'static str,
}

use std::collections::BTreeMap;
use std::collections::btree_map::Entry;
struct User {
    email: &'static str,
    name: &'static str,
}

pub fn fake_main() {
    // У всех коллекций есть операция созданий 'пустой' коллекции через метод new()
    let mut currencies = Vec::new();
    // Для быстрого создания Vec в Rust есть макрос
    currencies = vec!["ru", "usd", "gbr", "chf"];
    // Vec реализует Deref от типа slice, как и array,
    // который мы прошли в уроке 'типы данных', а
    // значит, с ним сразу же можно работать как с массивом
    let top2: &[&str] = &currencies[..2];
    println!("Our most integrated currencies: [{}, {}]", top2[0], top2[1]);

    // Когда мы достаём элементы - возвращается Option<T>
    let Some(last_currency) = currencies.pop() else {
        panic!("no more currencies!")
    };
    println!("Dropped integration with '{}'", last_currency);
    // Добавление в конец
    currencies.push("cny");
    // Методами slice можно пользоваться напрямую, например,
    // получить первый элемент (вернётся Optional<T>)..
    println!("Added integration with '{}'", currencies.first().unwrap()); // .last() тоже есть
    // ..или разбить мутабельный срез
    //   на два непересекающихся мутабельных среза
    let (first3, last1) = currencies.split_at_mut(3);
    first3[2] = "eur";
    println!("First currency in least integrated: '{}'", last1[0]);
    // Добавление по индексу будет паниковать, если попытаться
    // вставить в позицию > .len()
    currencies.insert(2, "irr");
    // К слову, .len() поддерживается во всех коллекциях, но это не метод трейта
    println!("Current supported currencies count: {}", currencies.len());
    // .clear() тоже есть, во всех динамических (это которые в куче данные держат) коллекциях
    currencies.clear();
    assert!(currencies.is_empty());

    // При создании вектора можно выделить ему место заранее
    let mut preallocated_vec: Vec<u32> = Vec::new();
    preallocated_vec.reserve(10);
    println!("count of elements: {}", preallocated_vec.len());
    println!(
        "count of elements this Vec can already store: {}",
        preallocated_vec.capacity()
    );

    // VecDeque
    // Макрос вроде vec! есть только для вектора
    // Для остальных коллекций (и для вектора тоже)
    // можно использовать .into()
    let mut tasks: std::collections::VecDeque<_> = ["register", "pay", "pay", "refund"].into();
    tasks.push_back("withdraw");
    tasks.push_front("due-diligence");
    println!("the task after next: {}", tasks.get(1).unwrap());
    println!("total tasks count: {}", tasks.len());
    println!("undo last task: {}", tasks.pop_back().unwrap());
    println!("undo first task: {}", tasks.pop_front().unwrap());
    *tasks.front_mut().unwrap() = "prepare";
    assert_eq!(tasks.front().unwrap(), &"prepare");
    dbg!(tasks);

    // Самобалансирующиеся деревья: BTreeSet и BTreeMap
    let mut students_class_a: BTreeSet<&'static str> = ["Bob", "Alice"].into();
    let next_student = "Eva";
    // .insert() вернёт, был ли элемент Новым
    if students_class_a.insert(next_student) {
        println!("added New user: {}", next_student);
    } else {
        println!("added Existing user: {}", next_student);
    }
    dbg!(*students_class_a.first().unwrap());
    let mut students_class_b = BTreeSet::new();
    students_class_b.insert("Bob");
    for in_a_but_not_in_b in students_class_a.difference(&students_class_b) {
        println!("student '{}' is not in class b", next_student);
    }
    println!(
        "the first one, who'll go to the board is: {}",
        students_class_a.first().unwrap()
    );
    println!(
        "the last one, who'll go to the board is: {}",
        students_class_a.last().unwrap()
    );
    // Элементы отсортированы по возрастанию, в каком бы порядке ни добавлялись
    assert_eq!(students_class_a.first().unwrap(), &"Alice");
    students_class_a.remove("Bob");
    assert!(!students_class_a.contains("Bob"));
    students_class_a.pop_last();
    assert!(!students_class_a.contains("Eva"));
    students_class_a.pop_first();
    assert!(students_class_a.is_empty());

    let mut subscriptions = BTreeSet::new();
    subscriptions.insert(Subscription {
        user: "he",
        service: "practicum",
    });
    subscriptions.insert(Subscription {
        user: "she",
        service: "practicum",
    });
    subscriptions.insert(Subscription {
        user: "he",
        service: "other",
    });
    assert_eq!(subscriptions.len(), 3);

    dbg!(subscriptions.first().unwrap().user);
    dbg!(subscriptions.last().unwrap().service);

    // BTreeMap
    let mut users = BTreeMap::new();
    users.insert(
        "topruster",
        User {
            email: "rust@yandex.ru",
            name: "Victor",
        },
    );
    users.insert(
        "student",
        User {
            email: "edu@yandex.ru",
            name: "Tesla",
        },
    );
    assert!(users.contains_key("student"));

    // У BTreeMap есть интересный метод entry() - в него можно передать ключ
    // и сразу же вставить элемент, не выполняя поиск в коллекции повторно
    match users.entry("other") {
        Entry::Vacant(v) => {
            v.insert(User {
                email: "other@mail.ru",
                name: "Bin",
            });
        }
        Entry::Occupied(o) => {
            println!("existing email: {}", o.get().email);
        }
    }
    assert_eq!(users.len(), 3);
    assert!(users.get("student").is_some());
    users.get_mut("student").unwrap().email = "new-edu@yandex.ru";
    assert_eq!(users.get("student").unwrap().email, "new-edu@yandex.ru");
}
