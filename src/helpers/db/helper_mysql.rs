use sqlx::{mysql::{MySqlPool, MySqlQueryResult}, Pool, MySql, mysql::MySqlConnectOptions};
use dotenv::dotenv;
use std::env;
use once_cell::sync::OnceCell;
use std::sync::Arc;

static DB_POOL: OnceCell<Arc<HelperMySql>> = OnceCell::new();

#[derive(Debug)]
pub struct HelperMySql {
    pool: Pool<MySql>,
}

impl HelperMySql {
    pub async fn new() -> Result<Self, sqlx::Error> {
        dotenv().ok();

        let host: String = env::var("MYSQL_CONN_DB_HOST").expect("MYSQL_CONN_DB_HOST não configurada");
        let username: String = env::var("MYSQL_CONN_DB_USERNAME").expect("MYSQL_CONN_DB_USERNAME não configurada");
        let password: String = env::var("MYSQL_CONN_DB_PASSWORD").expect("MYSQL_CONN_DB_PASSWORD não configurada");
        let database: String = env::var("MYSQL_CONN_DB_DATABASE").expect("MYSQL_CONN_DB_DATABASE não configurada");
        let port: u16 = env::var("MYSQL_CONN_DB_PORT").expect("MYSQL_CONN_DB_PORT não configurada").parse().unwrap();

        let options = MySqlConnectOptions::new()
            .host(&host)
            .username(&username)
            .password(&password)
            .database(&database)
            .port(port);

        let pool = MySqlPool::connect_with(options).await?;
        Ok(Self { pool })
    }

    pub async fn init() -> Result<(), sqlx::Error> {
        let helper = Self::new().await?;
        DB_POOL.set(Arc::new(helper))
            .expect("Failed to set DB pool");
        Ok(())
    }

    pub fn get_instance() -> Option<&'static Arc<HelperMySql>> {
        DB_POOL.get()
    }

    // Método para SELECTs que retorna rows
    pub async fn execute_select<T: AsRef<str>>(query: T) -> Result<Vec<sqlx::mysql::MySqlRow>, sqlx::Error> {
        let instance = Self::get_instance().expect("Database not initialized");
        return  sqlx::query(query.as_ref()).fetch_all(&instance.pool).await
    }

    // Método para INSERT, UPDATE, DELETE que retorna apenas o resultado
    pub async fn execute_query<T: AsRef<str>>(query: T) -> Result<MySqlQueryResult, sqlx::Error> {
        let instance = Self::get_instance().expect("Database not initialized");
    
        let result = sqlx::query(query.as_ref())
            .execute(&instance.pool)
            .await?;
        
        // Imprime os detalhes do resultado
        println!("Rows afetadas: {}", result.rows_affected());
        println!("Último ID inserido: {:?}", result.last_insert_id());
        
        Ok(result)
    }

    // Método para queries tipadas
    pub async fn query<T>(query: &str) -> Result<Vec<T>, sqlx::Error>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::mysql::MySqlRow> + Send + Unpin,
    {
        let instance = Self::get_instance()
            .expect("Database not initialized");
        sqlx::query_as(query)
            .fetch_all(&instance.pool)
            .await
    }
}