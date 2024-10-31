use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use crate::mvc::models::analise::model_analise::{ModelAnalise, ModelCreateAnalise};

//HELPERS
use crate::helpers::response::helpers_response::HelpersResponse;
pub struct ControllerAnalise;

impl ControllerAnalise {
    pub async fn criar_analise(Json(payload): Json<ModelCreateAnalise>,) -> impl IntoResponse {
        let analise: ModelAnalise = ModelAnalise::new(payload);
        
        return  (StatusCode::CREATED, Json(json!({
            "message": "Análise criada com sucesso",
            "data": analise
        })));
    }
    
    pub async fn listar_analises() -> impl IntoResponse {
        let analises: Vec<ModelAnalise> = ModelAnalise::listar();
        
        println!("Buscou as análises");
    
        
        return (StatusCode::OK, Json(json!({
            "data": analises
        })));
    }

    pub async fn obter_analise(_id: String) -> impl IntoResponse {


       return  match ModelAnalise::buscar_por_id(_id) {
            Some(analise) => (
                StatusCode::OK,
                Json(json!({
                    "data": analise
                }))
            ),
            None => (
                StatusCode::NOT_FOUND,
                Json(json!({
                    "error": "Análise não encontrada"
                }))
            )
        };
    }

    // função para testar uma nova rota
    pub async fn testar_rota(_id: String) -> impl IntoResponse {
        
        let analise: Option<ModelAnalise> = ModelAnalise::buscar_por_id(_id);

        return HelpersResponse::not_found(analise)
    }

    // função para testar uma nova rota
    pub async  fn testar_rota1() -> impl IntoResponse {
        
       return (StatusCode::OK, Json(json!({
            "data": {
                "analises": {
                    "id": 007,
                    "nome": "Guilherme de Souza"
                }
            }
        })));
    }
}
