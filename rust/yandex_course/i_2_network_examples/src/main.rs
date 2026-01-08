mod example_threads;
mod example_separated_threds;
mod atomic_and_mutex;


fn main() {
    println!("Hello, world!");
    atomic_and_mutex::main();
    example_separated_threds::main();
    example_threads::main_example_single_thred();
    example_threads::main();


}
