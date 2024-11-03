//BIBLIOTECAS
use serde::{Deserialize, Serialize};
use axum::{extract::Json,response::IntoResponse, http::StatusCode};
//HELPERS
use crate::helpers::mysql::helper_mysql::{self};

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

        print!("{}", data.usuario.email);
    
        let query: String = format!(
            r#"
            INSERT INTO `user` 
                (   
                    user_status_id, 
                    user_nome, 
                    user_sobre_nome, 
                    user_email, 
                    user_senha
                ) VALUES (
                    '1',
                    '{nome}',
                    '{sobre_nome}',
                    '{email}',
                    '{senha}'
                )
            "#,
            nome = data.usuario.nome,
            sobre_nome = "Silva",
            email = data.usuario.email,
            senha = "senha123"
        );
         
        // Agora pode usar execute_query com a instância (db)
        match helper_mysql::HelperMysql::execute_query(query).await {
            Ok(_) => (StatusCode::OK, Json(data)).into_response(),
            Err(e) => {
                println!("Erro ao inserir: {:?}", e);
                (StatusCode::INTERNAL_SERVER_ERROR, "Erro ao inserir").into_response()
            }
        }
    }
}
