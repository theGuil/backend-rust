pub mod model_locatario {
    use serde_json::{json, Value};
    use axum::{extract::Json, http::StatusCode};
    use crate::helpers::db::helper_mysql::HelperMySql;
    use serde::{Deserialize, Serialize};
    use sqlx::FromRow;
    // Tipos personalizados para melhor legibilidade
    type ApiResponse = Result<Json<Value>, (StatusCode, Json<Value>)>;


    #[derive(Debug, Deserialize, Serialize,FromRow, Clone)]
    pub struct Locatario {
        pub codigo: String,
        pub cnpj_cpf: Option<String>,
        pub nome: Option<String>,
        pub renda: f64,
        pub rg: Option<String>,
        pub telefone: Option<String>,
        pub celular: Option<String>,
        pub email: Option<String>

    }

    impl Locatario {
    
        pub async fn cadastrar_locatario(payload: Locatario) -> ApiResponse {
            println!("Teste resposta Json: {:?}",&payload);

            let query:String  = format!("
                INSERT INTO onda_locatario (
                    onda_locatario_cnpjcpf, 
                    onda_locatario_nome, 
                    onda_locatario_renda, 
                    onda_locatario_rg,
                    onda_locatario_telefone,
                    onda_locatario_celular, 
                    onda_locatario_email,
                    onda_locatario_cep,
                    onda_locatario_rua,
                    onda_locatario_numero,
                    onda_locatario_bairro,
                    onda_locatario_complemento,
                    onda_locatario_cidade,
                    onda_locatario_uf,
                    onda_locatario_status
                ) VALUES (
                    '{:?}', '{:?}', {:?}, '{:?}', '{:?}', '{:?}', '{:?}''
                )",
                payload.cnpj_cpf,
                payload.nome,
                payload.renda,
                payload.rg,
                payload.telefone,
                payload.celular,
                payload.email,
            );

           // println!("cadastrar locatário no banco ${}",query);

            match HelperMySql::query::<Locatario>(&query).await {
                Ok(locatarios) => Ok(Json(json!({
                    "status": true,
                    "message": "Sucesso ao buscar locatários",
                    "data": locatarios,
                    "cached": false
                }))),
                Err(e) => Err(Self::criar_erro_response(e))
            }
        }


        pub async fn buscar_todos() -> ApiResponse {
            const QUERY: &str = "SELECT * FROM onda_locatario";

            match HelperMySql::query::<Locatario>(QUERY).await {
                Ok(locatarios) => Ok(Json(json!({
                    "status": true,
                    "message": "Sucesso ao buscar locatários",
                    "data": locatarios,
                    "cached": false
                }))),
                Err(e) => Err(Self::criar_erro_response(e))
            }
        }

        pub async fn buscar_por_documento(documento: &str) -> ApiResponse {
            let query = format!(
                "SELECT * FROM onda_locatario WHERE onda_locatario_cnpjcpf = '{}'", 
                documento
            );

            match HelperMySql::query::<Locatario>(&query).await {
                Ok(locatarios) => Ok(Json(json!({
                    "status": true,
                    "message": "Sucesso ao buscar locatário pelo documento",
                    "data": locatarios,
                }))),
                Err(e) => Err(Self::criar_erro_response(e))
            }
        }

        pub async fn cadastrar(&self) -> ApiResponse {
            todo!("Implementar cadastro de locatário")
        }


        fn criar_erro_response(erro: impl std::error::Error) -> (StatusCode, Json<Value>) {
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                    "status": false,
                    "message": "Erro ao realizar consulta",
                    "error": erro.to_string()
                }))
            )
        }
    }
}


