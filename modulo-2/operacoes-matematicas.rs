/*
Operações matemáticas:
Em Rust, as operações matemáticas básicas são realizadas usando os operadores aritméticos padrão:
- Adição: +
- Subtração: -
- Multiplicação: *
- Divisão: /
- Resto (módulo): %
*/

fn main () {
    let a = 10;
    let b = 3;

    println!("{} + {} = {}", a, b, a + b); // 13
    println!("{} - {} = {}", a, b, a - b); // 7
    println!("{} * {} = {}", a, b, a * b); // 30
    println!("{} / {} = {}", a, b, a / b); // 3 (divisão inteira!)
    println!("{} % {} = {}", a, b, a % b); // 1 (resto)
}