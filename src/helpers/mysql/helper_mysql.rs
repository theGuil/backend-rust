use sqlx::{MySql, MySqlPool, Pool};
use dotenv::dotenv;
use std::env;
use once_cell::sync::OnceCell;
use std::sync::Arc;

static DB_POOL: OnceCell<Arc<HelperMysql>> = OnceCell::new();

// Adicione o derive Debug aqui
#[derive(Debug)]
pub struct HelperMysql {
    pool: Pool<MySql>,
}

impl HelperMysql {
    pub async fn new() -> Result<Self, sqlx::Error> {
        dotenv().ok();

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

        let pool: Pool<MySql> = MySqlPool::connect(&database_url).await?;
        Ok(Self { pool })
    }

    pub async fn init() -> Result<(), sqlx::Error> {
        let helper = Self::new().await?;
        DB_POOL.set(Arc::new(helper))
            .expect("Failed to set DB pool");
        Ok(())
    }

    pub fn get_instance() -> Option<&'static Arc<HelperMysql>> {
        DB_POOL.get()
    }

    pub async fn execute_query(query: &str) -> Result<sqlx::mysql::MySqlQueryResult, sqlx::Error> {
        let instance = Self::get_instance()
            .expect("Database not initialized");
        sqlx::query(query).execute(&instance.pool).await
    }
}