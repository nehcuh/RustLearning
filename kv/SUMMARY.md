# KV Server 项目总结

## 项目概述

这是一个基于 Rust 实现的简单键值存储服务器，使用 Protocol Buffers 作为通信协议，Tokio 作为异步运行时。项目采用分层架构设计，支持多种 Hash 操作和数据类型。

## 核心功能

### 支持的操作
- **HGET** - 获取单个键的值
- **HMGET** - 批量获取多个键的值
- **HGETALL** - 获取表中所有键值对
- **HSET** - 设置单个键值对
- **HMSET** - 批量设置键值对
- **HDEL** - 删除单个键
- **HMDEL** - 批量删除键
- **HEXIST** - 检查单个键是否存在
- **HMEXIST** - 批量检查键是否存在

### 支持的数据类型
- String
- Binary
- Integer (i64)
- Float (f64)
- Boolean

## 技术栈

- **语言**: Rust (edition 2024)
- **异步运行时**: Tokio
- **序列化**: Protocol Buffers (prost)
- **网络**: tokio-util codec
- **测试**: 内置测试框架

## 项目架构

### 分层设计

```
┌─────────────────────────────────────────┐
│          Network Layer (网络层)          │
│  Server / Client - TCP 通信处理          │
└─────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────┐
│           Codec Layer (编解码层)         │
│  ProstCodec - Protobuf 序列化/反序列化   │
└─────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────┐
│          Service Layer (服务层)          │
│  业务逻辑处理 - 命令路由和响应生成        │
└─────────────────────────────────────────┘
                    ↓
┌─────────────────────────────────────────┐
│        Storage Layer (存储层)            │
│  MemTable - 内存存储引擎 (HashMap)       │
└─────────────────────────────────────────┘
```

### 模块说明

#### 1. `src/codec.rs` - 编解码层
- 实现 `ProstCodec` 结构体
- 处理 Protocol Buffers 消息的编码和解码
- 使用帧格式：[4字节长度][数据内容]

#### 2. `src/storage/mod.rs` - 存储层
- 实现 `MemTable` 内存存储引擎
- 使用 `RwLock` 实现并发安全的读写
- 提供 Hash 操作的底层实现

#### 3. `src/service/mod.rs` - 服务层
- 实现 `Service` 结构体处理业务逻辑
- 命令路由和响应生成
- 错误处理和状态码管理

#### 4. `src/network/mod.rs` - 网络层
- 实现 `Server` 和 `Client`
- 处理 TCP 连接和消息传递
- 使用 `Framed` 进行流处理

#### 5. `src/pb/` - Protocol Buffers 生成代码
- `abi.rs` - 根据 `abi.proto` 自动生成
- `mod.rs` - Protobuf 模块封装

## 文件结构

```
kv/
├── abi.proto              # Protocol Buffers 定义
├── build.rs               # 构建脚本
├── Cargo.toml             # 项目配置
├── run_test.sh            # 自动化测试脚本
├── examples/
│   ├── server.rs          # 服务器启动示例
│   └── client.rs          # 客户端使用示例
├── src/
│   ├── codec.rs           # 编解码器
│   ├── lib.rs             # 库入口
│   ├── storage/
│   │   └── mod.rs         # 存储引擎
│   ├── service/
│   │   └── mod.rs         # 业务逻辑
│   ├── network/
│   │   └── mod.rs         # 网络层
│   └── pb/
│       ├── abi.rs         # Protobuf 生成代码
│       └── mod.rs         # Protobuf 模块
└── README.md              # 项目文档
```

## 使用方法

### 快速启动

1. **启动服务器**
```bash
cargo run --example server
```
服务器将在 `127.0.0.1:9876` 监听

2. **运行客户端测试**
```bash
cargo run --example client
```

3. **自动化测试**
```bash
./run_test.sh
```

### 代码集成

#### 创建自定义服务器
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

#### 创建自定义客户端
```rust
use kv::network::Client;
use kv::pb::abi::{CommandRequest, value::Value};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = Client::new("127.0.0.1:9876");

    // HSET
    let cmd = CommandRequest::new_hset("user", "name", Value::String("Alice".to_string()));
    let resp = client.send_command(cmd).await?;
    println!("Status: {}", resp.status);

    // HGET
    let cmd = CommandRequest {
        request_data: Some(kv::pb::abi::command_request::RequestData::Hget(
            kv::pb::abi::Hget {
                table: "user".to_string(),
                key: "name".to_string(),
            },
        )),
    };
    let resp = client.send_command(cmd).await?;
    if let Some(value) = resp.values.first() {
        println!("Value: {:?}", value);
    }

    Ok(())
}
```

## 测试

### 单元测试
```bash
cargo test
```

### 集成测试
```bash
./run_test.sh
```

测试覆盖：
- 存储层：HSET/HGET 操作
- 网络层：服务器/客户端通信
- 功能测试：完整的 CRUD 操作流程

## 协议设计

### 请求格式
```protobuf
message CommandRequest {
  oneof request_data {
    Hget hget = 1;
    Hmget hmget = 2;
    Hgetall hgetall = 3;
    Hset hset = 4;
    Hmset hmset = 5;
    Hdel hdel = 6;
    Hmdel hmdel = 7;
    Hexist hexist = 8;
    Hmexist hmexist = 9;
  }
}
```

### 响应格式
```protobuf
message CommandResponse {
  uint32 status = 1;       // 200 成功, 400 客户端错误, 404 未找到
  string message = 2;      // 响应消息
  repeated Value values = 3;
  repeated KvPair pairs = 4;
}
```

### 传输格式
```
[4字节长度][Protobuf消息]
```

## 性能特点

- **异步 I/O**: 基于 Tokio，支持高并发
- **零拷贝**: 使用 Bytes 和 BytesMut 优化内存使用
- **并发安全**: RwLock 实现读写锁，支持并发读
- **高效序列化**: Protocol Buffers 二进制格式

## 扩展方向

1. **持久化存储**: 添加磁盘存储支持（如 RocksDB）
2. **分布式**: 添加集群和副本支持
3. **认证**: 添加用户认证和权限控制
4. **事务**: 支持事务操作（MULTI/EXEC）
5. **过期时间**: 支持键的 TTL（Time To Live）
6. **监控**: 添加 metrics 和日志

## 依赖说明

```toml
[dependencies]
anyhow = "1.0.100"        # 错误处理
bytes = "1.11.0"          # 字节操作
prost = "0.14.3"          # Protocol Buffers
tokio = { version = "1.49.0", features = ["full"] }  # 异步运行时
tokio-util = { version = "0.7.18", features = ["codec"] }  # 编解码工具
futures = "0.3"           # 异步工具
```

## 注意事项

1. 当前实现使用内存存储，重启后数据会丢失
2. 每个连接都是独立的，不支持跨连接的数据共享（通过共享存储引擎）
3. 没有实现认证和授权机制
4. 生产环境需要添加更多错误处理和日志

## 总结

这个 KV Server 项目展示了如何使用 Rust 构建一个功能完整、架构清晰的网络服务。项目采用了现代 Rust 最佳实践，包括异步编程、错误处理、模块化设计等。通过这个项目，可以学习到：

- Protocol Buffers 在 Rust 中的使用
- Tokio 异步网络编程
- 分层架构设计
- 并发安全的实现
- 测试驱动开发

项目代码结构清晰，易于理解和扩展，是学习 Rust 网络编程的绝佳示例。
