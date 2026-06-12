use std::io;

fn main() {
    println!("=== SALA DE TREINAMENTO ===");

    // TODO 1: O Aquecimento (for)
    // Crie um laço 'for' que vá de 1 até 3 (inclusivo).
    // A cada repetição, imprima: "Iniciando Round X..."
    // Após terminar o laço, imprima "FIGHT!"

    let mut hp_chefao = 100;

    // TODO 2: A Batalha (while)
    // Crie um laço 'while' que continue rodando ENQUANTO o 'hp_chefao' for maior que 0.
    // Dentro do laço:
    // - Mostre o HP atual e peça para o jogador digitar o dano do ataque.
    // - Leia o teclado e converte o texto para número inteiro (i32).
    // - Subtraia o valor digitado do 'hp_chefao'.

    println!("K.O.! Você destruiu o alvo!\n");

    // TODO 3: Fim de Jogo (loop)
    // Crie um laço infinito usando 'loop'.
    // Dentro do laço:
    // - Peça para o usuário digitar 0 para sair da máquina.
    // - Leia o teclado e converta para número inteiro (i32).
    // - Use uma estrutura condicional (if):
    //   -> Se o número for 0, imprima "Game over" e quebre o laço (break).
    //   -> Senão, avise que a opção é inválida e deixe o laço repetir.
}
