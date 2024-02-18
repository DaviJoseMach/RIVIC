mod add;
mod list;
mod exc;


use std::io;
use add::adicionar;
use list::lista;
use exc::excluir;


fn main() {
    println!("🗒  AGENDA 🗒 \n  \ndigite --> (;comd) para ver os comandos disponiveis na agenda");

    loop {
    //variaveis ceirosas
    let mut escolha:String = String::new();


    println!(" \nDigite seu comando ⬇️");
    io::stdin().read_line(&mut escolha).expect("erro");
     // Removendo espaços em branco e caracteres de nova linha da entrada do usuário
    escolha = escolha.trim().to_string();

    if escolha.eq(";comd"){
        println!("\n🔻  COMANDOS 🔺\n \n ;add - (adiciona contatos) \n ;rem - (remove contatos) \n ;list - (ver a lista de contatos) \n \n sair - (fecha a agenda) \n ============================ \n \n")
    }else if escolha.eq(";add") {
        println!("\n");
        adicionar();
    }else if escolha.eq(";list"){
       println!("\n"); 
       lista();
    }else if escolha.eq(";rem") {
        excluir();
    }
    else if escolha.eq("sair"){
        break;
    }else {
        println!(" < Comando não reconhecido pelo sistema >")
    }
}

}
