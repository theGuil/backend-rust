

// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, FromRow, Serialize, Deserialize)]
pub struct Locatario {
    pub onda_locatario_id: i32,
    pub onda_locatario_codigo: String,
    pub onda_locatario_cnpjcpf: String,
    pub onda_locatario_nome: String,
    // pub onda_locatario_renda: String,
    // pub onda_locatario_rg: String,
    // pub onda_locatario_telefone: String,
    // pub onda_locatario_celular: String,
    // pub onda_locatario_email: String,
    // pub onda_locatario_cep: String,
    // pub onda_locatario_rua: String,
    // pub onda_locatario_numero: String,
    // pub onda_locatario_bairro: String,
    // pub onda_locatario_complemento: String,
    // pub onda_locatario_cidade: String,
    // pub onda_locatario_uf: String,
    // pub onda_locatario_copart1: String,
    // pub onda_locatario_copart1renda: f64,
    // pub onda_locatario_copart1cpf: String,
    // pub onda_locatario_copart1rg: String,
    // pub onda_locatario_copart2: String,
    // pub onda_locatario_copart2renda: f64,
    // pub onda_locatario_copart2cpf: String,
    // pub onda_locatario_copart2rg: String,
    // pub onda_locatario_imob: i32,
    // pub onda_locatario_valoraluguel: f64,
    // pub onda_locatario_criadopor: i32,
    // //pub onda_locatario_datacriacao: DateTime<Utc>,
    // pub onda_locatario_alteradopor: i32,
    // //pub onda_locatario_dataalteracao: DateTime<Utc>,
    // pub onda_locatario_status: i32,
}



pub mod model_locatario {
    //use sqlx::mysql::MySqlRow;
    use serde_json::{json, Value};
    use axum::{extract::Json, http::StatusCode};
    //HELPERS
    use crate::helpers::db::helper_mysql::HelperMySql;
    use super::Locatario;

    pub async fn cadastrar_locatario() {
        return ;
    }


    pub async  fn buscar_locatarios() -> Result<Json<Value>, (StatusCode, Json<Value>)>  {
        
        let query: &str = "SELECT * FROM onda_locatario";

        match HelperMySql::query::<Locatario>(&query).await {
            Ok(locatarios) => {
                Ok(Json(json!({
                    "status": true,
                    "message": "Sucesso ao buscar locatários",
                    "data": locatarios,
                    "cached": false
                })))
            }
            Err(e) => {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": false,
                        "message": "Erro ao realizar consulta",
                        "error": e.to_string()
                    }))
                ))
            }
        }
    }

 
    pub  async  fn buscar_locatario_documento(documento: &str) -> Result<Json<Value>, (StatusCode, Json<Value>)> {

        let query = format!("SELECT * FROM onda_locatario WHERE onda_locatario_cnpjcpf = '{}'", documento);

        match HelperMySql::query::<Locatario>(&query).await {
            Ok(locatarios) => {
                Ok(Json(json!({
                    "status": true,
                    "message": "Sucesso ao buscar locatário pelo documento",
                    "data": locatarios,
                })))
            }
            Err(e) => {
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": false,
                        "message": "Erro ao realizar consulta",
                        "error": e.to_string()
                    }))
                ))
            }
        }
    }
}
