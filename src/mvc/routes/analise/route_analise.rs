use axum::{
    routing::{get, post},
    Router,
};
use crate::mvc::controllers::analise::controller_analise::ControllerAnalise;

pub fn create_routes() -> Router {
    Router::new()
        .route("/analises", get(ControllerAnalise::listar_analises))
        .route("/analises", post(ControllerAnalise::criar_analise))
        .route("/analises/:id", get(ControllerAnalise::obter_analise))
        .route("/teste", get(ControllerAnalise::testar_rota))
        .route("/teste1", get(ControllerAnalise::testar_rota1))
}
