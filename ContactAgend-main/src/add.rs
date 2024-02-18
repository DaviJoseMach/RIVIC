use std::io;
use serde::{Serialize, Deserialize};
use serde_json;
#[derive(Serialize, Deserialize)]
struct Contato {
    nome: String,
    numero: String,
}
pub fn adicionar() {
    let mut agenda: Vec<Contato> = Vec::new();

    // Verificando se o arquivo existe e lendo o conteúdo, se existir
    if let Ok(file_content) = std::fs::read_to_string("agenda.json") {
        // Desserializando o conteúdo do arquivo para a agenda atual
        agenda = serde_json::from_str(&file_content).expect("Erro ao desserializar o JSON.");
    }loop {
        println!("Insira o nome:");
        let mut nome = String::new();
        io::stdin().read_line(&mut nome).expect("Erro ao ler o nome.");
        nome = nome.trim().to_string();

        println!("Insira o número da pessoa:");
        let mut numero = String::new();
        io::stdin().read_line(&mut numero).expect("Erro ao ler o número.");
        numero = numero.trim().to_string();

        // Adicionando o contato à agenda
        agenda.push(Contato { nome, numero });
        println!("Deseja adicionar mais contatos? Digite '0' para sair ou '1' para adicionar.");
        let mut opcao = String::new();
        io::stdin().read_line(&mut opcao).expect("Erro ao ler a opção.");

        if opcao.trim() != "1" {
            break;
        }
    }
    // Salvando a agenda completa em um arquivo JSON
    let json = serde_json::to_string(&agenda).expect("Erro ao serializar para JSON.");
    std::fs::write("agenda.json", json).expect("Erro ao salvar o arquivo JSON.");
    println!("Agenda de contatos salva no arquivo 'agenda.json'.");
}
