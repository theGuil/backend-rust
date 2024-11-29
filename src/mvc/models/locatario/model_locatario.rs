

// use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};


#[derive(Debug, Serialize, Deserialize)]
pub struct Locatario {
    pub onda_locatario_id: i32,
    pub onda_locatario_codigo: String,
    pub onda_locatario_cnpjcpf: String,
    pub onda_locatario_nome: String,
    pub onda_locatario_renda: String,
    pub onda_locatario_rg: String,
    pub onda_locatario_telefone: String,
    pub onda_locatario_celular: String,
    pub onda_locatario_email: String,
    pub onda_locatario_cep: String,
    pub onda_locatario_rua: String,
    pub onda_locatario_numero: String,
    pub onda_locatario_bairro: String,
    pub onda_locatario_complemento: String,
    pub onda_locatario_cidade: String,
    pub onda_locatario_uf: String,
    pub onda_locatario_copart1: String,
    pub onda_locatario_copart1renda: f64,
    pub onda_locatario_copart1cpf: String,
    pub onda_locatario_copart1rg: String,
    pub onda_locatario_copart2: String,
    pub onda_locatario_copart2renda: f64,
    pub onda_locatario_copart2cpf: String,
    pub onda_locatario_copart2rg: String,
    pub onda_locatario_imob: i32,
    pub onda_locatario_valoraluguel: f64,
    pub onda_locatario_criadopor: i32,
    //pub onda_locatario_datacriacao: DateTime<Utc>,
    pub onda_locatario_alteradopor: i32,
    //pub onda_locatario_dataalteracao: DateTime<Utc>,
    pub onda_locatario_status: i32,
}

pub mod model_locatario {
    //use sqlx::mysql::MySqlRow;
    use serde_json::{json, Value};
    use axum::{extract::Json, http::StatusCode};
    use sqlx::Row;
    //HELPERS
    use crate::helpers::db::helper_mysql::HelperMySql;
    use crate::helpers::cache::helper_cache::HelperCache;

    pub async fn cadastrar_locatario() {
        return ;
    }

    pub async  fn buscar_locatario() -> Result<Json<Value>, (StatusCode, Json<Value>)> {

        let query: &str = "SELECT * FROM onda_locatario";

        match HelperMySql::execute_select(query).await {
            Ok(results) => {

                let dados: Vec<Value> = results
                    .iter()
                    .map(|row| {
                        json!({
                            "onda_locatario_id": row.get::<i32, _>("onda_locatario_id"),
                            "onda_locatario_codigo": row.get::<String, _>("onda_locatario_codigo"),
                            "onda_locatario_cnpjcpf": row.get::<String, _>("onda_locatario_cnpjcpf"),

                        })
                    })
                    .collect();
    
                let response_data = json!({
                    "status": true,
                    "message": "Consulta realizada com sucesso",
                    "data": dados,
                    "cached": false
                });
    
                // Salva no cache
                if let Err(e) = HelperCache::set_json_cartafianca(json!(dados)) {
                    println!("Erro ao salvar no cache: {}", e);
                }
    
                Ok(Json(response_data))
            }
            Err(e) => {
                println!("Erro: {:?}", e);
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


