use std::env;

const C_RATIO: f64 = 0.5555555556;
const F_RATIO: f64 = 1.8;

fn main() {
    let args: Vec<String> = env::args().collect();

    let strategy = &args[1];
    let temp = &args[2];
    let temp: f64 = temp.parse().expect("Not a number!");

    if strategy == "celsius" {
        let celsius_temp = fahrenheit_to_celsius(temp);
        println!("Temp is {}°C", celsius_temp);
    } else {
        let fahrenheit_temp = celsius_to_fahrenheit(temp);
        println!("Temp is {}°F", fahrenheit_temp);
    }
}

fn celsius_to_fahrenheit(temp: f64) -> f64 {
    let converted_temp = temp * F_RATIO;
    converted_temp + 32.0
}

fn fahrenheit_to_celsius(temp: f64) -> f64 {
    let converted_temp = temp - 32.0;
    converted_temp * C_RATIO
}
