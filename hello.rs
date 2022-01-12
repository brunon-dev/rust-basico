fn main() {
    // RAII - Resource acquisition is initialization
    // tipagem estática e forte, mas em alguns contextos o compilador faz a inferencia de tipo da variável
    let name = "Usuário";
    println!("Alô mundo, {}!", name); // Macro
}
