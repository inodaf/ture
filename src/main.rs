mod modules;

fn main() {
  let fahrenheit = modules::conversions::to_fahrenheit(32);
  let celsius = modules::conversions::to_celsius(46);

  println!("Celsius to Fahrenheit = {}F°", fahrenheit);
  println!("Fahrenheit to Celsius = {}C°", celsius);
}
