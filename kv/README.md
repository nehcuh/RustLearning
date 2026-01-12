# KV Server

一个基于 Rust 的简单键值存储服务器，支持内存存储和 Protocol Buffers 通信协议。

## 功能特性

- 支持多种 Hash 操作：
  - `HGET` - 获取单个键的值
  - `HMGET` - 批量获取多个键的值
  - `HGETALL` - 获取表中所有键值对
  - `HSET` - 设置单个键值对
  - `HMSET` - 批量设置键值对
  - `HDEL` - 删除单个键
  - `HMDEL` - 批量删除键
  - `HEXIST` - 检查单个键是否存在
  - `HMEXIST` - 批量检查键是否存在
- 支持多种数据类型：String, Binary, Integer, Float, Boolean
- 基于 Protocol Buffers 的高效序列化
- 使用 Tokio 实现异步网络通信
- 内存存储引擎，支持并发访问

## 项目结构

```
kv/
├── abi.proto           # Protocol Buffers 定义文件
├── build.rs            # 构建脚本
├── Cargo.toml          # 项目依赖配置
├── examples/
│   ├── server.rs       # 服务器示例
│   └── client.rs       # 客户端示例
└── src/
    ├── codec.rs        # Protocol Buffers 编解码器
    ├── lib.rs          # 库入口
    ├── network/
    │   └── mod.rs      # 网络层（Server 和 Client）
    ├── pb/
    │   ├── abi.rs      # Protocol Buffers 生成的代码
    │   └── mod.rs      # Protobuf 模块
    ├── service/
    │   └── mod.rs      # 业务逻辑层
    └── storage/
        └── mod.rs      # 存储引擎层
```

## 快速开始

### 前置要求

- Rust 1.70 或更高版本
- Protocol Buffers 编译器（可选，用于修改 proto 文件）

### 编译项目

```bash
cargo build
```

### 运行服务器

在终端 1 中：

```bash
cargo run --example server
```

服务器将在 `127.0.0.1:9876` 上监听。

### 运行客户端

在终端 2 中：

```bash
cargo run --example client
```

## 使用示例

### 在代码中使用

#### 创建服务器

```rust
use kv::network::Server;
use kv::service::Service;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let service = Service::default();
    let server = Server::new("127.0.0.1:9876", service);
    server.run().await?;
    Ok(())
}
```

#### 创建客户端

```rust
use kv::network::Client;
use kv::pb::abi::{CommandRequest, value::Value};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new("127.0.0.1:9876");

    // HSET - 设置键值对
    let cmd = CommandRequest::new_hset("user", "name", Value::String("Alice".to_string()));
    let response = client.send_command(cmd).await?;
    println!("Status: {}", response.status);

    // HGET - 获取键值
    let cmd = CommandRequest {
        request_data: Some(kv::pb::abi::command_request::RequestData::Hget(
            kv::pb::abi::Hget {
                table: "user".to_string(),
                key: "name".to_string(),
            },
        )),
    };
    let response = client.send_command(cmd).await?;
    if let Some(value) = response.values.first() {
        println!("Value: {:?}", value);
    }

    Ok(())
}
```

## API 说明

### 命令格式

所有命令都通过 `CommandRequest` 结构发送，并通过 `CommandResponse` 接收响应。

### 命令类型

#### HGET

```protobuf
message Hget {
  string table = 1;
  string key = 2;
}
```

获取指定表中单个键的值。

#### HMGET

```protobuf
message Hmget {
  string table = 1;
  repeated string keys = 2;
}
```

批量获取指定表中多个键的值。

#### HGETALL

```protobuf
message Hgetall {
  string table = 1;
}
```

获取指定表中所有键值对。

#### HSET

```protobuf
message Hset {
  string table = 1;
  KvPair pair = 2;
}
```

在指定表中设置单个键值对。

#### HMSET

```protobuf
message Hmset {
  string table = 1;
  repeated KvPair pairs = 2;
}
```

在指定表中批量设置键值对。

#### HDEL

```protobuf
message Hdel {
  string table = 1;
  string key = 2;
}
```

删除指定表中单个键。

#### HMDEL

```protobuf
message Hmdel {
  string table = 1;
  repeated string keys = 2;
}
```

批量删除指定表中的多个键。

#### HEXIST

```protobuf
message Hexist {
  string table = 1;
  string key = 2;
}
```

检查指定表中单个键是否存在。

#### HMEXIST

```protobuf
message Hmexist {
  string table = 1;
  repeated string keys = 2;
}
```

批量检查指定表中多个键是否存在。

### 响应格式

```protobuf
message CommandResponse {
  uint32 status = 1;       // 状态码：200 成功，400 客户端错误，404 未找到
  string message = 2;      // 响应消息
  repeated Value values = 3;  // 返回的值列表
  repeated KvPair pairs = 4;  // 返回的键值对列表
}
```

## 运行测试

```bash
cargo test
```

## 构建 Protocol Buffers

如果修改了 `abi.proto` 文件，需要重新生成 Rust 代码：

```bash
cargo build
```

`build.rs` 会自动处理 Protocol Buffers 的编译。

## 架构说明

项目采用分层架构：

1. **存储层（Storage）**：负责数据的存储和检索，使用内存 HashMap 实现
2. **服务层（Service）**：处理业务逻辑，调用存储层执行操作
3. **网络层（Network）**：处理网络通信，使用 Protocol Buffers 进行序列化
4. **编解码层（Codec）**：负责 Protocol Buffers 消息的编码和解码

## License

MIT License
