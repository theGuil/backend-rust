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
        pub id: Option<i32>,
        pub codigo: Option<String>,
        pub cnpj_cpf: Option<String>,
        pub nome: Option<String>,
        pub renda: Option<String>,
        pub rg: Option<String>,
        pub telefone: Option<String>,
        pub celular: Option<String>,
        pub email: Option<String>,
        
        // Endereço
        pub cep: Option<String>,
        pub rua: Option<String>,
        pub numero: Option<String>,
        pub bairro: Option<String>,
        pub complemento: Option<String>,
        pub cidade: Option<String>,
        pub uf: Option<String>,
        
        // Coparticipantes
        pub copart1: Option<String>,
        pub copart1_renda: Option<f64>,
        pub copart1_cpf: Option<String>,
        pub copart1_rg: Option<String>,
        
        pub copart2: Option<String>,
        pub copart2_renda: Option<f64>,
        pub copart2_cpf: Option<String>,
        pub copart2_rg: Option<String>,
        
        // Outros dados
        pub imob: Option<i32>,
        pub valor_aluguel: Option<f64>,
        
        // Metadados
        pub criado_por: Option<i32>,
        //pub data_criacao: Option<DateTime<Utc>>, verificar como usar as datas sem erro
        pub alterado_por: Option<i32>,
        //pub data_alteracao: Option<DateTime<Utc>>, verificar como usar as tadas sem erro
        pub status: Option<i8>,

    }

    impl Locatario {
        // Métodos da struct
        pub async fn cadastrar_locatario(id: String)  {

            let query: String = format!("
                INSERT INTO onda_locatario (
                    onda_locatario_codigo, 
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
                    onda_locatario_copart1, 
                    onda_locatario_copart1renda, 
                    onda_locatario_copart1cpf, 
                    onda_locatario_copart1rg, 
                    onda_locatario_copart2, 
                    onda_locatario_copart2renda, 
                    onda_locatario_copart2cpf, 
                    onda_locatario_copart2rg, 
                    onda_locatario_imob, 
                    onda_locatario_valoraluguel, 
                    onda_locatario_criadopor, 
                    onda_locatario_datacriacao, 
                    onda_locatario_alteradopor, 
                    onda_locatario_dataalteracao, 
                    onda_locatario_status
                ) VALUES (
                 '{}'
                )
            ", id);
            
            println!("Dados query, '{}'", query);
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