fn main(){
  // Declarando um array de inteiros com tamanho 5
let array_inteiros: [i32; 5] = [1, 2, 3, 4, 5];

// Declarando um array de caracteres com tamanho 3
let array_caracteres: [char; 3] = ['a', 'b', 'c'];

// Declarando um array de strings com tamanho 4
let array_strings: [&str; 4] = ["hello", "world", "rust", "lang"];

// exibindo 3 valor do array no caso o 4
println!("{}", array_inteiros[3]);
}