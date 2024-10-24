use axum::{
    routing::get,
    Router,
    Json,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use tokio::net::TcpListener;  // Adicionado esta linha

// Define a estrutura do usuário
#[derive(Serialize, Deserialize)]
struct User {
    id: u32,
    name: String,
    email: String,
    age: u32,
}



// Handler para a rota /user
async fn get_user() -> Json<User> {
    let user = User {
        id: 1,
        name: String::from("João Silva"),
        email: String::from("joao@email.com"),
        age: 30,
    };

    Json(user)
}

async fn post_user() -> Json<User> {
    let user = User {
        id: 1,
        name: String::from("João Silva"),
        email: String::from("joao@email.com"),
        age: 30,
    };

    Json(user)
}

#[tokio::main]
async fn main() {
    // Cria as rotas
    let app = Router::new()
        .route("/user", get(get_user))
        .route("/user/register", get(post_user));

    // Define o endereço
    let addr = SocketAddr::from(([127, 0, 0, 1], 4000));
    println!("Server running on http://{}", addr);

    // Cria o listener
    let listener = TcpListener::bind(addr).await.unwrap();
    println!("Listening on {}", addr);

    // Inicia o servidor
    axum::serve(listener, app).await.unwrap();  // Mudou esta linha
}