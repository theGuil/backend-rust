use axum::{routing::{get, post},Router};
use crate::mvc::controllers::locatario::controller_locatario::controller_locatario;



pub fn create_routes() -> Router {
    let publuc_routes = Router::new()
    .route("/locatarios", get(controller_locatario::buscar_locatario))
    .route("/locatario", post(controller_locatario::cadastrar_locatario))
    .route("/locatario/:documento", get(controller_locatario::buscar_locatario_documento));

    Router::new().merge(publuc_routes)
}