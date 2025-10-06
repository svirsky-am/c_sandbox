#![allow(dead_code)]
#![allow(unused_variables)]

mod borrowing;
mod cicles;
mod closure;
mod closure_fn_threts;
mod data_types;
mod display_debug;
mod expresions;
mod fn_errors;
mod functions;
mod generics;
mod result_option;

use data_types::example_data_types;
// use expresions::fake_main;

fn main() {
    example_data_types();
    functions::greet();

    functions::print_coordinates(-1, 4);

    let is_exact_division = functions::is_divisible(10, 3);

    let temperature = functions::celsius_to_fahrenheit(23.0);
    expresions::fake_main();
    cicles::fake_main();
    closure::fake_main();
    borrowing::fake_main();
    generics::fake_main();
    display_debug::fake_main();

    closure_fn_threts::fake_main();
    result_option::fake_main();
    fn_errors::fake_main();
}
