fn my_thread_fn(label: &str) -> usize {
    let mut total = 0usize;
    for i in 0..5 {
        println!("my_thread_fn({}): {}", label, i);
        std::thread::sleep(std::time::Duration::from_millis(500));
        total += 1;
    }
    total
}

pub fn main_example_single_thred() {
    println!("main thread begins");
    let label = String::from("hello form main_example_single_thred");
    let thread = std::thread::spawn(move || my_thread_fn(&label));
    let total = thread.join().unwrap();
    println!("main thread ends, other thread sleeped {} times", total);
} 

pub fn main() {
    println!("Hello, world!");
    println!("main thread begins");
    let mut label1 = String::from("hello");
    let mut label2 = String::from("world");
    let (res1, res2) = std::thread::scope(
                     |scope| {
                         let r1 = scope.spawn(|| my_thread_fn(&label1));
                         let r2 = scope.spawn(|| my_thread_fn(&label2));
                         // нельзя обратиться по &mut self, когда выше есть просто &
                         //label1.clear();
                         //label2.clear();
                         println!("from main thread");
                         (r1.join().unwrap(), r2.join().unwrap())
                     }
                 );
    println!("main thread ends, other thread sleeped {:?} times", (res1, res2));
} 