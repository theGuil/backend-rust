use axum::{http::StatusCode, response::{IntoResponse,Json, Response}};
// use serde::Serialize;
use serde_json::json;
//use serde_json::Value;

pub struct HelpersResponse;

//#[derive(Serialize)]
// struct Responses {
//     code: &'static str,
//     r#type: &'static str,
//     message: String,
//     //results: Option<Value>,
// }

impl HelpersResponse {
    pub fn success(message: &str) ->  Response {
       (
            StatusCode::OK,
            Json(json!({
                "code": "SUCCESS",
                "type": "success",
                "message": message
            }))
        ).into_response()
    }

    pub fn error(message: &str) ->  Response {
         (
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(json!({
                "code": "ERROR",
                "type": "error",
                "message": message
            }))
        ).into_response()
    }
    
}
