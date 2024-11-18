use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use axum::{extract::Json, http::StatusCode};
use sqlx::Row;
//HELPERS
use crate::helpers::db::helper_mysql::HelperMySql;
use crate::helpers::cache::helper_cache::HelperCache;
#[derive(Debug, Serialize, Deserialize)]
struct CartaFianca {
    // Add fields based on your VW_CARTAFIANCA view structure
    id: i32,
    contrato: String
    // ... other fields
}


pub struct ModelAnalise;

impl ModelAnalise {
    pub async fn buscar_totas_analises() -> Result<Json<Value>, (StatusCode, Json<Value>)> {
        // Inicializa o cache
        HelperCache::initialize();
    
        // Tenta buscar do cache primeiro (como array)
        if let Some(cached_data) = HelperCache::get_array_cartafianca() {
            return Ok(Json(json!({
                "status": true,
                "message": "Dados recuperados do cache",
                "data": cached_data,
                "cached": true
            })));
        }
    
        // Se não estiver no cache, busca do banco
        let query: &str = "SELECT * FROM VW_CARTAFIANCA_GERAL";
        
        match HelperMySql::execute_select(query).await {
            Ok(results) => {
                let dados: Vec<Value> = results
                    .iter()
                    .map(|row| {
                        json!({
                            "id": row.get::<i32, _>("id"),
                            "contrato": row.get::<String, _>("contrato"),
                        })
                    })
                    .collect();
    
                let response_data = json!({
                    "status": true,
                    "message": "Consulta realizada com sucesso",
                    "data": dados,
                    "cached": false
                });
    
                // Salva no cache
                if let Err(e) = HelperCache::set_json_cartafianca(json!(dados)) {
                    println!("Erro ao salvar no cache: {}", e);
                }
    
                Ok(Json(response_data))
            }
            Err(e) => {
                println!("Erro: {:?}", e);
                Err((
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(json!({
                        "status": false,
                        "message": "Erro ao realizar consulta",
                        "error": e.to_string()
                    }))
                ))
            }
        }
    }


    pub async fn buscar_totas_analises_id(id: &str) -> Result<Json<Value>, (StatusCode, Json<Value>)> {
        // Inicializa o cache
        HelperCache::initialize();
    
        // Tenta buscar do cache primeiro
        if let Some(cached_item) = HelperCache::get_item_by_id(id) {
            return Ok(Json(json!({
                "status": true,
                "message": "Dados recuperados do cache",
                "data": cached_item,
                "cached": true
            })));
        }
    
        // Se não estiver no cache ou o item específico não for encontrado,
        // primeiro verifica se precisamos carregar todos os dados
        if HelperCache::get_array_cartafianca().is_none() {
            // Busca todos os dados do banco
            let query: &str = "SELECT * FROM VW_CARTAFIANCA_GERAL";
            
            match HelperMySql::execute_select(query).await {
                Ok(results) => {
                    let dados: Vec<Value> = results
                        .iter()
                        .map(|row| {
                            json!({
                                "id": row.get::<i32, _>("id"),
                                "contrato": row.get::<String, _>("contrato")
                            })
                        })
                        .collect();
    
                    // Salva no cache
                    if let Err(e) = HelperCache::set_json_cartafianca(json!(dados)) {
                        println!("Erro ao salvar no cache: {}", e);
                    }
    
                    // Tenta buscar o item específico novamente do cache
                    if let Some(item) = HelperCache::get_item_by_id(id) {
                        return Ok(Json(json!({
                            "status": true,
                            "message": "Dados recuperados do banco",
                            "data": item,
                            "cached": false
                        })));
                    }
                }
                Err(e) => {
                    println!("Erro: {:?}", e);
                    return Err((
                        StatusCode::INTERNAL_SERVER_ERROR,
                        Json(json!({
                            "status": false,
                            "message": "Erro ao realizar consulta",
                            "error": e.to_string()
                        }))
                    ));
                }
            }
        }
    
        // Se chegou aqui, o item não foi encontrado
        Err((
            StatusCode::NOT_FOUND,
            Json(json!({
                "status": false,
                "message": format!("Análise com ID {} não encontrada", id)
            }))
        ))
    }
}