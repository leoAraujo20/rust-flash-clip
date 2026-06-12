/*
Exibindo variáveis:
No Rust, é possível exibir o valor de uma variável usando a macro "println!".
A sintaxe básica para exibir uma variável é: "println!("O valor da variável é: {}", variavel);". 
O "{}" é um marcador de posição que será substituído pelo valor da variável.
*/

fn main (){
    let nome: &str = "João";
    let idade: i32 = 20;
    let nota: f64 = 9.5;
    let aprovado: bool = true;

    println!("Nome: {}", nome); // Nome: João
    println!("Idade: {} anos", idade); // Idade: 20 anos
    println!("Nota: {:.1}", nota); // Nota: 9.5
    println!("Foi Aprovado?: {}", aprovado); // Foi Aprovado?: true
    println!("{} tem {} anos", nome, idade); // João tem 20 anos   
}