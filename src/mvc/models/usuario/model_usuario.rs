// src/mvc/models/usuario/model_usuario.rs
use crate::helpers::mysql::helpers_mysql::HelperMysql;

pub struct ModelUsuario<'a> {
    helper_mysql: &'a HelperMysql, // Referência ao HelperMysql
}

impl<'a> ModelUsuario<'a> {
    // Método de criação do ModelUsuario com uma referência ao HelperMysql
    pub fn new(helper_mysql: &'a HelperMysql) -> Self {
        Self { helper_mysql }
    }

    // Função para inserir um usuário no banco de dados usando execute_query
    pub async fn inserir_usuario(&self, nome: &str, email: &str) -> Result<(), sqlx::Error> {
        // Monta a query SQL com parâmetros
        let query = "INSERT INTO usuarios (nome, email) VALUES (?, ?)";

        // Executa a query usando execute_query do HelperMysql
        let result = sqlx::query(query)
            .bind(nome)
            .bind(email)
            .execute(self.helper_mysql.get_pool())
            .await?;

        println!("Usuário inserido com sucesso, linhas afetadas: {}", result.rows_affected());
        Ok(())
    }
}
