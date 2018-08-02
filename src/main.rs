use std::env;

const VERSION: &'static str = env!("CARGO_PKG_VERSION");

const CELSIUS_NOTATION: &'static str = "°C";
const FAHRENHEIT_NOTATION: &'static str = "°F";

fn main() {
    let args: Vec<String> = env::args().collect();
    let direction: &str = &args[1];
    let value: f32 = match args[2].trim().parse() {
        Ok(num) => num,
        Err(_) => panic!("Invalid type for variable")
    };

    println!("Temp converter v. {}", VERSION);

    let mut notations = (CELSIUS_NOTATION, FAHRENHEIT_NOTATION);
    if direction == "f" {
        notations = (FAHRENHEIT_NOTATION, CELSIUS_NOTATION)
    }

    let converter: &Fn(f32) -> f32 = match direction {
        "c" => &celsius_to_fahrenheit,
        "f" => &fahrenheit_to_celsius,
        _ => return help()
    };
    println!("Result: {:0.2}{} => {:0.2}{}", value, notations.0, converter(value), notations.1);
}

fn help() {
    println!("usage:
temp_converter {{c|f}} <value>
    Convert celsius to fahrenheit and vise versa.");
}

fn celsius_to_fahrenheit(c_temp: f32) -> f32 {
    c_temp * 1.8 + 32.0
}

fn fahrenheit_to_celsius(f_temp: f32) -> f32 {
    (f_temp - 32.0) / 1.8
}
