use crate::mvc::routes::analise::route_analise;
use axum::Router;

pub async fn create_app() -> Router {
    Router::new().nest("/api", route_analise::create_routes())
}