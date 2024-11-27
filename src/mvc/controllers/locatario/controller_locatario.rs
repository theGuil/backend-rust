// use axum::{
//     //extract::Path,
//     response::IntoResponse
// };



pub mod controller_locatario {
    use crate::mvc::models::locatario::model_locatario::model_locatario;

    pub async fn cadastrar_locatario()  {
        model_locatario::cadastrar_locatario().await
    }
}