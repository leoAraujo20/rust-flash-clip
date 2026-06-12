/*
Estruturas de Condição:
No Rust, as estruturas de condição são usadas para tomar decisões com base em condições.
A estrutura de condição mais comum é a simples "if", que permite executar um bloco de código se uma condição for verdadeira.

Para organizar essas decisões, podemos classificar em três tipos:
- Condicional Simples: Apenas um "if". Cria uma única rota que só é executada se a condição for verdadeira.
- Condicional Composta: "if" e "else". Cria uma bifurcação, obrigando o programa a escolher entre dois caminhos (A ou B).
- Condicional Encadeada: "if", "else if" e "else". 
    Cria uma corrente de verificações. 
    O programa testa as condições uma por uma até encontrar uma verdadeira (como no exemplo abaixo).
*/

fn main() {
    let nota = 8.5;

    if nota >= 9.0 {
        println!("Excelente!");
    } else if nota >= 7.0 {
        println!("Bom! Aprovado");

    } else if nota >= 5.0 {
        println!("Recuperação");
    } else {
        println!("Reprovado");
    }
}