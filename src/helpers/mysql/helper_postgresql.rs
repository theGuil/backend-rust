use sqlx::{postgres::PgPool, Pool, Postgres, postgres::PgConnectOptions};
use dotenv::dotenv;
use std::env;
use once_cell::sync::OnceCell;
use std::sync::Arc;

static DB_POOL: OnceCell<Arc<HelperPostgreSql>> = OnceCell::new();

#[derive(Debug)]
pub struct HelperPostgreSql {
    pool: Pool<Postgres>,
}

impl HelperPostgreSql {
    pub async fn new() -> Result<Self, sqlx::Error> {
        dotenv().ok();

        let options = PgConnectOptions::new()
        .host(&env::var("CONN_DB_HOST").expect("CONN_DB_HOST não configurada"))
        .port(env::var("CONN_DB_PORT").expect("CONN_DB_PORT não configurada").parse().unwrap())
        .username(&env::var("CONN_DB_USERNAME").expect("CONN_DB_USERNAME não configurada"))
        .password(&env::var("CONN_DB_PASS").expect("CONN_DB_PASS não configurada"))
        .database(&env::var("CONN_DB_SCHEMA").expect("CONN_DB_SCHEMA não configurada"));

        let pool = PgPool::connect_with(options).await?;
        Ok(Self { pool })
    }

    pub async fn init() -> Result<(), sqlx::Error> {
        let helper = Self::new().await?;
        DB_POOL.set(Arc::new(helper))
            .expect("Failed to set DB pool");
        Ok(())
    }

    pub fn get_instance() -> Option<&'static Arc<HelperPostgreSql>> {
        DB_POOL.get()
    }

    pub async fn execute_query<T: AsRef<str>>(query: T) -> Result<sqlx::postgres::PgQueryResult, sqlx::Error> {
        let instance = Self::get_instance()
            .expect("Database not initialized");
        sqlx::query(query.as_ref()).execute(&instance.pool).await
    }

    // Método adicional para queries com retorno
    pub async fn query<T>(query: &str) -> Result<Vec<T>, sqlx::Error>
    where
        T: for<'r> sqlx::FromRow<'r, sqlx::postgres::PgRow> + Send + Unpin,
    {
        let instance = Self::get_instance()
            .expect("Database not initialized");
        sqlx::query_as(query)
            .fetch_all(&instance.pool)
            .await
    }
}