struct Pessoa {
    nome: String,
    idade: u32,
}

fn main(){
     // Criando uma instância da struct "Pessoa"
     let pessoa1 = Pessoa {
        nome: String::from("Sabino"),
        idade: 30,
    };

    // Acessando os campos da struct
    println!("Nome: {}", pessoa1.nome);
    println!("Idade: {}", pessoa1.idade);
}