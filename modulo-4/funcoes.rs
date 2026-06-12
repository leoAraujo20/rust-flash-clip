/*
Funções:
Funções são blocos de código que executam uma tarefa específica e podem ser reutilizados em várias partes do programa.
Elas ajudam a organizar o código, tornando-o mais limpo e fácil de entender.

- Declaração: Usamos a palavra "fn" seguida pelo nome da função.
- Parâmetros: São valores que a função recebe para trabalhar.
- Retorno: É o resultado que a função devolve para quem a chamou. No Rust, o retorno pode ser a última expressão sem ponto e vírgula.
*/

fn saudacao() {
    println!("Ola! Bem-vindo ao Rust!");
}

// Função com parâmetros e retorno
fn soma(a: i32, b: i32) -> i32 {
    a + b // A expressão final é o valor retornado
}

fn quadrado(n: i32) -> i32 {
    n * n
}

fn maior(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}

fn main() {
    saudacao();
    saudacao(); // pode chamar mais de uma vez

    let resultado = soma(3, 7);
    println!("3 + 7 = {}", resultado); // 3 + 7 = 10
    println!("5 + 5 = {}", soma(5, 5)); // 5 + 5 = 10

    println!("4² = {}", quadrado(4)); // 4² = 16
    println!("Maior: {}", maior(10, 7)); // Maior: 10
}
