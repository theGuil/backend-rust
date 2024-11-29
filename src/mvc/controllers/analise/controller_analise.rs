use axum::{
    extract::Path,
    response::IntoResponse,
    Json,
    http::StatusCode,
};
// use serde_json::json;
use serde_json::Value;


use crate::mvc::models::analise::model_analise::ModelAnalise;
// use crate::helpers::response::helpers_response::HelpersResponse;
use crate::mvc::models::locatario::model_locatario::model_locatario;
pub struct ControllerAnalise;

pub struct Teste {
   pub teste: String,
}


impl ControllerAnalise {
    pub async fn buscar_todas_analises() -> impl IntoResponse {
        
        model_locatario::buscar_locatario().await
    }

    pub async fn get_analise_by_id(Path(id): Path<String>) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
        ModelAnalise::buscar_totas_analises_id(&id).await
    }

    pub async fn get_analises_by_imob_id(Path(imob_id): Path<String>) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
        ModelAnalise::buscar_analises_by_imob_id(&imob_id).await
    }
}