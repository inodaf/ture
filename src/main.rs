mod modules;

fn main() {
  let fahrenheit = modules::conversion_service::to_fahrenheit(32);
  let celsius = modules::conversion_service::to_celsius(19);

  println!("Celsius to Fahrenheit = {}", fahrenheit);
  println!("Fahrenheit to Celsius = {}", celsius);
}
