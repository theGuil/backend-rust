

pub mod controller_testes {
    use axum::{ Json, http::StatusCode};
    use serde_json::{Value, json};
    use crate::helpers::geral::helpers_geral::helpers_geral;


    pub async fn testar_soma() -> Result<Json<Value>, (StatusCode, Json<Value>)> {
        
      
        let a: i32 = 20;
        let b: i32 = 150;

        let somda_realizada: i32 = helpers_geral::somar_notas(a, b).await;

        let resultado_response: Value = json!({
            "message": "Sucesso ao calcular numeros!",
            "resultado": somda_realizada
        });

        return Ok(Json(resultado_response));
    }
}