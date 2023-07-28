extern crate rand;
use std::io;
use rand::Rng;
fn main(){  
    let mut rng = rand::thread_rng();
    let numero_aleatorio: i32 = rng.gen_range(1..=5);

    loop {
        println!("De seu palpite");    
        let mut palpite: String = String::new();
        io::stdin().read_line(&mut palpite)
        .expect("Falha ao ler a entrada");
    
            let palpite: i32 = palpite.trim().parse()
            .expect("Escolha inválida. Por favor, insira um número válido.");
    
        if palpite == numero_aleatorio {
            println!("acertou");
            break;
        }else if palpite > numero_aleatorio  {
            print!("voce colocou um valor maior \n")
        }else if palpite < numero_aleatorio {
            println!("voce colocou um numero menor")
         }else {
            println!("fez coisa errada hein meu nobre")
        }
    }

}