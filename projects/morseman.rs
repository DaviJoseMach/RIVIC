use std::io;

fn main() {
    print!("| BEM VINDO AO MORSEMAN | \n \n");

    let mut name = String::new();
    let mut code = String::new();
    println!("Insira seu nome: ");
    io::stdin().read_line(&mut name).expect("Falha ao ler a entrada");
    println!("Seu codeName: ");
    io::stdin().read_line(&mut code).expect("Falha ao ler a entrada");
    println!("Bem vindo, senhor {} ({})", name.trim(), code.trim());
}
