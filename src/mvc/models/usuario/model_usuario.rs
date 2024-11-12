//BIBLIOTECAS
use serde::{Deserialize, Serialize};

use axum::{extract::Json,response::IntoResponse, http::StatusCode};
//HELPERS
use crate::{
    helpers::mysql::helper_mysql::HelperPostgreSql,
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



impl ModelUsuario{

    pub async fn inserir_usuario(data: Json<UsuarioRequest>) -> impl IntoResponse {

        let query: String = format!(
            "INSERT INTO usuario (usuario_status_id, usuario_nome, usuario_sobre_nome, usuario_email, usuario_senha) 
             VALUES ('1', '{}', '{}', '{}', '{}')",
            data.usuario.nome,
            "Silva",
            data.usuario.email,
            "senha123"
        );
         
        // Agora pode usar execute_query com a instância (db)
        match HelperPostgreSql::execute_query(query).await {
            Ok(_) => {

    
                (StatusCode::CREATED, data).into_response()
            },
            Err(_e) => {
 
                return HelpersResponse::error("Erro ao inserir usuário");
            }
        }
    }

    pub async fn verificar_email_existe(email: &String) -> impl IntoResponse {
        let query: String = format!("
            SELECT 
                * 
            FROM usuario 
            WHERE usuario_email = '{}'
        ",
            email
        );
    

       match HelperPostgreSql::execute_query(query).await {
            Ok(results) => {
                if results.rows_affected() == 0 {
                    println!("Nenhum registro encontrado");
                    return   HelpersResponse::success("Nenhum registro encontrado").into_response()
                  
                } else {
                    return   HelpersResponse::success("007").into_response()
                }
                
            }
            Err(_e) =>{ 
                return   HelpersResponse::success("Query executed successfully").into_response()
            },
        }

    }
}
