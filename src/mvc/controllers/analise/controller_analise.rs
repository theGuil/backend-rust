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

pub struct ControllerAnalise;

pub struct Teste {
   pub teste: String,
}


impl ControllerAnalise {
    pub async fn buscar_todas_analises() -> impl IntoResponse {

        println!("Passou aqui!");

        ModelAnalise::buscar_totas_analises().await

     

       // HelpersResponse::success("Sucesso ao buscar análises!",     json!(carta_fianca.))
   
    }

    // Para a rota que busca uma análise específica por ID
    pub async fn get_analise_by_id(Path(id): Path<String>) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
        ModelAnalise::buscar_totas_analises_id(&id).await
    }
}