mod modules;

fn main() {
  let fahrenheit = modules::conversions::to_fahrenheit(32);
  let celsius = modules::conversions::to_celsius(46);

  modules::presenters::print_conversion(modules::presenters::print_celsius, celsius);
  modules::presenters::print_conversion(modules::presenters::print_fahrenheint, fahrenheit);

  println!("32 Celsius to Fahrenheit = {}F°", fahrenheit);
  println!("46 Fahrenheit to Celsius = {}C°", celsius);
}
