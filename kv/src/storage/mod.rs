use crate::pb::abi::{KvPair, Value};
use std::collections::HashMap;
use std::sync::{Arc, RwLock};

/// 内存存储引擎
#[derive(Debug, Default, Clone)]
pub struct MemTable {
    data: Arc<RwLock<HashMap<String, HashMap<String, Value>>>>,
}

impl MemTable {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn hget(&self, table: &str, key: &str) -> Option<Value> {
        let data = self.data.read().unwrap();
        data.get(table)?.get(key).cloned()
    }

    pub fn hset(&self, table: &str, pair: KvPair) {
        let mut data = self.data.write().unwrap();
        let table_data = data.entry(table.to_string()).or_insert_with(HashMap::new);
        table_data.insert(pair.key, pair.value.unwrap());
    }

    pub fn hgetall(&self, table: &str) -> Vec<KvPair> {
        let data = self.data.read().unwrap();
        data.get(table)
            .map(|table_data| {
                table_data
                    .iter()
                    .map(|(k, v)| KvPair {
                        key: k.clone(),
                        value: Some(v.clone()),
                    })
                    .collect()
            })
            .unwrap_or_default()
    }

    pub fn hdel(&self, table: &str, key: &str) -> Option<Value> {
        let mut data = self.data.write().unwrap();
        data.get_mut(table)?.remove(key)
    }

    pub fn hexist(&self, table: &str, key: &str) -> bool {
        let data = self.data.read().unwrap();
        data.get(table)
            .map(|table_data| table_data.contains_key(key))
            .unwrap_or(false)
    }

    pub fn hmget(&self, table: &str, keys: &[String]) -> Vec<Option<Value>> {
        let data = self.data.read().unwrap();
        keys.iter()
            .map(|key| {
                data.get(table)
                    .and_then(|table_data| table_data.get(key))
                    .cloned()
            })
            .collect()
    }

    pub fn hmset(&self, table: &str, pairs: Vec<KvPair>) {
        let mut data = self.data.write().unwrap();
        let table_data = data.entry(table.to_string()).or_insert_with(HashMap::new);
        for pair in pairs {
            if let Some(value) = pair.value {
                table_data.insert(pair.key, value);
            }
        }
    }

    pub fn hmdel(&self, table: &str, keys: &[String]) -> Vec<Option<Value>> {
        let mut data = self.data.write().unwrap();
        if let Some(table_data) = data.get_mut(table) {
            keys.iter().map(|key| table_data.remove(key)).collect()
        } else {
            vec![None; keys.len()]
        }
    }

    pub fn hmexist(&self, table: &str, keys: &[String]) -> Vec<bool> {
        let data = self.data.read().unwrap();
        keys.iter()
            .map(|key| {
                data.get(table)
                    .map(|table_data| table_data.contains_key(key))
                    .unwrap_or(false)
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::pb::abi::value::Value as ValueEnum;

    #[test]
    fn test_hset_hget() {
        let table = MemTable::new();

        let value = Value {
            value: Some(ValueEnum::String("hello".to_string())),
        };
        let pair = KvPair {
            key: "name".to_string(),
            value: Some(value),
        };

        table.hset("user", pair);

        let result = table.hget("user", "name");
        assert!(result.is_some());
        if let Some(v) = result {
            if let Some(ValueEnum::String(s)) = v.value {
                assert_eq!(s, "hello");
            } else {
                panic!("Expected String value");
            }
        }
    }
}
