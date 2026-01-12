use crate::pb::abi::{
    CommandRequest, CommandResponse, KvPair, Value, command_request::RequestData,
    value::Value as ValueEnum,
};
use crate::storage::MemTable;
use std::sync::Arc;

/// 服务层，处理业务逻辑
#[derive(Clone)]
pub struct Service {
    inner: Arc<Inner>,
}

struct Inner {
    store: MemTable,
}

impl Service {
    pub fn new(store: MemTable) -> Self {
        Self {
            inner: Arc::new(Inner { store }),
        }
    }

    pub fn execute(&self, cmd: CommandRequest) -> CommandResponse {
        match cmd.request_data {
            Some(RequestData::Hget(v)) => self.hget(&v.table, &v.key),
            Some(RequestData::Hmget(v)) => self.hmget(&v.table, &v.keys),
            Some(RequestData::Hgetall(v)) => self.hgetall(&v.table),
            Some(RequestData::Hset(v)) => self.hset(&v.table, v.pair),
            Some(RequestData::Hmset(v)) => self.hmset(&v.table, v.pairs),
            Some(RequestData::Hdel(v)) => self.hdel(&v.table, &v.key),
            Some(RequestData::Hmdel(v)) => self.hmdel(&v.table, &v.keys),
            Some(RequestData::Hexist(v)) => self.hexist(&v.table, &v.key),
            Some(RequestData::Hmexist(v)) => self.hmexist(&v.table, &v.keys),
            None => CommandResponse {
                status: 400,
                message: "No request data".to_string(),
                values: vec![],
                pairs: vec![],
            },
        }
    }

    fn hget(&self, table: &str, key: &str) -> CommandResponse {
        match self.inner.store.hget(table, key) {
            Some(value) => CommandResponse {
                status: 200,
                message: "OK".to_string(),
                values: vec![value],
                pairs: vec![],
            },
            None => CommandResponse {
                status: 404,
                message: "Key not found".to_string(),
                values: vec![],
                pairs: vec![],
            },
        }
    }

    fn hmget(&self, table: &str, keys: &[String]) -> CommandResponse {
        let values = self.inner.store.hmget(table, keys);
        let values = values.into_iter().filter_map(|v| v).collect();
        CommandResponse {
            status: 200,
            message: "OK".to_string(),
            values,
            pairs: vec![],
        }
    }

    fn hgetall(&self, table: &str) -> CommandResponse {
        let pairs = self.inner.store.hgetall(table);
        CommandResponse {
            status: 200,
            message: "OK".to_string(),
            values: vec![],
            pairs,
        }
    }

    fn hset(&self, table: &str, pair: Option<KvPair>) -> CommandResponse {
        if let Some(pair) = pair {
            self.inner.store.hset(table, pair);
            CommandResponse {
                status: 200,
                message: "OK".to_string(),
                values: vec![],
                pairs: vec![],
            }
        } else {
            CommandResponse {
                status: 400,
                message: "Pair is required".to_string(),
                values: vec![],
                pairs: vec![],
            }
        }
    }

    fn hmset(&self, table: &str, pairs: Vec<KvPair>) -> CommandResponse {
        self.inner.store.hmset(table, pairs);
        CommandResponse {
            status: 200,
            message: "OK".to_string(),
            values: vec![],
            pairs: vec![],
        }
    }

    fn hdel(&self, table: &str, key: &str) -> CommandResponse {
        if self.inner.store.hdel(table, key).is_some() {
            CommandResponse {
                status: 200,
                message: "OK".to_string(),
                values: vec![],
                pairs: vec![],
            }
        } else {
            CommandResponse {
                status: 404,
                message: "Key not found".to_string(),
                values: vec![],
                pairs: vec![],
            }
        }
    }

    fn hmdel(&self, table: &str, keys: &[String]) -> CommandResponse {
        self.inner.store.hmdel(table, keys);
        CommandResponse {
            status: 200,
            message: "OK".to_string(),
            values: vec![],
            pairs: vec![],
        }
    }

    fn hexist(&self, table: &str, key: &str) -> CommandResponse {
        let exists = self.inner.store.hexist(table, key);
        CommandResponse {
            status: 200,
            message: "OK".to_string(),
            values: vec![Value {
                value: Some(ValueEnum::Boolean(exists)),
            }],
            pairs: vec![],
        }
    }

    fn hmexist(&self, table: &str, keys: &[String]) -> CommandResponse {
        let results = self.inner.store.hmexist(table, keys);
        let values = results
            .into_iter()
            .map(|b| Value {
                value: Some(ValueEnum::Boolean(b)),
            })
            .collect();
        CommandResponse {
            status: 200,
            message: "OK".to_string(),
            values,
            pairs: vec![],
        }
    }
}

impl Default for Service {
    fn default() -> Self {
        Self::new(MemTable::new())
    }
}
