

pub struct Locatario {
    pub id: i32,
    pub codigo: String,
    pub cnpj_cpf: String,
    pub nome: String,
    pub renda: String,
    pub rg: String,
    pub telefone: String,
    pub celular: String,
    pub email: String,
    pub cep: String,
    pub rua: String,
    pub numero: String,
    pub bairro: String,
    pub complemento: String,
    pub cidade: String,
    pub uf: String,
    pub copart1: String,
    pub copart1_renda: f64,
    pub copart1_cpf: String,
    pub copart1_rg: String,
    pub copart2: String,
    pub copart2_renda: f64,
    pub copart2_cpf: String,
    pub copart2_rg: String,
    pub imob: i32,
    pub valor_aluguel: f64,
    pub criado_por: i32,
    pub data_criacao: chrono::DateTime<chrono::Utc>,
    pub alterado_por: i32,
    pub data_alteracao: chrono::DateTime<chrono::Utc>,
    pub status: i32,
}

pub mod model_locatario {
    pub async fn cadastrar_locatario() {
        return ;
    }
}


