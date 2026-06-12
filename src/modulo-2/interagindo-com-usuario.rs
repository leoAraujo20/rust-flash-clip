/*
Interagindo com o usuário:
Em Rust, para ler a entrada do usuário, você pode usar a biblioteca padrão 'std::io'.  
O processo geralmente envolve criar uma variável para armazenar a entrada, 
ler a linha do usuário e, em seguida, processar a entrada conforme necessário.
*/

use std::io;

fn main () {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    println!("Você digitou: {}", entrada.trim());
}