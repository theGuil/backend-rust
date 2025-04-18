
mod server;

pub mod helpers {
    pub mod db {
        pub mod helper_mysql;
        pub mod helper_postgresql;
    }
    pub mod middleware {
        pub mod token;
    }
    pub mod response {
        pub mod helpers_response;
    }
    pub mod cache {
        pub mod helper_cache;
    }
    pub mod geral {
        pub mod helpers_geral;
    }
}
pub mod mvc {
    pub mod models {
        pub mod usuario {
            pub mod model_usuario;
        }
        pub mod analise {
            pub mod model_analise;
        }
        pub mod locatario {
            pub mod model_locatario;
        }
    }
    pub mod controllers {
        pub mod usuario {
            pub mod controller_usuario;
        }
        pub mod analise {
            pub mod controller_analise;
        }
        pub mod locatario {
            pub mod controller_locatario;
        }
        pub mod testes {
            pub mod controller_testes;
        }
    }
    pub mod routes {
        pub mod usuario {
            pub mod route_usuario;
        }
        pub mod analise {
            pub mod route_analise;
        }
        pub mod locatario {
            pub mod route_locatario;
        }
        pub mod testes {
            pub mod route_testes;
        }
    }
}

//IMPORTAÇÕES
use crate::helpers::db::helper_postgresql::HelperPostgreSql;


#[tokio::main]
async fn main() {
    let app: axum::Router= server::create_app().await;
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Servidor rodando em http://0.0.0.0:3000");
    

    match HelperPostgreSql::init().await {
        Ok(_helper) => println!("MYSQL -Conexão com o banco de dados estabelecida com sucesso!"),
        Err(e) => eprintln!("MYSQL - Erro ao conectar ao banco de dados: {}", e),
    };

   
    axum::serve(listener, app).await.unwrap();

}

