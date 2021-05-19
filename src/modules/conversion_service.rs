pub fn to_celsius(fahrenheit: i32) -> i32 {
  (fahrenheit - 32) * 5 / 9
}

pub fn to_fahrenheit(celsius: i32) -> i32 {
  celsius * 9 / 5 + 32
}