use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use axum::{extract::Json, http::StatusCode};

//HELPERS
use crate::helpers::db::helper_mysql::HelperMySql;

#[derive(Debug, Serialize, Deserialize)]
struct CartaFianca {
    // Add fields based on your VW_CARTAFIANCA view structure
    id: i32,
    // ... other fields
}

pub struct ModelAnalise;

impl ModelAnalise {
    pub async fn buscar_totas_analises() -> Result<(), (StatusCode, Json<Value>)>  {
        let query:&str  = "SELECT * FROM VW_CARTAFIANCA";
        
        match HelperMySql::execute_query(query).await {
            Ok(results) => {
                if results.rows_affected() > 0 {
                    // Se encontrou o email, retorna erro
                    Err((
                        StatusCode::BAD_REQUEST,
                        Json(json!({
                            "status": false,
                            "message": "Email já cadastrado no sistema"
                        }))
                    ))
                } else {
                    // Se não encontrou o email, continua o fluxo
                    Ok(())
                }
            }
            Err(_e) => {
                // Se der erro na consulta
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": false,
                        "message": "Erro ao verificar email"
                    }))
                ))
            }
        }
    }

   
        
    
}