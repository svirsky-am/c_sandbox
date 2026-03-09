mod example_threads;
mod example_separated_threds;
mod atomic_and_mutex;
mod exampe_object_safety_rule;
mod exampe_dyn_runtime;

fn main() {
    println!("Hello, world!");
    exampe_dyn_runtime::main();
    exampe_object_safety_rule::main();
    atomic_and_mutex::main();
    example_separated_threds::main();
    example_threads::main_example_single_thred();
    example_threads::main();


}
