fn call_fn_mut<F: FnMut()>(mut f: F) {
    f();
    f();
}
fn call_fn_once<F: FnOnce()>(f: F) {
    f();
}

fn add(x: i32, y: i32) -> i32 {
    x + y
}

fn multiply(x: i32, y: i32) -> i32 {
    x * y
}

// Функция принимает ссылку на функцию (fn type)
fn calculate_with_fn(op: fn(i32, i32) -> i32, a: i32, b: i32) -> i32 {
    op(a, b)
}

// Функция принимает любой Fn-трейт (более гибко)
fn calculate_with_trait<F>(op: F, a: i32, b: i32) -> i32
where
    F: Fn(i32, i32) -> i32,
{
    op(a, b)
}

fn execute_task<F>(task: F) -> String
where
    F: FnOnce() -> String,
{
    task() // Вызываем только один раз
}

fn repeat_action<F>(action: F, times: usize)
where
    F: Fn(), // Вызываем много раз
{
    for _ in 0..times {
        action();
    }
}

fn accumulate<F>(mut accumulator: F, values: Vec<i32>) -> i32
where
    F: FnMut(i32) -> i32,
{
    let mut result = 0;
    for value in values {
        result = accumulator(value);
    }
    result
}

// Дженерик функторы: T: FnOnce(A) -> B

// Дженерик функтор с конкретными типами
fn transform<F>(value: i32, func: F) -> String
where
    F: FnOnce(i32) -> String,
{
    func(value)
}

// Полностью дженерик функтор
fn apply<T, U, F>(value: T, func: F) -> U
where
    F: FnOnce(T) -> U,
{
    func(value)
}

// // F может быть замыканием, обычной функцией или любым типом с Fn
// fn apply<T, F>(value: T, func: F) -> T
// where
//     F: FnOnce(T) -> T, // <-- ограничение по Fn-трейту
// {
//     func(value) // вызываем как функцию
// }

struct MyMap<I, F> {
    iter: I,
    f: F,
}

impl<I, F, B> Iterator for MyMap<I, F>
where
    I: Iterator,
    F: FnMut(I::Item) -> B,
{
    type Item = B;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next().map(&mut self.f)
    }
}

fn make_adder(n: i32) -> impl Fn(i32) -> i32 {
    // ваш код здесь
    move |x: i32| -> i32 { n + x }
    // move |x| x + n
}

// fn transform<F>(value: i32, func: F) -> String
//   where
//       F: FnOnce(i32) -> String,
//   {
//       func(value)
//   }

pub fn fake_main() {
    let mut counter = 0;
    let mut increment = || {
        counter += 1;
        println!("Счётчик: {}", counter);
    };

    increment(); // Счётчик: 1
    increment(); // Счётчик: 2
    // Пример замыкания с указанием типов:
    let add_one: fn(i32) -> i32 = |x: i32| -> i32 { x + 1 };
    println!("{}", add_one(5)); // 6

    // Трейты Fn, FnOnce, FnMut

    let mut value = 1;

    // Замыкание, которое может изменять value
    let mut my_closure = || {
        println!("Inside closure: value = {}", value);
        value += 1;
    };

    println!("--- Используем как FnMut ---");
    call_fn_mut(&mut my_closure);
    // Первый вызов внутри call_fn_mut: value = 1 -> 2
    // Второй вызов внутри call_fn_mut: value = 2 -> 3
    println!("After FnMut: value = {}", value); // 3

    println!("--- Используем как FnOnce ---");
    let my_closure_once = move || {
        println!("Final value inside FnOnce: {}", value);
        // value в замыкании move = 3
    };
    call_fn_once(my_closure_once);
    // После вызова FnOnce, переменная value перемещена в замыкание
    // println!("{}", value);
    // нельзя использовать, так как move забрал владение

    let s = String::from("Rust");

    // Захват по ссылке (обычное замыкание)
    let borrow_closure = || {
        println!("Borrowed: {}", s); // использует ссылку на s
    };
    borrow_closure();
    println!("После borrow_closure: {}", s); // s всё ещё доступна

    // Захват по значению (move)
    let move_closure = move || {
        println!("Moved: {}", s); // берёт владение s
    };
    move_closure(); // println!("После move_closure: {}", s); // ❌ s больше недоступна

    // Пример передачи ссылки для функции

    // Оба способа работают с обычными функциями
    println!("{}", calculate_with_fn(add, 2, 3)); // 5
    println!("{}", calculate_with_trait(add, 2, 3)); // 5

    // Но только trait-версия работает с замыканиями
    let closure = |x, y| x + y + 1;

    // calculate_with_fn(closure, 2, 3);    //  Ошибка!
    println!("{}", calculate_with_trait(closure, 2, 3)); // 6

    // Передаём замыкание в дженерик функтор
    let result = apply(5, |x| x * 2); // 10

    // Или обычную функцию
    fn double(x: i32) -> i32 {
        x * 2
    }

    let nums = vec![1, 2, 3, 4, 5];

    // создаём свой map
    let doubled: Vec<_> = MyMap {
        iter: nums.into_iter(),
        f: |x| x * 2,
    }
    .collect();

    println!("{:?}", doubled); // [2, 4, 6, 8, 10]

    // let add_one: fn(i32) -> i32 = |x: i32| -> i32 { x + 1 };
    let add5 = make_adder(5);
    let add10 = make_adder(10);

    println!("{}", add5(3)); // 8
    println!("{}", add10(3)); // 13
}
