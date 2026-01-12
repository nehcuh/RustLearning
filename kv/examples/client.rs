use anyhow::Result;
use kv::network::Client;
use kv::pb::abi::{CommandRequest, value::Value};

#[tokio::main]
async fn main() -> Result<()> {
    let client = Client::new("127.0.0.1:9876");

    // HSET - 设置键值对
    println!("=== HSET ===");
    let cmd = CommandRequest::new_hset("user", "name", Value::String("Alice".to_string()));
    let resp = client.send_command(cmd).await?;
    println!("Status: {}, Message: {}", resp.status, resp.message);

    let cmd = CommandRequest::new_hset("user", "age", Value::Integer(30));
    let resp = client.send_command(cmd).await?;
    println!("Status: {}, Message: {}", resp.status, resp.message);

    // HGET - 获取单个键的值
    println!("\n=== HGET ===");
    let cmd = CommandRequest {
        request_data: Some(kv::pb::abi::command_request::RequestData::Hget(
            kv::pb::abi::Hget {
                table: "user".to_string(),
                key: "name".to_string(),
            },
        )),
    };
    let resp = client.send_command(cmd).await?;
    println!("Status: {}, Message: {}", resp.status, resp.message);
    if let Some(value) = resp.values.first() {
        println!("Value: {:?}", value);
    }

    // HMGET - 批量获取多个键的值
    println!("\n=== HMGET ===");
    let cmd = CommandRequest {
        request_data: Some(kv::pb::abi::command_request::RequestData::Hmget(
            kv::pb::abi::Hmget {
                table: "user".to_string(),
                keys: vec!["name".to_string(), "age".to_string()],
            },
        )),
    };
    let resp = client.send_command(cmd).await?;
    println!("Status: {}, Message: {}", resp.status, resp.message);
    for value in &resp.values {
        println!("Value: {:?}", value);
    }

    // HGETALL - 获取表中所有键值对
    println!("\n=== HGETALL ===");
    let cmd = CommandRequest {
        request_data: Some(kv::pb::abi::command_request::RequestData::Hgetall(
            kv::pb::abi::Hgetall {
                table: "user".to_string(),
            },
        )),
    };
    let resp = client.send_command(cmd).await?;
    println!("Status: {}, Message: {}", resp.status, resp.message);
    for pair in &resp.pairs {
        println!("Key: {}, Value: {:?}", pair.key, pair.value);
    }

    // HEXIST - 检查键是否存在
    println!("\n=== HEXIST ===");
    let cmd = CommandRequest {
        request_data: Some(kv::pb::abi::command_request::RequestData::Hexist(
            kv::pb::abi::Hexist {
                table: "user".to_string(),
                key: "name".to_string(),
            },
        )),
    };
    let resp = client.send_command(cmd).await?;
    println!("Status: {}, Message: {}", resp.status, resp.message);
    if let Some(value) = resp.values.first() {
        if let Some(Value::Boolean(b)) = &value.value {
            println!("Name exists: {}", b);
        }
    }

    // HDEL - 删除键
    println!("\n=== HDEL ===");
    let cmd = CommandRequest {
        request_data: Some(kv::pb::abi::command_request::RequestData::Hdel(
            kv::pb::abi::Hdel {
                table: "user".to_string(),
                key: "age".to_string(),
            },
        )),
    };
    let resp = client.send_command(cmd).await?;
    println!("Status: {}, Message: {}", resp.status, resp.message);

    // 验证删除
    let cmd = CommandRequest {
        request_data: Some(kv::pb::abi::command_request::RequestData::Hgetall(
            kv::pb::abi::Hgetall {
                table: "user".to_string(),
            },
        )),
    };
    let resp = client.send_command(cmd).await?;
    println!("\nAfter HDEL, remaining pairs:");
    for pair in &resp.pairs {
        println!("Key: {}, Value: {:?}", pair.key, pair.value);
    }

    Ok(())
}
