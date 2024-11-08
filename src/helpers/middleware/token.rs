use axum::{
    body::Body,
    http::{Request, StatusCode},
    middleware::Next,
    response::{Response, IntoResponse},
    Json
};

use serde::{Deserialize, Serialize};
use jsonwebtoken::{decode,encode,Header, DecodingKey, EncodingKey,  Validation};
use serde_json::json;
use chrono::{Duration, Utc};
use crate::mvc::models::usuario::model_usuario::UsuarioRequest;
use crate::helpers::response::helpers_response::HelpersResponse;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Claims {
    pub sub: String,
    pub exp: usize,
    pub iat: usize,
}

pub struct HelperMiddlewareToken {
    encoding_key: EncodingKey,
    decoding_key: DecodingKey,
}

impl HelperMiddlewareToken {
    
    pub fn new() -> Self {
        let secret: &[u8; 17] = b"!25#r$9634A85$236";
        Self {
            encoding_key: EncodingKey::from_secret(secret),
            decoding_key: DecodingKey::from_secret(secret),
        }
    }

    pub async fn create_token(&self,usuario: Json<UsuarioRequest>) -> Response {
        let now = Utc::now();
        let exp = (now + Duration::hours(24)).timestamp() as usize;
        let iat = now.timestamp() as usize;
        
        let claims = Claims {
            sub: usuario.usuario.email.clone(),
            exp,
            iat,
        };

        match encode(
            &Header::default(), 
            &claims, 
            &self.encoding_key
        ) {
            Ok(token) => {
                  HelpersResponse::success("Query executed successfully").into_response()
            },
            Err(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "message": "Erro ao gerar token"
                }))
            ).into_response()
        }
    }

    pub async fn verify_token(&self,mut req: Request<Body>,next: Next,) -> Response {
// Primeiro, obtenha o valor do cabeçalho Authorization
        let auth_header = req
            .headers()
            .get("Authorization")
            .and_then(|value| value.to_str().ok())
            .map(|s| s.trim_start_matches("Bearer "));
            

        match auth_header {
            Some(token) => {
                let validation = Validation::default();
                match decode::<Claims>(token, &self.decoding_key, &validation) {
                    Ok(token_data) => {
                        req.extensions_mut().insert(token_data.claims);
                        next.run(req).await
                    }
                    Err(_) => (
                        StatusCode::UNAUTHORIZED,
                        Json(json!({ "message": "Token inválido" }))
                    ).into_response()
                }
            },
            None => (
                StatusCode::UNAUTHORIZED,
                Json(json!({ "message": "Token não fornecido" }))
            ).into_response()
        }
    }
}