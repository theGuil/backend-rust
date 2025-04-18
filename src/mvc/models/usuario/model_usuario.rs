//BIBLIOTECAS
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};

use axum::{extract::Json,response::{IntoResponse, Response}, http::StatusCode};
//HELPERS
use crate::{
    helpers::db::helper_postgresql::HelperPostgreSql,
    helpers::response::helpers_response::HelpersResponse
    
};
pub struct ModelUsuario;

// Definição das structs para referência
#[derive(Debug, Serialize, Deserialize)]
pub struct UsuarioRequest {
    pub usuario: Usuario
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Usuario {
    pub nome: String,
    pub email: String,
    pub senha: String
    // outros campos...
}
#[derive(Debug)]
pub struct ApiError {
    status_code: StatusCode,
    message: String,
}

impl IntoResponse for ApiError {
    fn into_response(self) -> Response {
        (
            self.status_code,
            Json(json!({
                "status": false,
                "message": self.message
            }))
        ).into_response()
    }
}


impl ModelUsuario{

    pub async fn inserir_usuario(data: Json<UsuarioRequest>) -> impl IntoResponse {

        let query: String = format!(
            "INSERT INTO usuario (
                status_id, 
                nome,
                email, 
                senha
                ) 
             VALUES (
                '1',
                '{}', 
                '{}',
                '{}'
            )",
            data.usuario.nome,
            data.usuario.email,
            "senha123"
        );
         
        // Agora pode usar execute_query com a instância (db)
        match HelperPostgreSql::execute_query(query).await {
            Ok(_) => {

    
                (StatusCode::CREATED, data).into_response()
            },
            Err(_e) => {
                println!("passou aqui {}", _e);
                return HelpersResponse::error("Erro ao inserir usuário").into_response();
            }
        }
    }

    pub async fn verificar_email_existe(email: &String) -> Result<(), (StatusCode, Json<Value>)> {
        let query = format!(
            "SELECT * FROM usuario WHERE email = '{}'",
            email
        );

        match HelperPostgreSql::execute_query(query).await {
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
   
                println!("passou aqui {}", _e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": false,
                        "message": "Erro ao verificar email",
                    }))
                ))
            }
        }
    }
}
