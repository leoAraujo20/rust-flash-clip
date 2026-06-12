/*
Ownership (Posse) e Borrowing (Empréstimo):
Este é o conceito mais importante do Rust! Ele garante que o programa use a memória de forma segura.

- Ownership: Cada valor tem um "dono". Quando o dono sai de cena, o valor é apagado.
- Transferência: Se você passa o valor para outra variável, a antiga perde o acesso (o livro muda de dono).
- Borrowing (Empréstimo): Você pode "emprestar" um valor usando o símbolo "&". Assim, você pode usar o valor sem tirar a posse do dono original.
*/

fn imprimir(texto: &String) {
    // recebe empréstimo
    println!("{}", texto);
}

fn main() {
    let livro = String::from("Dom Quixote");
    let novo_dono = livro; // 'livro' deu o livro pra 'novo_dono'

    // println!("{}", livro); ERRO! 'livro' não tem mais o livro!
    println!("{}", novo_dono); // OK!

    let livro = String::from("Dom Quixote");
    imprimir(&livro); // empresta, não transfere
    println!("Ainda tenho: {}", livro); // funciona!
}
