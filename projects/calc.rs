use std::io;


fn main(){  
    println!("escolha uma operação: \n 1 - soma \n 2 - subtração \n 3 - divisão \n 4 - multiplicação \n");
    let mut escolha = String::new(); 

    // Lê a entrada do usuário e a armazena na variável "escolha"
    io::stdin().read_line(&mut escolha)
        .expect("Falha ao ler a entrada");

    // Converte a entrada para um número inteiro (i8)
    let escolha: i8 = escolha.trim().parse()
        .expect("Escolha inválida. Por favor, insira um número válido.");

        match escolha {
            1 => println!("Você escolheu Soma."),
            2 => println!("Você escolheu Subtração."),
            3 => println!("Você escolheu Divisão."),
            4 => println!("Você escolheu Multiplicação."),
            _ => println!("Opção inválida. Por favor, insira um número válido (1 a 4)."),
        }

        println!("Insira o valor de A e logo após o valor de B para que a operação seja executada");
        let mut a:String = String::new();
        let mut b:String = String::new();

        io::stdin().read_line(&mut a)
        .expect("Falha ao ler a entrada");
        io::stdin().read_line(&mut b)
        .expect("Falha ao ler a entrada");

        let a: i32 = a.trim().parse()
        .expect("Escolha inválida. Por favor, insira um número válido.");
        let b: i32 = b.trim().parse()
        .expect("Escolha inválida. Por favor, insira um número válido.");


        if escolha == 1{
            println!("Resultado: {}", a + &b);
        }
        else if escolha == 2{
            println!("Resultado: {}", a - &b);
        }
        else if escolha == 3{
            println!("Resultado: {}", a / &b);
        }
        else if escolha == 4{
            println!("Resultado: {}", a * &b);
        }
        else{
            print!("enceirouuu");
        }
    loop {}

}