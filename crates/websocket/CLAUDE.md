# WebSocket - WebSocket 客户端模块

[根目录](../../CLAUDE.md) > [crates](../) > **websocket**

> 最后更新：2026-05-01

## 模块职责

WebSocket 模块为 Warp 提供了一个跨平台的 WebSocket 客户端实现，支持原生平台和 WASM 目标。该模块实现了 `graphql-ws-client` 的 `WebsocketMessage` trait，可以直接用作 GraphQL WebSocket 客户端的底层传输。

**核心功能**：
- 跨平台 WebSocket 客户端（Native + WASM）
- 支持 GraphQL over WebSocket 协议
- HTTP 代理支持（原生平台）
- TLS/SSL 支持
- 统一的 Message 类型抽象

## 入口与启动

### 主要类型

**`WebSocket`** - WebSocket 客户端封装
- 提供跨平台的连接接口
- 支持拆分为独立的 Sink 和 Stream

**`Message`** - WebSocket 消息类型
- `Text(String)` - 文本消息
- `Binary(Vec<u8>)` - 二进制消息

### 使用示例

```rust
// 连接到 WebSocket
let socket = WebSocket::connect(
    "wss://example.com/graphql",
    ["graphql-transport-ws"]
).await?;

// 拆分为读写流
let (sink, stream) = socket.split().await;

// 或直接转换为 GraphQL 客户端
let graphql_client = socket.into_graphql_client_builder().await;
```

## 对外接口

### 核心接口

**`WebSocket::connect`**：
```rust
// 原生平台：支持自定义请求头
pub async fn connect(
    request: impl IntoClientRequest,
    protocols: impl IntoIterator<Item = &str>,
) -> anyhow::Result<Self>

// WASM 平台：仅支持 URL
pub async fn connect(
    url: impl AsRef<str>,
    protocols: impl IntoIterator<Item = &str>,
) -> anyhow::Result<Self>
```

**`WebSocket::split`**：
```rust
pub async fn split(self) -> (impl Sink, impl Stream)
```
返回独立的写入器和读取器。

**`WebSocket::into_graphql_client_builder`**：
```rust
pub async fn into_graphql_client_builder(self) -> graphql_ws_client::ClientBuilder
```
直接转换为 GraphQL WebSocket 客户端构建器。

### Trait 实现

**`WebsocketMessage` trait**：
- `new(text: String)` - 从文本创建消息
- `text(&self)` - 获取文本内容
- `new_binary(bytes: Vec<u8>)` - 从二进制创建消息
- `binary(&self)` - 获取二进制内容
- `new_text(text: String)` - 创建文本消息

## 关键依赖与配置

### 依赖项

**核心依赖**：
- `futures` / `futures-util` - 异步流和汇
- `thiserror` - 错误处理
- `anyhow` - 错误上下文
- `pin-project` - 固定大小投影

**GraphQL 集成**：
- `graphql-ws-client` - GraphQL WebSocket 协议支持

**原生平台依赖**：
- `async-tungstenite` - WebSocket 实现
- `tokio` / `tokio-rustls` - 异步运行时和 TLS
- `hyper` / `hyper-util` - HTTP 客户端（用于代理）
- `rustls` / `rustls-platform-verifier` - TLS 验证

**WASM 平台依赖**：
- `ws_stream_wasm` - WebSocket WASM 实现

### 配置

**环境变量**（原生平台代理支持）：
- `HTTPS_PROXY` - HTTPS 代理地址
- `HTTP_PROXY` - HTTP 代理地址
- `ALL_PROXY` - 通用代理地址
- `NO_PROXY` - 代理绕过列表

## 数据模型

### 消息类型

**`Message`** - WebSocket 消息封装
- 包装平台特定的消息类型
- 实现统一的 `WebsocketMessage` trait

**`Error`** - 错误类型
- 包装底层 WebSocket 错误
- 实现 `std::error::Error`

### 平台特定实现

**Native (`native.rs`)**：
- 基于 `async-tungstenite`
- 支持 HTTP 代理隧道
- 支持 TLS/SSL
- TCP 流连接

**WASM (`wasm.rs`)**：
- 基于 `ws_stream_wasm`
- 浏览器 WebSocket API
- 不支持自定义请求头

## 测试与质量

### 测试覆盖

✅ **单元测试**：20 个测试（主要在 `proxy_tests.rs`）
- 代理解析测试
- 代理连接测试
- NO_PROXY 测试

### 测试策略

**测试重点**：
1. 代理功能测试（`proxy_tests.rs`）
2. 消息序列化/反序列化测试
3. 跨平台兼容性测试

**运行测试**：
```bash
# 运行所有测试
cargo test -p websocket

# 运行代理测试
cargo test -p websocket --test proxy_tests
```

## 常见问题 (FAQ)

### Q: 如何在原生平台使用自定义请求头？

**A**:
```rust
use http::Request;

let mut request = Request::builder()
    .uri("wss://example.com/graphql")
    .header("Authorization", "Bearer token")
    .body(())?;

let socket = WebSocket::connect(request, ["graphql-ws"]).await?;
```

注意：WASM 平台不支持自定义请求头。

### Q: 如何配置代理？

**A**: 设置环境变量：
```bash
export HTTPS_PROXY=http://proxy.example.com:8080
export HTTP_PROXY=http://proxy.example.com:8080
export NO_PROXY=localhost,127.0.0.1,.local
```

模块会自动读取这些环境变量并使用代理。

### Q: WebSocket 连接失败如何调试？

**A**:
1. 检查网络连接和 DNS 解析
2. 验证 TLS 证书（原生平台使用系统验证器）
3. 查看代理配置是否正确
4. 启用日志记录（`log` crate）
5. 使用 WebSocket 测试工具验证服务器端

### Q: 如何处理连接断开和重连？

**A**:
```rust
loop {
    match WebSocket::connect(url, ["graphql-ws"]).await {
        Ok(socket) => {
            let (mut sink, mut stream) = socket.split().await;
            // 使用 sink 和 stream
            // 当连接断开时，循环会重新连接
        }
        Err(e) => {
            log::error!("Connection failed: {:?}", e);
            tokio::time::sleep(Duration::from_secs(5)).await;
        }
    }
}
```

### Q: WASM 平台有哪些限制？

**A**:
- ❌ 不支持自定义请求头
- ❌ 不支持 HTTP 代理
- ✅ 支持标准的 WebSocket 协议
- ✅ 支持 GraphQL over WebSocket

### Q: 如何与 GraphQL 集成？

**A**:
```rust
use graphql_ws_client::Client;

let socket = WebSocket::connect(url, ["graphql-transport-ws"]).await?;
let client = socket.into_graphql_client_builder().await;
// 使用 client 发送 GraphQL 查询
```

## 相关文件清单

### 核心文件

- `src/lib.rs` - 模块入口，公共 API
- `src/native.rs` - 原生平台实现
- `src/wasm.rs` - WASM 平台实现
- `src/proxy.rs` - 代理支持逻辑

### 测试文件

- `src/proxy_tests.rs` - 代理功能测试
- `src/sink_map_err_tests.rs` - 错误映射测试

### 辅助文件

- `src/sink_map_err.rs` - 错误类型转换工具

## 变更记录

### 2026-05-01

- ✅ 创建模块文档
- ✅ 记录跨平台接口差异
- ✅ 文档化代理配置方法
- ✅ 添加 GraphQL 集成示例

---

*本文档由 AI 自动生成和维护。如有问题或建议，请在 issue 中提出。*
