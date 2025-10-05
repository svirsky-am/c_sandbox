mod data_types;
mod expresions;
mod functions;

use data_types::example_data_types;
// use expresions::fake_main;

fn main() {
    example_data_types();
    functions::greet();

    functions::print_coordinates(-1, 4);

    let is_exact_division = functions::is_divisible(10, 3);

    let temperature = functions::celsius_to_fahrenheit(23.0);
    expresions::fake_main();
}
