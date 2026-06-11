/*
Texto em número:
Em Rust, para converter um texto (String) em um número, você pode usar o método 'parse()'. 
Ele tenta converter a string em um tipo numérico específico, como i32,
*/

use std::io;

fn main () {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();

    let numero: i32 = entrada.trim().parse().unwrap();
    println!("O dobro de {} é {}", numero, numero * 2);
}