use axum::Router;
//IMPORTAÇÕES DAS ROTAS
use crate::mvc::routes::usuario::route_usuario;
use crate::mvc::routes::analise::route_analise;
use crate::mvc::routes::locatario::route_locatario;





//RODAS DA ANALISE
pub async fn create_app() -> Router { 
    Router::new()
        .nest("/usuario", route_usuario::create_routes())
        .nest("/", route_analise::create_routes())
        .nest("/", route_locatario::create_routes())
}


