fn main() {
    let temperature_fahrenheit = 69.0;
    
    println!("Temperature in Celsius: {}", convert_f_to_c(temperature_fahrenheit));
}

fn convert_f_to_c(temp: f64) -> f64 {
    return (temp - 32.0) * (5.0/9.0);
}
