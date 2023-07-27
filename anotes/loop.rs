fn main(){
    let mut counter = 0;

    loop {
        println!("O contador é: {}", counter);
        counter += 1;

        if counter == 5 {
            break; // Interrompe o loop quando o contador atinge 5
        }
    }

}