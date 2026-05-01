[根目录](../../CLAUDE.md) > [crates](../) > **http_server**

# http_server - HTTP 服务器

## 模块职责

`http_server` 为 Warp 客户端提供轻量级本地 HTTP 服务器，用于处理本地 HTTP 请求（如回调、本地 API 端点等）。该模块使用 Axum 框架构建，在独立线程上运行，避免与主应用程序冲突。

## 入口与启动

- **主入口文件**: `src/lib.rs`
- **核心类型**:
  - `HttpServer` - HTTP 服务器单例实体
- **端口**: 9277（拼写 "WARP"，不与已知端口冲突）

## 对外接口

### HTTP 服务器 (HttpServer)

```rust
use http_server::HttpServer;
use warpui::{ModelContext, SingletonEntity};
use axum::{Router, routing::get};

// 创建 HTTP 服务器（作为单例）
let _http_server = cx.new_entity(|cx| {
    let router = Router::new()
        .route("/health", get(|| async { "OK" }))
        .route("/api/callback", get(callback_handler));

    HttpServer::new(vec![router], cx)
});
```

**关键特性**:
- 单例模式（整个应用程序只有一个实例）
- 在专用 Tokio 运行时上运行（单线程）
- 自动处理服务器启动失败（记录警告，不崩溃）
- 支持多个路由器的合并

### 路由定义

```rust
use axum::{Router, routing::{get, post}, Json};

// 简单路由
let router = Router::new()
    .route("/hello", get(|| async { "Hello, World!" }));

// 带处理器的路由
let router = Router::new()
    .route("/api/data", get(get_data_handler).post(create_data_handler));

async fn get_data_handler() -> Json<&'static str> {
    Json("data")
}

async fn create_data_handler(Json(payload): Json<Data>) -> Json<Response> {
    // 处理数据
    Json(Response { success: true })
}
```

### 服务器配置

**当前配置**:
- **地址**: `127.0.0.1:9277`
- **运行时**: 独立的 Tokio 多线程运行时（1 个工作线程）
- **中间件**: `TraceLayer`（HTTP 请求日志）

**端口选择**:
- 拼写 "WARP"（9 = W, 2 = A, 7 = R, 7 = P）
- 不与已知端口冲突（参考 [List of TCP and UDP port numbers](https://en.wikipedia.org/wiki/List_of_TCP_and_UDP_port_numbers)）

## 关键依赖与配置

### 依赖关系

```toml
[dependencies]
axum = { workspace = true }           # Web 框架
tokio = { workspace = true }          # 异步运行时
tower = { workspace = true }          # 中间件
tower-http = { workspace = true }     # HTTP 特定中间件
warpui = { workspace = true }         # UI 框架（Entity, ModelContext）
log = { workspace = true }            # 日志
```

### 中间件

```toml
tower-http = { workspace = true, features = ["trace", "cors"] }
```

- **Trace** - 记录所有 HTTP 请求
- **CORS** - 跨源资源共享（可用于本地开发）

## 数据模型

### HttpServer

HTTP 服务器实体：

```rust
pub struct HttpServer {
    _runtime: Option<tokio::runtime::Runtime>,
}
```

**设计原则**:
- 持有运行时以保持服务器活跃
- 使用 `Option` 处理启动失败（服务器变为空操作）
- 单例实体（通过 `SingletonEntity` trait）

### Entity 实现

```rust
impl Entity for HttpServer {
    type Event = ();  // 无事件
}

impl SingletonEntity for HttpServer {}
```

**无事件设计**: 服务器启动后不生成事件，错误仅通过日志记录。

## 测试与质量

### 测试覆盖

- ⚠️ 无内置单元测试
- ✅ 通过集成测试验证服务器功能
- ⚠️ 需要手动测试验证端点

### 运行测试

```bash
# 运行 http_server 测试
cargo test -p http_server

# 使用 nextest 运行
cargo nextest run -p http_server
```

### 手动测试

```bash
# 启动 Warp
cargo run

# 在另一个终端测试端点
curl http://127.0.0.1:9277/health
curl http://127.0.0.1:9277/api/callback
```

## 常见问题 (FAQ)

### Q: 为什么使用独立的 Tokio 运行时？

Warp 目前没有共享的 Tokio 运行时。使用独立运行时可以：
- 避免与主应用程序的异步上下文冲突
- 确保服务器始终运行（即使主线程阻塞）
- 提供隔离性（服务器崩溃不影响主应用）

**未来计划**: 当有共享 Tokio 运行时时，移除独立运行时。

### Q: 如何添加新路由？

1. 定义路由处理器函数
2. 创建 `Router` 并添加路由
3. 在创建 `HttpServer` 时传入路由器

```rust
let router = Router::new()
    .route("/my-endpoint", get(my_handler));

HttpServer::new(vec![router], cx);
```

### Q: 如何处理 CORS？

使用 `tower-http` 的 CORS 中间件：

```rust
use tower_http::cors::{CorsLayer, Any};
use axum::Router;

let router = Router::new()
    .route("/api/data", get(handler))
    .layer(CorsLayer::new().allow_origin(Any));
```

### Q: 服务器启动失败会怎样？

服务器启动失败会：
1. 记录警告日志：`log::warn!("Failed to start local HTTP server: {err:#}");`
2. `_runtime` 字段设为 `None`
3. 服务器变为空操作（不崩溃）

这确保即使端口冲突或其他问题，应用程序也能继续运行。

### Q: 如何修改端口？

编辑 `src/lib.rs` 中的 `PORT` 常量：

```rust
// spells "Warp"
const PORT: u16 = 9277;  // 修改为其他端口
```

**注意**: 选择端口时确保不与其他服务冲突。

### Q: 如何添加 TLS/HTTPS？

当前实现仅支持 HTTP。添加 HTTPS 需要：
1. 使用 `axum_server::ServerTls` 配置 TLS
2. 提供证书和私钥
3. 修改 `spawn_server` 函数

由于这是本地服务器（127.0.0.1），通常不需要 HTTPS。

## 相关文件清单

### 核心模块
- `src/lib.rs` - 主入口，HTTP 服务器实现

### 配置
- `Cargo.toml` - 依赖和特性标志配置

## 变更记录 (Changelog)

### 2026-05-01 (22:54)

- ✅ 创建 http_server 模块文档
- ✅ 记录 HTTP 服务器 API
- ✅ 记录路由定义方法
- ✅ 记录服务器配置和端口选择
- ✅ 记录独立运行时的设计决策
- ✅ 添加导航面包屑
- ✅ 记录测试策略和常见问题

---

*本文档由 AI 自动生成和维护。如有问题或建议，请在 issue 中提出。*
