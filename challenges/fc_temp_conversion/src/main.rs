use std::io::{self, Write};

fn main() {
    print!("Enter temperature in Fahrenheit: ");
    io::stdout().flush().unwrap();

    let mut temperature_fahrenheit = String::new();
    io::stdin()
        .read_line(&mut temperature_fahrenheit)
        .expect("Invalid input");

    let temperature_fahrenheit: f64 = temperature_fahrenheit
                                        .trim()
                                        .parse()
                                        .expect("Please type a number");

    println!("Temperature in Celsius: {}", convert_f_to_c(temperature_fahrenheit));
}

fn convert_f_to_c(temp: f64) -> f64 {
    return (temp - 32.0) * (5.0/9.0);
}
