use axum::{
    routing::{get, post},
    Router,
};
use crate::mvc::controllers::analise::controller_analise;

pub fn create_routes() -> Router {
    Router::new()
        .route("/analises", get(controller_analise::listar_analises))
        .route("/analises", post(controller_analise::criar_analise))
        .route("/analises/:id", get(controller_analise::obter_analise))
}