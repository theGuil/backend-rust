// src/helpers/mysql/helpers_mysql.rs
use sqlx::{MySql, MySqlPool, Pool};
use dotenv::dotenv;
use std::env;

pub struct HelperMysql {
    pool: Pool<MySql>,
}

impl HelperMysql {
    // Inicializa o pool de conexões
    pub async fn new() -> Result<Self, sqlx::Error> {
        dotenv().ok(); // Carrega as variáveis de ambiente do arquivo .env
        
        let host: String = env::var("CONN_DB_HOST").expect("CONN_DB_HOST não configurada");
        let username: String = env::var("CONN_DB_USERNAME").expect("CONN_DB_USERNAME não configurada");
        let schema: String = env::var("CONN_DB_SCHEMA").expect("CONN_DB_SCHEMA não configurada");
        let password: String = env::var("CONN_DB_PASS").expect("CONN_DB_PASS não configurada");
        
        let port_str = env::var("CONN_DB_PORT").expect("CONN_DB_PORT não configurada");
        println!("Valor da porta (string): {}", port_str);
        let port: u16 = port_str.parse().expect("CONN_DB_PORT deve ser um número válido");
        println!("Valor da porta (u16): {}", port);
        
        

        let database_url: String = format!("mysql://{}:{}@{}:{}/{}", username, password, host, port, schema);
        println!("URL de conexão: {}", database_url);
        
        println!("URL de conexão: {}", database_url);

        let pool: Pool<MySql> = MySqlPool::connect(&database_url).await?;
        Ok(Self { pool })
    }

    // Função para obter uma referência ao pool
    pub fn get_pool(&self) -> &Pool<MySql> {
        &self.pool
    }

    // Função para executar queries no banco de dados
    pub async fn execute_query(&self,query: &str,) -> Result<sqlx::mysql::MySqlQueryResult, sqlx::Error> {
        
        sqlx::query(query).execute(&self.pool).await
    }
}
