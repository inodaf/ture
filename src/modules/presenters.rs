const MESSAGES: [&str; 4] = [
    "🌨  Low temperature ahead",
    "🌥  Normal temperature ahead",
    "⛅️  Medium temperature ahead",
    "☀️  High temperature ahead",
];

pub fn print_celsius(celsius: i32) -> &'static str {
    let message: &str = if celsius <= 16 {
        MESSAGES[0]
    } else if celsius >= 17 && celsius <= 26 {
        MESSAGES[1]
    } else if celsius >= 27 && celsius <= 33 {
        MESSAGES[2]
    } else {
        MESSAGES[3]
    };

    message
}

pub fn print_fahrenheint(fahrenheit: i32) -> &'static str {
    let message: &str = if fahrenheit <= 59 {
        MESSAGES[0]
    } else if fahrenheit >= 60 && fahrenheit <= 77 {
        MESSAGES[1]
    } else if fahrenheit >= 78 && fahrenheit <= 90 {
        MESSAGES[2]
    } else {
        MESSAGES[3]
    };

    message
}

pub fn print_conversion(printer: fn(i32) -> &'static str, value: i32) {
    println!("{}", printer(value))
}
