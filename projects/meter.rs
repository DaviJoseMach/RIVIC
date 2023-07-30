extern crate rand;

use rand::Rng;
use std::io;

fn main() {
    println!("Digite o nome da cidade:");

    let mut city = String::new();
    io::stdin().read_line(&mut city).expect("Falha ao ler a entrada");

    let mut atual = rand::thread_rng();
    let temperaturaa: i128 = atual.gen_range(15..=35);

    let mut maxima = rand::thread_rng();
    let temperaturamaxima: i128 = maxima.gen_range(35..=42);

    let mut minima = rand::thread_rng();
    let temperaturaminima: i128 = minima.gen_range(0..=15);

    println!(
        "A cidade {}, está com {} Celsius, com máxima de {}, e mínima de {}", city.trim(),
        temperaturaa,
        temperaturamaxima,
        temperaturaminima
    );
}
