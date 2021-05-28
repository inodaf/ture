pub fn to_celsius(fahrenheit: i32) -> i32 {
    (fahrenheit - 32) * 5 / 9
}

pub fn to_fahrenheit(celsius: i32) -> i32 {
    celsius * 9 / 5 + 32
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_correctly_converts_fahrenheit_to_celsius() {
        assert_eq!(to_celsius(46), 7)
    }

    #[test]
    fn it_correctly_converts_celsius_to_fahrenheit() {
        assert_eq!(to_fahrenheit(6), 42)
    }

    #[test]
    fn it_handles_negative_values_for_fahrenheit_conversion() {
        assert_eq!(to_fahrenheit(-50), -58);
    }

    #[test]
    fn it_handles_negative_values_for_celsius_conversion() {
        assert_eq!(to_celsius(14), -10);
    }
}
