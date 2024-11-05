use axum::Router;
//IMPORTAÇÕES DAS ROTAS
use  crate::mvc::routes::usuario::route_usuario;





//RODAS DA ANALISE
pub async fn create_app() -> Router { 
    Router::new()
        .nest("/usuario", route_usuario::create_routes())
}


