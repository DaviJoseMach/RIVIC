fn celsius_to_fahrenheit(celsius: f64) -> f64 {
    (celsius * 9.0 / 5.0) + 32.0
}

fn main() {
    let celsius_temperature: f64 = 25.0; // Substitua pelo valor da temperatura em Celsius que você deseja converter
    let fahrenheit_temperature = celsius_to_fahrenheit(celsius_temperature);
    println!("{} graus Celsius é igual a {} graus Fahrenheit.", celsius_temperature, fahrenheit_temperature);
}
