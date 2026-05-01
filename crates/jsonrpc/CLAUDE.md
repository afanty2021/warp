# JSON-RPC 模块

[根目录](../../CLAUDE.md) > [crates](../) > **jsonrpc**

## 模块职责

提供 JSON-RPC 2.0 协议的服务端实现，用于处理客户端的 JSON-RPC 请求和通知。该模块支持异步请求处理、通知事件和自定义传输层。

## 入口与启动

- **主要入口**：`crates/jsonrpc/src/lib.rs`
- **核心服务**：`JsonRpcService`
- **传输层抽象**：`Transport`

## 对外接口

### 核心类型

```rust
pub struct JsonRpcService;
pub enum RequestId;
pub enum ServerNotificationEvent;
pub trait Transport;
```

### 功能特性

- **请求处理**：支持批量和单个 JSON-RPC 请求
- **通知处理**：服务器主动推送通知给客户端
- **传输层抽象**：支持自定义传输协议（WebSocket、IPC 等）
- **异步支持**：基于 Tokio 的异步请求处理

## 关键依赖与配置

### 依赖项

- `warpui`：UI 框架集成
- `serde`：JSON 序列化/反序列化
- `tokio`：异步运行时

### 配置要求

- 需要 `serde_json` 支持 JSON 格式
- 传输层需要实现 `Transport` trait

## 数据模型

### 请求模型

- **RequestId**：请求标识符（数字或字符串）
- **ServerNotificationEvent**：服务器通知事件

### 响应模型

遵循 JSON-RPC 2.0 规范：
- 成功响应：包含 `result` 字段
- 错误响应：包含 `error` 字段

## 测试与质量

### 测试覆盖

- ✅ 单元测试：`crates/jsonrpc/src/service_tests.rs`
- 测试内容：
  - 请求解析和验证
  - 响应格式验证
  - 错误处理
  - 批量请求处理

### 代码质量

- 遵循 Rust 编码规范
- 使用 `serde` 进行类型安全的序列化
- 异步错误处理

## 常见问题 (FAQ)

### Q: 如何实现自定义传输层？

A: 实现 `Transport` trait，提供发送和接收消息的能力。

### Q: 支持批量请求吗？

A: 是的，JSON-RPC 2.0 规范支持批量请求。

### Q: 如何处理错误？

A: 使用 JSON-RPC 标准错误格式，包含错误码和消息。

## 相关文件清单

- `src/lib.rs`：模块导出
- `src/service.rs`：JSON-RPC 服务实现
- `src/service_tests.rs`：单元测试
- `src/transport.rs`：传输层抽象

## 变更记录

### 2026-05-01 (23:18)

- ✅ 初始化模块文档
- ✅ 添加架构说明和接口文档
- ✅ 添加测试覆盖说明
