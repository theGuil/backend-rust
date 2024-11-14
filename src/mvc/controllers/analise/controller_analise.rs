use axum::response::IntoResponse;
// use serde_json::json;



use crate::mvc::models::analise::model_analise::ModelAnalise;
// use crate::helpers::response::helpers_response::HelpersResponse;

pub struct ControllerAnalise;

pub struct Teste {
   pub teste: String,
}


impl ControllerAnalise {
    pub async fn buscar_todas_analises() -> impl IntoResponse {

        println!("Passou aqui!");

        ModelAnalise::buscar_totas_analises().await.into_response()

     

       // HelpersResponse::success("Sucesso ao buscar análises!",     json!(carta_fianca.))
   
    }
}