use axum::{extract::Json,response::IntoResponse};


//HELPERS
//use crate::helpers::mysql::helper_mysql::HelperMysql;
//MODELS
use crate::mvc::models::usuario::model_usuario::{ModelUsuario, UsuarioRequest};





pub struct ControllerUsuario;



impl ControllerUsuario {
    pub async fn register_usuario(data: Json<UsuarioRequest>) -> impl IntoResponse {
        
        ModelUsuario::verificar_email_existe(&data.usuario.email).await;

        ModelUsuario::inserir_usuario(data).await

        //(StatusCode::OK, Json(usuario))
    }
}