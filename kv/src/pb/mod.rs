pub mod abi;

impl abi::CommandRequest {
    pub fn new_hset(
        table: impl Into<String>,
        key: impl Into<String>,
        value: abi::value::Value,
    ) -> Self {
        Self {
            request_data: Some(abi::command_request::RequestData::Hset(abi::Hset {
                table: table.into(),
                pair: Some(abi::KvPair::new(key, value)),
            })),
        }
    }
}

impl abi::KvPair {
    pub fn new(key: impl Into<String>, value: abi::value::Value) -> Self {
        Self {
            key: key.into(),
            value: Some(abi::Value { value: Some(value) }),
        }
    }
}

impl From<String> for abi::value::Value {
    fn from(value: String) -> Self {
        Self::String(value)
    }
}
