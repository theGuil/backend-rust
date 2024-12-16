


pub mod controller_locatario {
    use axum::{extract::Path,response::IntoResponse};
    use crate::mvc::models::locatario::model_locatario::model_locatario;

    pub async fn cadastrar_locatario()  {
        let id: i32 = 150000;
        let codigo: String = "LOC-2323213-2024".to_string();
        let cnpjcpf: String = "10548372950".to_string();
        let nome: String = "Guilherme de Souza".to_string();
        model_locatario::Locatario::new(id, codigo, cnpjcpf, nome);
    }

    pub async  fn buscar_locatario() -> impl IntoResponse {
        model_locatario::Locatario::buscar_todos().await
    }

    pub async  fn buscar_locatario_documento(Path(documento): Path<String>) -> impl IntoResponse {
        print!("documento, ${}",documento);
        model_locatario::Locatario::buscar_por_documento(&documento).await
    }
}