/*
Operações com texto:
Em Rust, o tratamento de textos é rigoroso para garantir a segurança da memória.
Existem dois tipos principais que você usará no dia a dia:

1. &str (String Slice): É um texto estático, de tamanho fixo, que costuma 
   já estar embutido no código. Ex: "João"
2. String: É um objeto dinâmico (alocado no heap) que pode crescer, 
   diminuir e ser modificado. Ex: String::from("João")

A REGRA DE OURO DA CONCATENAÇÃO (Operador +):
O operador '+' não junta dois textos estáticos. A fórmula dele exige que 
o lado esquerdo seja uma 'String' (que tem espaço para crescer) e os 
lados seguintes sejam referências '&str'.
-> String + &str = String

3.trim(): Remove os espaços em branco das bordas de um texto. 
   Ex: "   Rust é rápido!   ".trim() -> "Rust é rápido!"
*/

fn main() {
    let nome = String::from("João");
    let sobrenome:&str = "Silva";
    
    // Concatenar (juntar strings)
    let nome_completo = nome + " " + sobrenome;
    println!("{}", nome_completo); // João Silva 

    // Limpar espaços nas bordas (Trim)
    let texto_com_espacos = "   Rust é rápido!   ";
    println!("Trim: '{}'", texto_com_espacos.trim()); // 'Rust é rápido!'
}