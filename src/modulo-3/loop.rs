/*
Estrutura de Repetição: Loop
No Rust, a estrutura de repetição "loop" é usada para criar um loop infinito.
A sintaxe básica para criar um loop é: "loop { /* código a ser repetido */ }".
Dentro do loop, é possível usar a palavra-chave "break" para sair do loop quando uma determinada condição for atendida.
também é possível usar a palavra-chave "continue" para pular para a próxima iteração do loop, 
ignorando o restante do código dentro do loop para aquela iteração específica.
*/

fn main() {
    let mut i = 0;
    loop {
        i += 1;
        if i == 3 {
            continue;
        }
        if i > 5 {
            break;
        }
        println!("i = {}", i);
    }
}