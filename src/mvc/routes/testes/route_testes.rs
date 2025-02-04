use axum::{routing::get, Router};
use crate::mvc::controllers::testes::controller_testes::controller_testes;


pub fn create_routes() -> Router {

    // Combinar rotas
    Router::new().route("/testes/somar",  get(controller_testes::testar_soma))

}