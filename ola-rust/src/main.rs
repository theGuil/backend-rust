enum Resultado {
    Sucesso(i32),

}

fn processar() -> Resultado {
    // Simulando um processamento que retorna um resultado
    Resultado::Sucesso(200)
}

fn main() {
    match processar() {
        Resultado::Sucesso(codigo) => println!("Sucesso com código: {}", codigo),
        
    }
}
