
mod server;

pub mod helpers {
    pub mod mysql {
        pub mod helper_mysql;
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
        pub mod usuario {
            pub mod model_usuario;
        }
    }
    pub mod controllers {
        pub mod analise {
            pub mod controller_analise;
        }
        pub mod usuario {
            pub mod controller_usuario;
        }
    }
    pub mod routes {
        pub mod analise {
            pub mod route_analise;
        }
        pub mod usuario {
            pub mod route_usuario;
        }
    }
}

//IMPORTAÇÕES
use crate::helpers::mysql::helper_mysql::HelperMysql;

#[tokio::main]
async fn main() {
    let app: axum::Router= server::create_app().await;
    let listener: tokio::net::TcpListener = tokio::net::TcpListener::bind("0.0.0.0:3000").await.unwrap();
    println!("Servidor rodando em http://0.0.0.0:3000");
    

    match HelperMysql::new().await {
        Ok(_helper) => {
            println!("Conexão com o banco de dados estabelecida com sucesso!");

            // Aqui você pode usar o helper para executar consultas
            // Exemplo: let result = helper.execute_query("SELECT * FROM tabela").await;
        }
        Err(e) => {
            eprintln!("Erro ao conectar ao banco de dados: {}", e);
        }
    };

   
    axum::serve(listener, app).await.unwrap();

}