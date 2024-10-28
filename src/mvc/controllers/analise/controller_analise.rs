use axum::{
    extract::Json,
    http::StatusCode,
    response::IntoResponse,
};
use serde_json::json;
use crate::mvc::models::analise::model_analise::{Analise, CreateAnalise};

pub struct ControllerAnalise;

impl ControllerAnalise {
    pub async fn criar_analise(Json(payload): Json<CreateAnalise>,) -> impl IntoResponse {
        let analise: Analise = Analise::new(payload);
        
        (StatusCode::CREATED, Json(json!({
            "message": "Análise criada com sucesso",
            "data": analise
        })))
    }
    
    pub async fn listar_analises() -> impl IntoResponse {
        let analises: Vec<Analise> = Analise::listar();
        
        println!("Buscou as análises");
    
        
        (StatusCode::OK, Json(json!({
            "data": analises
        })))
    }

    pub async fn obter_analise(_id: String) -> impl IntoResponse {
        match Analise::buscar_por_id(_id) {
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
        }
    }

    // função para testar uma nova rota
    pub async  fn testar_rota() -> impl IntoResponse {
        
        (StatusCode::OK, Json(json!({
            "data": {
                "analises": {
                    "id": 122,
                    "nome": "Guilherme de Souza"
                }
            }
        })))
    }

    // função para testar uma nova rota
    pub async  fn testar_rota1() -> impl IntoResponse {
        
        (StatusCode::OK, Json(json!({
            "data": {
                "analises": {
                    "id": 122,
                    "nome": "Guilherme de Souza"
                }
            }
        })))
    }
}
