use std::io;

fn main() {
    println!("=== JOGO DE ADIVINHAÇÃO (1 a 100) ===");

    // O número secreto já está gerado para você!
    // O intervalo 1..101 garante um número entre 1 e 100.
    let secreto: i32 = fastrand::i32(1..101);

    // TODO 1: Crie um laço de repetição infinito (loop) para o jogo rodar até o jogador acertar.

    // TODO 2: Dentro do laço, peça para o jogador digitar o seu chute.
    // Crie a variável (String), leia o teclado com read_line e use o unwrap().

    // TODO 3: Converta o texto digitado para um número inteiro (i32).
    // Lembre-se de usar a dupla .trim().parse().unwrap()

    // TODO 4: Crie a estrutura condicional encadeada (if / else if / else) para testar o chute:
    // - Se o chute for MENOR que o segredo: imprima "Muito baixo! Tente mais alto."
    // - Se o chute for MAIOR que o segredo: imprima "Muito alto! Tente mais baixo."
    // - Se o chute for IGUAL (else): imprima "Acertou! Parabéns!" e pare o laço (break).
}
