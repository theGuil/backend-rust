use std::sync::Mutex;
use std::time::{Duration, SystemTime, Instant};
use serde_json::{Value, json};
use std::collections::HashMap;

static CACHE: Mutex<Option<HelperCache>> = Mutex::new(None);

struct CacheItem {
    array_data: Value,
    grouped_data: HashMap<String, Value>,    // Agrupado por imob_id
    indexed_data: HashMap<String, Value>,    // Indexado por id
    expires_at: SystemTime,
}

pub struct HelperCache {
    cartafianca_data: Option<CacheItem>,
    ttl: Duration,
}

impl HelperCache {
    pub fn get_instance() -> &'static Mutex<Option<HelperCache>> {
        &CACHE
    }

    pub fn initialize() {
        let mut cache = CACHE.lock().unwrap();
        if cache.is_none() {
            *cache = Some(HelperCache {
                cartafianca_data: None,
                ttl: Duration::from_secs(3600),
            });
        }
    }

    pub fn set_ttl(seconds: u64) -> Result<(), String> {
        let mut cache = CACHE.lock().map_err(|e| e.to_string())?;
        if let Some(cache_manager) = cache.as_mut() {
            cache_manager.ttl = Duration::from_secs(seconds);
        }
        Ok(())
    }

    fn is_expired(cache_item: &CacheItem) -> bool {
        SystemTime::now()
            .duration_since(cache_item.expires_at)
            .is_ok()
    }

    // Função para criar índice por ID
    fn array_to_indexed_map(array_data: &Value) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        
        if let Some(array) = array_data.as_array() {
            map.reserve(array.len());
            
            for item in array {
                if let Some(id) = item.get("id") {
                    let id_str = id.as_i64()
                        .map(|id| id.to_string())
                        .or_else(|| id.as_str().map(|s| s.to_string()))
                        .unwrap_or_default();

                    map.insert(id_str, item.clone());
                }
            }
        }
        
        map
    }

    // Função para agrupar por imob_id
    fn array_to_grouped_map(array_data: &Value) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        
        if let Some(array) = array_data.as_array() {
            map.reserve(array.len());
            
            let mut temp_map: HashMap<String, Vec<Value>> = HashMap::new();
            
            for item in array {
                if let Some(imob_id) = item.get("imob_id") {
                    let imob_id_str = imob_id.as_i64()
                        .map(|id| id.to_string())
                        .or_else(|| imob_id.as_str().map(|s| s.to_string()))
                        .unwrap_or_default();

                    temp_map.entry(imob_id_str)
                        .or_insert_with(Vec::new)
                        .push(item.clone());
                }
            }

            for (key, values) in temp_map {
                map.insert(key, json!(values));
            }
        }
        
        map
    }

    pub fn set_json_cartafianca(json_data: Value) -> Result<(), String> {
        let start = Instant::now();
        let mut cache = CACHE.lock().map_err(|e| e.to_string())?;
        
        if let Some(cache_manager) = cache.as_mut() {
            let grouped_data = Self::array_to_grouped_map(&json_data);
            let indexed_data = Self::array_to_indexed_map(&json_data);
            
            let cache_item = CacheItem {
                array_data: json_data,
                grouped_data,
                indexed_data,
                expires_at: SystemTime::now() + cache_manager.ttl,
            };
            
            cache_manager.cartafianca_data = Some(cache_item);
        }
        
        println!("Tempo de salvamento no cache: {:?}", start.elapsed());
        Ok(())
    }

    pub fn get_array_cartafianca() -> Option<Value> {
        let start = Instant::now();
        let cache = CACHE.lock().ok()?;
        
        if let Some(cache_manager) = cache.as_ref() {
            if let Some(cache_item) = &cache_manager.cartafianca_data {
                if !Self::is_expired(cache_item) {
                    println!("Tempo de acesso ao array do cache: {:?}", start.elapsed());
                    return Some(cache_item.array_data.clone());
                }
            }
        }
        
        None
    }

    pub fn get_grupo_cartafianca_by_imob() -> Option<Value> {
        let start = Instant::now();
        let cache = CACHE.lock().ok()?;
        
        if let Some(cache_manager) = cache.as_ref() {
            if let Some(cache_item) = &cache_manager.cartafianca_data {
                if !Self::is_expired(cache_item) {
                    let result = json!(cache_item.grouped_data);
                    println!("Tempo de acesso aos grupos do cache: {:?}", start.elapsed());
                    return Some(result);
                }
            }
        }
        
        None
    }

    pub fn get_cartafianca_by_imob_id(imob_id: &str) -> Option<Value> {
        //let start = Instant::now();
        let cache = CACHE.lock().ok()?;
        
        if let Some(cache_manager) = cache.as_ref() {
            if let Some(cache_item) = &cache_manager.cartafianca_data {
                if !Self::is_expired(cache_item) {
                    let result = cache_item.grouped_data.get(imob_id).cloned();
                    //println!("Tempo de acesso por imob_id no cache: {:?}", start.elapsed());
                    return result;
                }
            }
        }
        
        None
    }

    // Buscar item específico pelo ID
    pub fn get_item_by_id(id: &str) -> Option<Value> {
        //let start = Instant::now();
        let cache = CACHE.lock().ok()?;
        
        if let Some(cache_manager) = cache.as_ref() {
            if let Some(cache_item) = &cache_manager.cartafianca_data {
                if !Self::is_expired(cache_item) {
                    let result = cache_item.indexed_data.get(id).cloned();
                    //println!("Tempo de acesso por ID no cache: {:?}", start.elapsed());
                    return result;
                }
            }
        }
        
        None
    }

    pub fn clear_cartafianca_cache() -> Result<(), String> {
        let start = Instant::now();
        let mut cache = CACHE.lock().map_err(|e| e.to_string())?;
        if let Some(cache_manager) = cache.as_mut() {
            cache_manager.cartafianca_data = None;
        }
        println!("Tempo de limpeza do cache: {:?}", start.elapsed());
        Ok(())
    }

    pub fn get_cache_status() -> Result<Value, String> {
        let start = Instant::now();
        let cache = CACHE.lock().map_err(|e| e.to_string())?;
        
        let status = match &*cache {
            Some(cache_manager) => {
                match &cache_manager.cartafianca_data {
                    Some(cache_item) => {
                        let is_expired = Self::is_expired(cache_item);
                        json!({
                            "has_data": true,
                            "expired": is_expired,
                            "ttl_seconds": cache_manager.ttl.as_secs(),
                            "items_count": cache_item.array_data.as_array().map_or(0, |arr| arr.len()),
                            "groups_count": cache_item.grouped_data.len(),
                            "indexed_count": cache_item.indexed_data.len()
                        })
                    },
                    None => json!({
                        "has_data": false,
                        "ttl_seconds": cache_manager.ttl.as_secs()
                    })
                }
            },
            None => json!({
                "initialized": false
            })
        };

        println!("Tempo de verificação do status: {:?}", start.elapsed());
        Ok(status)
    }
}