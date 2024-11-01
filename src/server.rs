use axum::Router;
//IMPORTAÇÕES DAS ROTAS
use  crate::mvc::routes::usuario::route_usuario;
use crate::mvc::routes::analise::route_analise;




//RODAS DA ANALISE
pub async fn create_app() -> Router { 
    Router::new()
        .nest("/api", route_analise::create_routes())
        .nest("/usuario", route_usuario::create_routes())
}


