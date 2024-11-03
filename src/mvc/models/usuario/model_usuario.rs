// src/mvc/models/usuario/model_usuario.rs
//use crate::helpers::mysql::helpers_mysql::HelperMysql;
use serde::{Deserialize, Serialize};
use axum::Json;

//use crate::mvc::controllers::usuario;

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


    // Função para inserir um usuário no banco de dados usando execute_query
    pub async fn inserir_usuario(Json(data): Json<UsuarioRequest>)  {
        // Monta a query SQL com parâmetros
        //let query = "INSERT INTO usuarios (nome, email) VALUES (?, ?)";



        println!("passou para inserir o usuário com mysql, {:?}", data.usuario.email)


   
    }
}
