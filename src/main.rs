
mod server;

pub mod helpers {
    pub mod mysql {
        pub mod helpers_mysql;
    }
    pub mod response {
        pub mod helpers_response;
    }
}
pub mod mvc {
    pub mod models {
        pub mod analise {
            pub mod model_analise;
        }
    }
    pub mod controllers {
        pub mod analise {
            pub mod controller_analise;
        }
    }
    pub mod routes {
        pub mod analise {
            pub mod route_analise;
        }
    }
}


#[tokio::main]
async fn main() {
    let app: axum::Router= server::create_app().await;
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Servidor rodando em http://0.0.0.0:3000");
    axum::serve(listener, app).await.unwrap();
}