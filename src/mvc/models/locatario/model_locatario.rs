pub mod model_locatario {
    //use sqlx::mysql::MySqlRow;
    use serde_json::{json, Value};
    use axum::{extract::Json, http::StatusCode};
    //HELPERS
    use crate::helpers::db::helper_mysql::HelperMySql;
    use serde::{Deserialize, Serialize};
    use sqlx::FromRow;
    
    #[derive(Debug, FromRow, Serialize, Deserialize)]
    pub struct Locatario {
        pub onda_locatario_id: i32,
        pub onda_locatario_codigo: String,
        pub onda_locatario_cnpjcpf: String,
        pub onda_locatario_nome: String,
    }


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
