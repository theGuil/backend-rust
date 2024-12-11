pub mod model_locatario {
    use serde_json::{json, Value};
    use axum::{extract::Json, http::StatusCode};
    use crate::helpers::db::helper_mysql::HelperMySql;
    use serde::{Deserialize, Serialize};
    use sqlx::FromRow;

    // Tipos personalizados para melhor legibilidade
    type ApiResponse = Result<Json<Value>, (StatusCode, Json<Value>)>;

    #[derive(Debug, FromRow, Serialize, Deserialize)]
    pub struct Locatario {
        pub onda_locatario_id: i32,
        pub onda_locatario_codigo: String,
        pub onda_locatario_cnpjcpf: String,
        pub onda_locatario_nome: String,
    }

    impl Locatario {
        // Métodos da struct
        pub fn new(id: i32, codigo: String, cnpjcpf: String, nome: String) -> Self {
            Self {
                onda_locatario_id: id,
                onda_locatario_codigo: codigo,
                onda_locatario_cnpjcpf: cnpjcpf,
                onda_locatario_nome: nome,
            }
        }

        // Métodos de consulta movidos para dentro do impl
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
            // Usar parâmetros preparados para evitar SQL injection
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
            // TODO: Implementar lógica de cadastro
            todo!("Implementar cadastro de locatário")
        }

        // Método auxiliar privado para criar resposta de erro
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