use std::sync::Mutex;
use std::time::{Duration, SystemTime};
use serde_json::Value;
use std::collections::HashMap;

// Singleton do Cache usando static
static CACHE: Mutex<Option<HelperCache>> = Mutex::new(None);

struct CacheItem {
    array_data: Value,  // Mantém o array original
    map_data: HashMap<String, Value>,  // Mantém o objeto indexado por ID
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

    fn is_expired(cache_item: &CacheItem) -> bool {
        SystemTime::now()
            .duration_since(cache_item.expires_at)
            .is_ok()
    }

    pub fn set_ttl(seconds: u64) -> Result<(), String> {
        let mut cache = CACHE.lock().map_err(|e| e.to_string())?;
        if let Some(cache_manager) = cache.as_mut() {
            cache_manager.ttl = Duration::from_secs(seconds);
        }
        Ok(())
    }

    // Função auxiliar para converter array em HashMap
    fn array_to_map(array_data: &Value) -> HashMap<String, Value> {
        let mut map = HashMap::new();
        
        if let Some(array) = array_data.as_array() {
            for item in array {
                if let Some(id) = item.get("id") {
                    if let Some(id_str) = id.as_str() {
                        map.insert(id_str.to_string(), item.clone());
                    } else if let Some(id_num) = id.as_i64() {
                        map.insert(id_num.to_string(), item.clone());
                    }
                }
            }
        }
        
        map
    }

    // Salvar dados no cache (agora mantém ambos os formatos)
    pub fn set_json_cartafianca(json_data: Value) -> Result<(), String> {
        let mut cache = CACHE.lock().map_err(|e| e.to_string())?;
        
        if let Some(cache_manager) = cache.as_mut() {
            let map_data = Self::array_to_map(&json_data);
            
            let cache_item = CacheItem {
                array_data: json_data,
                map_data,
                expires_at: SystemTime::now() + cache_manager.ttl,
            };
            
            cache_manager.cartafianca_data = Some(cache_item);
        }
        
        Ok(())
    }

    // Pegar dados como array
    pub fn get_array_cartafianca() -> Option<Value> {
        let cache = CACHE.lock().ok()?;
        
        if let Some(cache_manager) = cache.as_ref() {
            if let Some(cache_item) = &cache_manager.cartafianca_data {
                if !Self::is_expired(cache_item) {
                    return Some(cache_item.array_data.clone());
                }
            }
        }
        
        None
    }

    // Pegar dados como objeto indexado por ID
    pub fn get_objeto_cartafianca_id() -> Option<HashMap<String, Value>> {
        let cache = CACHE.lock().ok()?;
        
        if let Some(cache_manager) = cache.as_ref() {
            if let Some(cache_item) = &cache_manager.cartafianca_data {
                if !Self::is_expired(cache_item) {
                    return Some(cache_item.map_data.clone());
                }
            }
        }
        
        None
    }

    // Pegar um item específico pelo ID
    pub fn get_item_by_id(id: &str) -> Option<Value> {
        let cache = CACHE.lock().ok()?;
        
        if let Some(cache_manager) = cache.as_ref() {
            if let Some(cache_item) = &cache_manager.cartafianca_data {
                if !Self::is_expired(cache_item) {
                    return cache_item.map_data.get(id).cloned();
                }
            }
        }
        
        None
    }

    pub fn clear_cartafianca_cache() -> Result<(), String> {
        let mut cache = CACHE.lock().map_err(|e| e.to_string())?;
        if let Some(cache_manager) = cache.as_mut() {
            cache_manager.cartafianca_data = None;
        }
        Ok(())
    }
}