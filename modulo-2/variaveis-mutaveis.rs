/*
Variáveis Mutáveis:
Para destravarmos o cadeado do rust e permitir que uma variável seja mutável, 
precisamos usar a palavra-chave "mut" ao declarar a variável. 
Isso permite que o valor da variável seja alterado
*/

fn main () {
    let mut pontos = 0;
    println!("Pontos: {}", pontos); // Pontos: 0

    pontos = 10;
    println!("Pontos: {}", pontos); // Pontos: 10

    pontos = pontos + 5;
    println!("Pontos: {}", pontos); // Pontos: 15
}