use axum::{
    response::IntoResponse,
    Json,
    Extension,
    http::StatusCode,
};
use serde_json::json;
use crate::{
    helpers::middleware::token::{Claims, HelperMiddlewareToken},
    mvc::models::usuario::model_usuario::UsuarioRequest,
};

pub struct ControllerUsuario;

impl ControllerUsuario {
    pub async fn login(Json(usuario): Json<UsuarioRequest>) -> impl IntoResponse {

        let auth: HelperMiddlewareToken = HelperMiddlewareToken::new();
        
        auth.create_token(Json(usuario)).await
    }


    pub async fn register_usuario() -> impl IntoResponse {
        // Implementação temporária
        (
            StatusCode::CREATED,
            Json(json!({
                "message": "Usuário registrado"
            }))
        )
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