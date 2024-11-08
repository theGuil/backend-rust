use axum::{
    response::IntoResponse,
    Json,
    Extension,
    http::StatusCode,
};
use serde_json::json;
use crate::{
    helpers::middleware::token::{Claims, HelperMiddlewareToken},
    mvc::models::usuario::model_usuario::{UsuarioRequest, ModelUsuario},
};

pub struct ControllerUsuario;

impl ControllerUsuario {
    pub async fn login(Json(data): Json<UsuarioRequest>) -> impl IntoResponse {

        let auth: HelperMiddlewareToken = HelperMiddlewareToken::new();
        
        auth.create_token(Json(data)).await
    }


    pub async fn register_usuario(data: Json<UsuarioRequest>) -> impl IntoResponse {
        // Implementação temporária
        println!("passou para registrar");
        ModelUsuario::verificar_email_existe(&data.usuario.email).await
 

    }

    pub async fn get_perfil(Extension(claims): Extension<Claims>,Json(data): Json<UsuarioRequest>) -> impl IntoResponse {
        // Agora você tem acesso às claims e aos dados do request
        (
            StatusCode::OK,
            Json(json!({
                "user_id": claims.sub,
                "data": data
            }))
        )
    }


    pub async fn update_usuario(Extension(claims): Extension<Claims>) -> impl IntoResponse {
        (
            StatusCode::OK,
            Json(json!({
                "user_id": claims.sub,
                "message": "Usuário atualizado"
            }))
        )
    }
}