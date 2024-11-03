//BIBLIOTECAS
use serde::{Deserialize, Serialize};
use axum::{extract::Json,response::IntoResponse, http::StatusCode};
//HELPERS
use crate::helpers::mysql::helper_mysql::HelperMysql;

pub struct ModelUsuario;

// Struct interna para os dados do usuário
#[derive(Debug, Deserialize, Serialize)]
pub struct Usuario {
    nome: String,
    email: String,
}

// Struct principal que contém o objeto usuario
#[derive(Debug, Deserialize, Serialize)]
pub struct UsuarioRequest {
    usuario: Usuario,
}


impl ModelUsuario{

    pub async fn inserir_usuario(Json(data): Json<UsuarioRequest>) -> impl IntoResponse {
        // Primeiro, cria uma instância de HelperMysql
        let db = match HelperMysql::new().await {
            Ok(db) => db,
            Err(e) => {
                println!("Erro ao conectar: {:?}", e);
                return (StatusCode::INTERNAL_SERVER_ERROR, "Erro ao conectar").into_response();
            }
        };
    
        // Sua query
        let query = "
            INSERT INTO `user` 
                (   user_status_id, 
                    user_nome, 
                    user_sobre_nome, 
                    user_email, 
                    user_senha
                ) VALUES ('1', 
                    'João', 
                    'Silva', 
                    'joao.silva@email.com', 
                    'senha123'
                )";
    
        // Agora pode usar execute_query com a instância (db)
        match db.execute_query(query).await {
            Ok(_) => (StatusCode::OK, "Inserido com sucesso").into_response(),
            Err(e) => {
                println!("Erro ao inserir: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Erro ao inserir").into_response()
            }
        }
    }
}
