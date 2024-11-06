use axum::{
    routing::post, 
    Router,
    middleware::from_fn,
    body::Body,
    http::Request,
    response::Response,
    middleware::Next,
};


use crate::{
    mvc::controllers::usuario::controller_usuario::ControllerUsuario,
    helpers::middleware::token::HelperMiddlewareToken,
};

// Middleware de autenticação
async fn auth_middleware(
    req: Request<Body>,
    next: Next,
) -> Response {
    let secret: &[u8; 17] = b"sua_chave_secreta";
    let auth = HelperMiddlewareToken::new(secret);
    auth.verify_token(req, next).await
}

pub fn create_routes() -> Router {
    // Rotas públicas (sem middleware)
    let public_routes = Router::new()
        .route("/login", post(ControllerUsuario::login))
        .route("/register", post(ControllerUsuario::register_usuario));

    // Rotas protegidas (com middleware)
    let protected_routes = Router::new()
        .route("/perfil", post(ControllerUsuario::get_perfil))
        .route("/atualizar", post(ControllerUsuario::update_usuario))
        .layer(from_fn(auth_middleware));

    // Combinar rotas
    Router::new()
        .merge(public_routes)
        .merge(protected_routes)
}