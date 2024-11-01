use axum::{routing::{get, post}, Router};
use crate::mvc::controllers::usuario::controller_usuario::ControllerUsuario;



pub fn create_routes() -> Router {
    Router::new()
        .route("/login", get(ControllerUsuario::register_usuario))
        .route("/register", post(ControllerUsuario::register_usuario))
}


