/*
Estrutura de Repetição: While
O loop "while" é usado para repetir um bloco de código enquanto uma condição for verdadeira.
A cada iteração, a condição é verificada: se for verdadeira, o código dentro do bloco é executado;
se for falsa, o loop é encerrado e o programa continua sua execução normal.

É ideal para situações onde não sabemos exatamente o número de repetições necessárias,
mas temos uma condição clara de parada.
*/

use std::io;

fn main() {
    // Exemplo 1: Contagem Regressiva
    println!("--- Contagem Regressiva ---");
    let mut contagem = 5;

    while contagem > 0 {
        println!("{}", contagem);
        contagem -= 1; // Atalho para contagem = contagem - 1
    }

    // Exemplo 2: Validando Entrada do Usuário
    println!("\n--- Validando Entrada ---");
    let mut entrada = String::new();
    let mut numero: i32 = 0;

    // O loop continua enquanto o número digitado não for positivo
    while numero <= 0 {
        entrada.clear();
        println!("Digite um número positivo:");
        io::stdin().read_line(&mut entrada).unwrap();

        // Tenta converter o texto em número
        numero = entrada.trim().parse().unwrap();
    }

    println!("Número válido: {}", numero);
}
