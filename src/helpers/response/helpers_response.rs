use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;

pub struct HelpersResponse;

impl HelpersResponse {
    pub fn sucesso<T: serde::Serialize>(data: T) -> impl IntoResponse {
        (StatusCode::OK, Json(json!({ "status": "success", "data": data })))
    }
    
    pub fn not_found<T: serde::Serialize>(data: T) -> impl IntoResponse {
        (StatusCode::NOT_FOUND, Json(json!({ "status": "error", "data": data })))
    }


}