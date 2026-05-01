[根目录](../../CLAUDE.md) > [crates](../) > **http_client**

# http_client - HTTP 客户端

## 模块职责

`http_client` 为 Warp 应用程序提供统一的 HTTP 客户端抽象，封装 `reqwest` 库以支持跨平台异步 HTTP 请求（包括 WASM）。该模块提供请求/响应钩子、自定义头部、EventSource 支持和 OAuth2 集成。

## 入口与启动

- **主入口文件**: `src/lib.rs`
- **核心类型**:
  - `Client` - HTTP 客户端包装器
  - `RequestBuilder` - 请求构建器
  - `EventSourceStream` - Server-Sent Events 流

## 对外接口

### HTTP 客户端 (Client)

```rust
use http_client::Client;

// 创建默认客户端
let client = Client::default();

// 创建带钩子的客户端
let client = Client::new()
    .with_before_request_hook(|request, json_payload| {
        // 在发送前修改请求
        log::info!("Sending request to {:?}", request.url());
    })
    .with_after_response_hook(|response| {
        // 处理响应
        log::info!("Received response: {:?}", response.status());
    });
```

### 发送请求

```rust
use http_client::Client;
use serde::Serialize;

#[derive(Serialize)]
struct Body {
    name: String,
    value: i32,
}

let client = Client::default();

// GET 请求
let response = client.get("https://api.example.com/data").await?;

// POST JSON
let response = client
    .post("https://api.example.com/create")
    .json(&Body { name: "test".to_string(), value: 42 })
    .await?;

// POST 表单
let response = client
    .post("https://api.example.com/form")
    .form(&[("key", "value")])
    .await?;

// 带自定义头部
let response = client
    .get("https://api.example.com/data")
    .header("Authorization", "Bearer token")
    .await?;
```

### EventSource (Server-Sent Events)

```rust
use http_client::Client;
use futures::StreamExt;

let client = Client::default();

// 创建 EventSource 连接
let mut event_stream = client
    .event_source("https://api.example.com/events")
    .await?;

// 处理事件
while let Some(event) = event_stream.next().await {
    match event {
        Ok(event) => {
            println!("Event: {:?}", event);
        },
        Err(error) => {
            eprintln!("Error: {:?}", error);
        }
    }
}
```

### 自定义头部

Warp 自动添加以下自定义头部：

```rust
// 模块定义的常量
pub const CLIENT_RELEASE_VERSION_HEADER_KEY: &str = "X-Warp-Client-Version";
pub const WARP_OS_CATEGORY: &str = "X-Warp-OS-Category";
pub const WARP_OS_NAME: &str = "X-Warp-OS-Name";
pub const WARP_OS_VERSION: &str = "X-Warp-OS-Version";
pub const WARP_OS_LINUX_KERNEL_VERSION: &str = "X-Warp-OS-Linux-Kernel-Version";
pub const WARP_CLIENT_ID: &str = "X-Warp-Client-ID";
```

**环境变量配置**:
- `WARP_EXTRA_HTTP_HEADERS` - 额外的 HTTP 头部（仅 `Channel::Integration`）
  - 格式：换行分隔的 `Name:Value` 对

## 关键依赖与配置

### 依赖关系

```toml
[dependencies]
reqwest = { workspace = true }              # HTTP 客户端
reqwest-eventsource = { workspace = true }  # SSE 支持
http = { workspace = true }                 # HTTP 类型
serde = { workspace = true }                # 序列化
serde_json = { workspace = true }           # JSON 序列化
bytes = { workspace = true }                # 字节缓冲
futures = { workspace = true }              # 异步工具
async-compat = { workspace = true }         # Tokio 兼容性
async-stream = { workspace = true }         # 异步流
oauth2 = { workspace = true }               # OAuth2 支持
warp_core = { workspace = true }            # 核心工具
```

### 平台特定依赖

```toml
[target.'cfg(target_arch = "wasm32")'.dependencies]
gloo = { workspace = true }                      # WASM 工具
wasm-bindgen-futures = { workspace = true }      # WASM 异步
```

### 特性标志

- `test-util` - 测试工具支持

## 数据模型

### Client

HTTP 客户端包装器：

```rust
pub struct Client {
    wrapped: reqwest::Client,
    before_request_sent: Option<RequestHookFn>,
    after_response_received: Option<ResponseHookFn>,
}
```

**关键特性**:
- 包装 `reqwest::Client`
- 支持请求前/响应后钩子
- 自动处理 Tokio 兼容性（通过 `async-compat`）

### RequestBuilder

请求构建器：

```rust
pub struct RequestBuilder<'a> {
    wrapped: reqwest::RequestBuilder,
    client: &'a Client,
}
```

**关键特性**:
- 包装 `reqwest::RequestBuilder`
- 确保异步调用在 Tokio 上下文外运行（通过 `compat`）
- 支持所有标准 HTTP 方法（GET, POST, PUT, DELETE, etc.）

### EventSourceStream

Server-Sent Events 流：

```rust
// WASM 版本（无 Send bound）
pub type EventSourceStream = futures::stream::LocalBoxStream<
    'static,
    Result<reqwest_eventsource::Event, reqwest_eventsource::Error>,
>;

// 原生版本（有 Send bound）
pub type EventSourceStream = futures::stream::BoxStream<
    'static,
    Result<reqwest_eventsource::Event, reqwest_eventsource::Error>,
>;
```

### 钩子类型

```rust
// 请求钩子
pub type RequestHookFn = Box<dyn Fn(&reqwest::Request, &Option<String>) + 'static + Send + Sync>;

// 响应钩子
pub type ResponseHookFn = Box<dyn Fn(&reqwest::Response) + 'static + Send + Sync>;
```

## 测试与质量

### 测试覆盖

- ⚠️ 单元测试有限（主要依赖集成测试）
- ✅ 通过 `crates/integration` 进行端到端测试
- ⚠️ 需要 HTTP 服务器模拟进行完整测试

### 运行测试

```bash
# 运行 http_client 测试
cargo test -p http_client

# 使用 nextest 运行
cargo nextest run -p http_client
```

## 常见问题 (FAQ)

### Q: 为什么需要 async-compat？

Warp 的某些部分在 Tokio 运行时外执行异步代码。`async-compat` 提供了 `Compat` 包装器，使 Tokio 的异步代码可以在其他运行时（或无运行时）中运行。

### Q: WASM 版本有什么不同？

WASM 版本：
- 使用 `LocalBoxStream`（无 `Send` bound）
- 使用 `gloo` 和 `wasm-bindgen-futures`
- 浏览器是单线程的，所以 `Send` 不必要

原生版本：
- 使用 `BoxStream`（有 `Send` bound）
- 使用 Tokio 运行时
- 支持多线程

### Q: 如何添加认证？

使用标准 HTTP 头部或 OAuth2：

```rust
// Bearer Token
client.get(url).header(AUTHORIZATION, "Bearer token").await?;

// OAuth2（需要 oauth2 crate）
use oauth2::{
    AuthorizationCode,
    CsrfToken,
    PkceCodeChallenge,
    // ...
};
```

### Q: 如何处理错误？

所有请求返回 `Result`，错误类型包括：
- `reqwest::Error` - 网络、解析、HTTP 错误
- `reqwest_eventsource::Error` - EventSource 特定错误

建议使用 `?` 传播错误或 `map_err` 转换错误。

### Q: 如何自定义超时？

在创建 `Client` 时配置：

```rust
use std::time::Duration;
use reqwest::Client;

let reqwest_client = Client::builder()
    .timeout(Duration::from_secs(30))
    .build()?;

// 然后用 http_client::Client::wrap(reqwest_client) 包装
```

## 相关文件清单

### 核心模块
- `src/lib.rs` - 主入口，客户端和请求构建器实现

### 配置
- `Cargo.toml` - 依赖和特性标志配置

## 变更记录 (Changelog)

### 2026-05-01 (22:54)

- ✅ 创建 http_client 模块文档
- ✅ 记录 HTTP 客户端 API
- ✅ 记录 EventSource 支持
- ✅ 记录自定义头部和环境变量
- ✅ 记录平台特定差异（WASM vs 原生）
- ✅ 添加导航面包屑
- ✅ 记录测试策略和常见问题

---

*本文档由 AI 自动生成和维护。如有问题或建议，请在 issue 中提出。*
