use axum::{
    routing::get, 
    Router,
    // middleware::from_fn,
    // body::Body,
    // http::Request,
    // response::Response,
    // middleware::Next,
};


use crate::{
    mvc::controllers::analise::controller_analise::ControllerAnalise,
    // helpers::middleware::token::HelperMiddlewareToken,
};

// // Middleware de autenticação
// async fn auth_middleware(req: Request<Body>,next: Next) -> Response {

//     let auth: HelperMiddlewareToken = HelperMiddlewareToken::new();
//     auth.verify_token(req, next).await
// }

pub fn create_routes() -> Router {
    // Rotas públicas (sem middleware)
    let public_routes = Router::new()
        .route("/analises",  get(ControllerAnalise::buscar_todas_analises))
        .route("/analises/:id",  get(ControllerAnalise::get_analise_by_id))
        .route("/analises/imobiliaria/:id", get(ControllerAnalise::get_analises_by_imob_id));
       

    // Combinar rotas
    Router::new()
        .merge(public_routes)

}