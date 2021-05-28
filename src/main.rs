use std::env;

mod modules;

fn handle_celsius(degrees: i32) {
    let conversion: i32 = modules::conversions::to_fahrenheit(degrees);
    modules::presenters::print_conversion(modules::presenters::print_fahrenheint, conversion)
}

fn handle_fahrenheit(degrees: i32) {
    let conversion: i32 = modules::conversions::to_celsius(degrees);
    modules::presenters::print_conversion(modules::presenters::print_celsius, conversion);
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let unit: &str = &args[1].to_lowercase();
    let degrees: i32 = match args[2].trim().parse() {
        Ok(num) => num,
        Err(_) => panic!(),
    };

    match unit {
        "c" => handle_celsius(degrees),
        "f" => handle_fahrenheit(degrees),
        _ => panic!(),
    };
}
