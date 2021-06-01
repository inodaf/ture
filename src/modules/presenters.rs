const MESSAGES: [&str; 4] = [
    "🌨  Low temperature ahead",
    "🌥  Normal temperature ahead",
    "⛅️  Medium temperature ahead",
    "☀️  High temperature ahead",
];

pub fn print_celsius(&celsius: &i32, (unit, degrees): (&str, &i32)) {
    let message: &str = match celsius {
        t if t <= 16 => MESSAGES[0],
        t if t >= 17 && t <= 26 => MESSAGES[1],
        t if t >= 27 && t <= 33 => MESSAGES[3],
        _ => MESSAGES[3],
    };

    println!("{}{}° = {}C°, {}", degrees, unit, &celsius, message)
}

pub fn print_fahrenheint(&fahrenheit: &i32, (unit, degrees): (&str, &i32)) {
    let message: &str = match fahrenheit {
        t if t <= 59 => MESSAGES[0],
        t if t >= 60 && t <= 77 => MESSAGES[1],
        t if t >= 78 && t <= 90 => MESSAGES[2],
        _ => MESSAGES[3],
    };

    println!("{}{}° = {}F°, {}", degrees, unit, &fahrenheit, message)
}
