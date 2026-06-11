use std::io;

fn main () {
    let mut entrada = String::new();
    io::stdin().read_line(&mut entrada).unwrap();
    println!("Você digitou: {}", entrada.trim());
}