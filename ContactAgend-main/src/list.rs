//arquivo list
use serde_json;
use serde::{Serialize, Deserialize};
use std::fs;
#[derive(Debug, Deserialize, Serialize)]
struct Contato {
    nome: String,
    numero: String
}
pub fn lista() {
    let file_content = fs::read_to_string("agenda.json")
        .expect("Erro ao ler o arquivo JSON");

    let pessoas: Vec<Contato> = serde_json::from_str(&file_content)
        .expect("Erro ao desserializar o JSON");

    for pessoa in pessoas {
        println!("Nome: {}, Numero: {}", pessoa.nome, pessoa.numero);
    }
}