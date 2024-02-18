use std::fs;
use std::io;
use serde_json;
use serde::{Serialize, Deserialize};
use serde_json::{Value, Map};

#[derive(Debug, Serialize, Deserialize)]
struct Contato {
    nome: String,
    numero: String,
}

impl Contato {
    fn new(nome: String, numero: String) -> Self {
        Contato { nome, numero }
    }
}

pub fn excluir() {
    let file_content = fs::read_to_string("agenda.json")
        .expect("Erro ao ler o arquivo JSON");

    let mut agenda: Vec<Contato> = serde_json::from_str(&file_content)
        .expect("Erro ao fazer parsing do JSON");

    println!("Contatos na Agenda:");
    for (i, contato) in agenda.iter().enumerate() {
        println!("{}. Nome: {} - Número: {}", i + 1, contato.nome, contato.numero);
    }

    println!("Digite o número do contato que deseja remover:");
    let mut input = String::new();
    io::stdin().read_line(&mut input)
        .expect("Falha ao ler a entrada");

    let index = input.trim().parse::<usize>()
        .expect("Entrada inválida. Por favor, digite um número válido.");

    if index <= 0 || index > agenda.len() {
        println!("Índice inválido. Nenhum contato foi removido.");
    } else {
        agenda.remove(index - 1);
        let updated_json = serde_json::to_string(&agenda)
            .expect("Erro ao converter para JSON");

        fs::write("agenda.json", updated_json)
            .expect("Erro ao escrever no arquivo JSON");

        println!("Contato removido com sucesso!");
    }
}