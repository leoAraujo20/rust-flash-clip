/*
Estrutura de Repetição: For
For é muito útil para iterar sobre coleções, como arrays, vetores ou intervalos.
A sintaxe básica para usar um loop for é: "for elemento in coleção { /* código a ser executado para cada elemento */ }".
Dentro do loop, "elemento" representa o valor atual da coleção que está sendo iterado. 
O loop for continuará a iterar até que todos os elementos da coleção tenham sido processados.
*/

fn main() {

    // Iterando sobre um intervalo de números usando o operador ..
    for i in 1..5 {
        println!("{}", i); // Imprime: 1, 2, 3, 4

    }

    // O operador ..= inclui o número final (5) na iteração
    for i in 1..=5 {
        println!("{}", i); // Imprime: 1, 2, 3, 4, 5
    }

    //tabuada do 7
    for i in 1..=10 {
        println!("7 x {} = {}", i, 7 * i);
    }

    //Curiosidade: iterando sobre um array(Lista)
    let frutas = ["maçã", "banana", "laranja"];
    for fruta in frutas {
        println!("Fruta: {}", fruta);
    }
 }