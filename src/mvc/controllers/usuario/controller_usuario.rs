use axum::{extract::Json,response::IntoResponse};


//HELPERS
//use crate::helpers::mysql::helper_mysql::HelperMysql;
//MODELS
use crate::mvc::models::usuario::model_usuario::{ModelUsuario, UsuarioRequest};





pub struct ControllerUsuario;



impl ControllerUsuario {
    pub async fn register_usuario(Json(usuario): Json<UsuarioRequest>) -> impl IntoResponse {
   
        ModelUsuario::inserir_usuario(Json(usuario)).await;

        //(StatusCode::OK, Json(usuario))
    }
}