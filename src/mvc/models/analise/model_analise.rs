use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Analise {
    pub id: String,
    pub titulo: String,
    pub descricao: String,
    pub status: String,
    pub created_at: String,
}

#[derive(Debug, Deserialize)]
pub struct CreateAnalise {
    pub titulo: String,
    pub descricao: String,
}

impl Analise {
    pub fn new(payload: CreateAnalise) -> Self {
        let now: chrono::DateTime<chrono::Local> = chrono::Local::now();
        
        Self {
            id: Uuid::new_v4().to_string(),
            titulo: payload.titulo,
            descricao: payload.descricao,
            status: "pendente".to_string(),
            created_at: now.to_rfc3339(),
        }
    }
    
    pub fn listar() -> Vec<Analise> {
        // Aqui você implementaria a lógica para buscar do banco de dados
        print!("Passou dentro da análise!");
        vec![]
    }
    
    pub fn buscar_por_id(_id: String) -> Option<Analise> {
        // Aqui você implementaria a lógica para buscar a análise do banco de dados
        Some(Analise {
            id: _id,
            titulo: "Análise Exemplo".to_string(),
            descricao: "Descrição da análise".to_string(),
            status: "pendente".to_string(),
            created_at: "2024-10-22T12:34:56Z".to_string(),
        })
    }
}