


pub mod controller_locatario {
    use axum::{extract::Path,response::IntoResponse};
    use crate::mvc::models::locatario::model_locatario::model_locatario;

    pub async fn cadastrar_locatario()  {
  
    }

    pub async  fn buscar_locatario() -> impl IntoResponse {
        model_locatario::Locatario::buscar_todos().await
    }

    pub async  fn buscar_locatario_documento(Path(documento): Path<String>) -> impl IntoResponse {
        print!("documento, ${}",documento);
          model_locatario::Locatario::buscar_por_documento(&documento).await
    }
}