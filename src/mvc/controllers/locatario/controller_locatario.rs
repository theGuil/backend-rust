


pub mod controller_locatario {
    use axum::{extract::Path,response::IntoResponse};
    use crate::mvc::models::locatario::model_locatario::model_locatario;

    pub async fn cadastrar_locatario()  {
        model_locatario::cadastrar_locatario().await
    }

    pub async  fn buscar_locatario() -> impl IntoResponse {
        model_locatario::buscar_locatarios().await
    }

    pub async  fn buscar_locatario_documento(Path(documento): Path<String>) -> impl IntoResponse {
          model_locatario::buscar_locatario_documento(&documento).await
    }
}