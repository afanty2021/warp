# app-installation-detection - 应用安装检测服务

[根目录](../../CLAUDE.md) > [crates](../) > **app-installation-detection**

> 最后更新：2026年 5月 1 日

## 模块职责

提供 HTTP 服务用于检测 Warp 应用程序的安装状态。这是一个轻量级的检测服务，允许特定来源验证 Warp 是否已正确安装。

## 入口与启动

### 主要入口

- **文件**：`src/lib.rs`
- **函数**：`make_router()`
- **类型**：Axum Router 构建器

### HTTP 端点

| 端点 | 方法 | 描述 | 响应 |
|------|------|------|------|
| `/install_detection` | GET | 检测 Warp 安装状态 | `"ok"` |

## 对外接口

### 核心函数

```rust
pub fn make_router() -> Router
```

创建配置了 CORS 和追踪的 Axum 路由器。

**特性**：
- CORS 配置允许来自 `localhost:8080/8082` 和 `*.warp.dev` 的请求
- 集成 HTTP 追踪层
- 仅支持 GET 方法

### 检测端点

```rust
async fn detect_installation() -> &'static str
```

返回 `"ok"` 表示 Warp 已安装并可访问。

## 关键依赖与配置

### Cargo.toml 依赖

```toml
[dependencies]
anyhow = "1.0"
axum = "0.8.4"
axum-extra = "0.10.1"
command = { path = "../command" }
tokio = { version = "1.37", features = ["full"] }
nix = "0.26.4"
tower = "0.5.2"
tower-http = { version = "0.6.6", features = ["trace", "cors"] }
tracing = "0.1.40"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }
warp_cli = { path = "../warp_cli" }
```

### 平台特定依赖

**Windows**：
- `win32job = "2.0.1"` - Windows 作业对象管理

## 数据模型

### 无持久化数据

此模块不维护任何状态或数据模型，仅提供简单的 HTTP 响应。

## 测试与质量

### 测试覆盖

- **单元测试**：❌ 无
- **集成测试**：❌ 无
- **文档测试**：❌ 无

### 代码质量

- 使用 `tracing` 进行请求追踪
- 实现了 CORS 安全策略
- 使用 Tower 中间件栈

## 常见问题 (FAQ)

### Q: 为什么需要这个检测服务？

A: 允许外部服务（如 Warp 服务器）验证本地 Warp 安装是否正常运行，用于远程代理和云功能。

### Q: CORS 限制是什么？

A: 仅允许来自以下来源的请求：
- `http://localhost:8080`
- `http://localhost:8082`
- `https://warp.dev`
- `*.warp.dev`（任何子域名）

### Q: 为什么响应如此简单？

A: 此端点仅用于检测服务可用性，不需要返回详细信息。HTTP 200 状态码和 `"ok"` 响应体足以表示服务正常。

## 相关文件清单

```
crates/app-installation-detection/
├── Cargo.toml              # 包配置
├── src/
│   └── lib.rs              # 主入口（55 行）
└── CLAUDE.md               # 本文档
```

## 变更记录

### 2026-05-01

- ✅ 创建初始模块文档
- ✅ 记录 HTTP 接口和 CORS 配置
- ✅ 添加依赖和平台特定信息
- ✅ 添加导航面包屑

---

*此模块文档是 Warp AI 上下文的一部分*
