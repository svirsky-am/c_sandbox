struct HolderBit {
    // В структурах и перечислениях все ссылки нужно аннотировать временем жизни
    // Так, здесь мы ограничили время жизни данных, на которые ссылается dataref,
    // концом жизни программы
    dataref: &'static str,
}

struct HolderWork<'a> {
    dataref: &'a str,
}

// Здесь явные лайфтаймы уже необходимы
// Мы говорим компилятору, как связаны лайфтаймы входа и выхода, а компилятор - проверяет
fn get<'a>(s1: &'a str, s2: &str) -> &'a str {
    s1
}
// В перечислениях могут быть ссылки -> в перечислениях используются лайфтаймы
enum Kind<'a> {
    Holder(&'a str),
}
// Лайфтаймы могут зависеть друг от друга!
// Например, мы можем для замены в Kind::Holder использовать ссылку с другим лайфтаймом
// - главное, чтобы значение, на которое она указывает, жило не меньше,
//   чем жило значение предыдущей ссылки
fn set<'shorter, 'sameOrLonger: 'shorter>(kind: &mut Kind<'shorter>, value: &'sameOrLonger str) {
    match kind {
        Kind::Holder(myref) => *myref = value,
    }
}

pub fn fake_main() {
    // Лайфтаймы
    //
    let owned = String::from("hello, world");
    // let holder = HolderBit { dataref: &owned }; // error[E0597]: `value` does not live long enough
    // `owned` dropped here while still borrowed

    let holder = HolderWork { dataref: &owned };

    let s1 = String::from("hello");
    let s2 = String::from("world");
    let some = get(&s1, &s2);
    println!("got: {}", some);
    let mut holder = Kind::Holder(&s1);
    set(&mut holder, &s2);
}
