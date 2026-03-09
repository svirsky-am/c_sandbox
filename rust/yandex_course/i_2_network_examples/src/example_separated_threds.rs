use std::rc::Rc;

#[derive(Debug)]
struct HolderV1<'a> {
    r: &'a u32,
}
#[derive(Debug)]
struct HolderV2 {
    r: Rc<u32>,
}
// Нельзя
//fn makeV1() -> (u32, HolderV1<'a>) {
//    let value = 42u32;
//    let holder = HolderV1{r: &value};
//    (value, holder)
//}
// Можно и нужно!
fn makeV2() -> (Rc<u32>, HolderV2) {
    let value = Rc::new(42u32);
    let holder = HolderV2{r: value.clone()};
    (value, holder)
}

pub fn main() {
    println!("{:?}", makeV2());
} 