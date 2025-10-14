use std::collections::BTreeSet;
use std::collections::HashSet;

fn is_not_odd(a: &u32) -> bool {
    a % 2 != 0
}

fn is_even(n: &u32) -> bool {
    n % 2 == 0
}

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
struct Transaction {
    id: u128,
    wallet_id: u128,
    sum: i64,
}
impl From<(u128, u128, i64)> for Transaction {
    fn from((id, wallet_id, sum): (u128, u128, i64)) -> Self {
        Self { id, wallet_id, sum }
    }
}
struct Wallet {
    id: u128,
    ballance: u64,
    latest_history: Vec<Transaction>,
}
impl Wallet {
    // update принимает транзакции, возвращает количество транзакций, которые относились к этому кошельку
    fn update(
        &mut self,
        transactions: impl IntoIterator<Item = Transaction, IntoIter: Clone>,
    ) -> usize {
        let filtered = transactions.into_iter().filter(|t| {
            t.wallet_id == self.id && self.latest_history.iter().all(|hist| hist.id != t.id)
        });
        let count_usize = filtered.clone().count();
        self.ballance += filtered.clone().map(|t| t.sum).sum::<i64>() as u64;
        let filtered_transactions_vec = filtered.collect::<Vec<_>>();
        println!(
            "Updated with {} transactions: {:?}",
            count_usize, filtered_transactions_vec
        );
        self.latest_history.extend(filtered_transactions_vec);
        count_usize
    }
}

pub fn fake_main() {
    // Методы all() и any()
    let mut nums = vec![2u32, 4, 6];
    // Можно передавать замыкание,..
    let mut result = nums.iter().any(is_not_odd);
    let is_logged_in = true;
    // assert!(, "{}", false);
    assert!(nums.iter().all(|&n| n % 2 == 0) == true);
    // ..а можно - указатель на функцию
    assert!(nums.iter().all(is_not_odd) == false);
    assert!(nums.iter().any(is_not_odd) == false);
    assert!(nums.iter().all(is_even) == true);
    assert!(nums.iter().any(is_even) == true);
    println!("Result is withot 3: {}", result); // Output: False 

    nums.insert(2, 3u32);
    result = nums.iter().any(is_not_odd);
    assert!(nums.iter().all(|&n| n % 2 == 0) == false);
    assert!(nums.iter().all(is_not_odd) == false);
    assert!(nums.iter().any(is_not_odd) == true);
    assert!(nums.iter().all(is_even) == false);
    assert!(nums.iter().any(is_even) == true);
    println!("Result after adding 3: {}", result); // Output: True
    // Адаптеры map(), enumerate() и метод collect()

    let mut nums = vec![0, 5, 10, 15];
    let mut into_x10 = nums.iter().map(|n| n * 10);
    assert_eq!(into_x10.next(), Some(0));
    assert_eq!(into_x10.next(), Some(50));
    assert_eq!(into_x10.next(), Some(100));
    assert_eq!(into_x10.next(), Some(150));
    assert_eq!(into_x10.next(), None);

    let mut into_enumerated = nums.iter().enumerate();
    assert_eq!(into_enumerated.next(), Some((0, &0)));
    assert_eq!(into_enumerated.next(), Some((1, &5)));
    assert_eq!(into_enumerated.next(), Some((2, &10)));

    let collected = nums.iter().map(|n| n * 10).enumerate().collect::<Vec<_>>();
    assert_eq!(collected, vec![(0, 0), (1, 50), (2, 100), (3, 150)]);

    let kv = [("key0", "value0"), ("key1", "value1")];

    let collected = kv.into_iter().collect::<std::collections::HashMap<_, _>>();
    assert_eq!(
        collected,
        std::collections::HashMap::from([("key0", "value0"), ("key1", "value1")])
    );

    // Метод count(), адаптеры chain() и zip(), метод unzip()

    let ordinary_transactions = vec![0, 1, 2];
    let promotion_transactions = BTreeSet::from([10, 20]);
    let all_transactions: Vec<_> = ordinary_transactions
        .into_iter()
        .chain(promotion_transactions.into_iter())
        .collect();

    assert_eq!(all_transactions.len(), 5);
    let mut iter_all_transactions = all_transactions.into_iter();
    assert_eq!(iter_all_transactions.next(), Some(0));
    assert_eq!(iter_all_transactions.next(), Some(1));
    assert_eq!(iter_all_transactions.next(), Some(2));
    assert_eq!(iter_all_transactions.next(), Some(10));
    assert_eq!(iter_all_transactions.next(), Some(20));
    assert_eq!(iter_all_transactions.next(), None);

    let users_queue = ["user2", "user1", "user2"];
    let operations_queue = vec![200, 100, 222];
    let zipped = users_queue.into_iter().zip(operations_queue.into_iter());
    let mut to_show = zipped.clone();
    assert_eq!(to_show.next(), Some(("user2", 200)));
    assert_eq!(to_show.next(), Some(("user1", 100)));
    assert_eq!(to_show.next(), Some(("user2", 222)));
    assert_eq!(to_show.next(), None);

    let (unique_users, operations): (BTreeSet<_>, Vec<_>) = zipped.unzip();
    assert_eq!(unique_users, BTreeSet::from(["user1", "user2"]));
    assert_eq!(operations, vec![200, 100, 222]);

    // Адаптеры skip(M), take(N) и step_by(K)
    let decision_weights = [0, 1, 2, 3, 4].into_iter();
    let mut larges_weights = decision_weights.clone().skip(3);
    let mut smallest_weights = decision_weights.clone().take(3);
    let mut more_different_weights = decision_weights.clone().step_by(2);

    assert_eq!(larges_weights.next(), Some(3));
    assert_eq!(larges_weights.next(), Some(4));
    assert_eq!(larges_weights.next(), None);

    assert_eq!(smallest_weights.next(), Some(0));
    assert_eq!(smallest_weights.next(), Some(1));
    assert_eq!(smallest_weights.next(), Some(2));
    assert_eq!(smallest_weights.next(), None);

    assert_eq!(more_different_weights.next(), Some(0));
    assert_eq!(more_different_weights.next(), Some(2));
    assert_eq!(more_different_weights.next(), Some(4));
    dbg!(more_different_weights.next());
    // assert_eq!(more_dirrefent_weights.next(), None);

    // Методы sum(), product(), max(), min()
    let nums = [3u32, 1, 4, 2];
    // let nums_iter = nums.into_iter();
    assert_eq!(nums.into_iter().sum::<u32>(), 10);
    assert_eq!(nums.into_iter().product::<u32>(), 24);
    assert_eq!(nums.into_iter().max(), Some(4));
    assert_eq!(nums.into_iter().min(), Some(1u32));
    let ages_and_names = [("Bob", 33), ("Alice", 25), ("Eva", 30)];
    // cmp() - метод чисел, который мы можем использовать, чтобы не писать операцию с std::cmp::Ordering самим
    assert_eq!(
        ages_and_names
            .into_iter()
            .max_by(|left, right| left.0.cmp(right.0))
            .unwrap(),
        ("Eva", 30)
    );
    assert_eq!(
        ages_and_names
            .into_iter()
            .max_by_key(|element| element.1)
            .unwrap(),
        ("Bob", 33)
    );

    // Адаптеры cloned(), copied(), cycle() и fold()
    let last_users_did_ops = vec![
        "Alice".to_string(),
        "Bob".into(),
        "Alice".into(),
        "Eva".into(),
    ];
    let unique = last_users_did_ops.iter().cloned().collect::<HashSet<_>>();
    let last_currencies = vec!["BTC", "ETH", "XDG", "BTC"];
    let unique = last_currencies.iter().copied().collect::<HashSet<_>>();

    let transaction_ops = ["pre_check", "apply", "post_check"];
    let mut user_state_cursor = transaction_ops.into_iter().cycle();
    assert_eq!(user_state_cursor.next(), Some("pre_check"));
    assert_eq!(user_state_cursor.next(), Some("apply"));
    assert_eq!(user_state_cursor.next(), Some("post_check"));
    assert_eq!(user_state_cursor.next(), Some("pre_check"));
    assert_eq!(user_state_cursor.next(), Some("apply"));

    let payed_sums = [100u32, 200, 50, 300];
    let platform_bonus_points = payed_sums
        .iter()
        .fold(0, |next, collected| collected + next * 3 / 2 + 2);
    // .fold(0.0, |acc, &amount| acc + amount as f64 * 1.5 + 2.0) as u32;
    println!("collected bonus for purchases: {}", platform_bonus_points);

    // Адаптер filter() и методы find()/position()
    let numbers = [0i32, 1, 2, 3, 4, 5, 21];
    let mut odd = numbers.clone().into_iter().filter(|num| 0 == num % 2);
    let mut odd_backup = odd.clone();
    let mut odd_backup_alt = numbers.into_iter();
    // let mut odd_backup2 = odd.clone();
    assert_eq!(odd.next(), Some(0));
    assert_eq!(odd.next(), Some(2));
    assert_eq!(odd.next(), Some(4));
    assert_eq!(odd.next(), None);
    // position() учитывает только 'последний' итератор!
    assert_eq!(odd_backup.clone().position(|num| num == 4), Some(2));
    for test_num in numbers.iter().filter(|(item)| *item % 7 == 0).cloned() {
        println!("test_num {}", test_num);
    }

    // dbg!(odd_backup_alt.clone().find(|num| num % 21 == 0));
    assert_eq!(
        odd_backup_alt.clone().find(|num| num % 7 == 0 && *num != 0),
        Some(21)
    );

    let percents = [10i32, 15, 20, 40, 60, 80];
    // find_map позволяет не только проверять истинность, но и сразу преобразовывать результат
    // Дополнительно, здесь мы заиспользовали метод bool::then_some(V), который при истинности
    // bool возвращает Some(V), и None иначе
    // let larger_fraction = percents
    //     .iter()
    //     .find_map(|v| (*v > 50).then_some(*v as f32).unwrap() / 100f32);
    let larger_fraction = percents
        .iter()
        .find_map(|&v| (v > 50).then_some(v as f32 / 100.0));
    assert_eq!(larger_fraction, Some(0.6));

    // let comparisons = [1.22, 1., 1.15, 2., 1., 0.8, 1.4];
    let comparisons = [1.22, 1., 1.15, 2., 1., 0.8, 1.4];
    // let mut differences = comparisons
    //     .iter()
    //     .filter_map(|f| (*f != 1.).then_some(((f - 1.) * 100.0) as i32));
    let mut differences = comparisons
        .iter()
        .filter_map(|&f| (f != 1.0).then_some(((f - 1.0) * 100.0) as i32));
    assert_eq!(differences.next(), Some(21));
    assert_eq!(differences.next(), Some(14));
    assert_eq!(differences.next(), Some(100));
    assert_eq!(differences.next(), Some(-19));
    assert_eq!(differences.next(), Some(39));
    assert_eq!(differences.next(), None);

    // for in и итераторы

    let mut array = [0i32, 1, 2, 3];
    for v in &array {
        // как если бы мы вызвали `for v in array.iter()`
        println!("{}", v); // array всё ещё доступен, элементы через v изменять нельзя
    }
    println!("for v in &mut array ");
    for v in &mut array {
        // как если бы мы вызвали `for v in array.iter_mut()`
        *v *= 2;
        println!("{}", v);
    }
    println!("for mut v in array");
    for mut v in array {
        // array перемещён - теперь мутабельность зависит от наличия mut перед v
        v *= 3;
        println!("{}", v);
    }

    let mut users_and_ballance = [
        ("Alice".to_string(), 100u32),
        ("Bob".into(), 1500),
        ("Eva".into(), 1300),
    ];
    let mut premium_users: Vec<String> = Vec::new();
    for premium_user in users_and_ballance
        .iter()
        .filter_map(|(user, ballance)| (*ballance > 1000u32).then_some(user))
        .cloned()
    {
        println!("premium user: {}", premium_user);
        premium_users.push(premium_user);
    }
    println!(
        "premium users: {}, all users: {}",
        premium_users.len(),
        users_and_ballance.len()
    );

    let last_visitors = vec!["Alice", "Bob", "Alice", "Eva", "John", "Alice"];
    let mut unique_visitors = HashSet::from(["Victor"]);
    // Разные коллекции - не проблема! Ведь элементы в итераторах - одного и те же типа
    unique_visitors.extend(last_visitors);
    assert_eq!(unique_visitors.len(), 5);

    // Задача

    // допустим, latest_history был загружен из файла
    let latest_history: Vec<Transaction> = vec![
        (0, 32, 100).into(),
        (1, 32, -50).into(),
        (4, 32, 150).into(),
    ];
    let mut wallet = Wallet {
        id: 32,
        ballance: 100,
        latest_history,
    };

    // допустим, загрузили последние транзакции из сети
    let updates1: Vec<Transaction> = vec![
        (2, 10, -500).into(),
        (3, 21, 10000).into(),
        (4, 32, 150).into(),
        (5, 32, 50).into(),
        (6, 10, 1000).into(),
        (8, 32, 50).into(),
    ];
    let updates2 = std::collections::VecDeque::<Transaction>::from([
        Transaction::from((4, 32, 150)),
        (7, 32, 100).into(),
        (10, 10, 100).into(),
        (11, 32, -50).into(),
    ]);

    println!("Got transactions:");
    for update in updates1.iter().chain(updates2.iter()) {
        // итерация по транзакциям из updates1 и updates2
        println!("{:?}", update)
    }

    assert_eq!(wallet.update(updates1), 2);
    assert_eq!(wallet.ballance, 200);
    assert_eq!(wallet.update(updates2), 2);
    assert_eq!(wallet.ballance, 250);
}
