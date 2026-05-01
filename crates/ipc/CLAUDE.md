# IPC - 进程间通信模块

[根目录](../../CLAUDE.md) > [crates](../) > **ipc**

> 最后更新：2026-05-01

## 模块职责

IPC（Inter-Process Communication）模块为 Warp 提供了一个基本的 IPC 请求/响应协议实现。该模块设计用于支持 Warp 应用程序与第三方插件（运行在独立的"插件宿主"进程中）之间的通信，但也设计为可扩展到其他用例（如终端服务器）。

**核心功能**：
- 类型安全的请求/响应协议
- 服务端和客户端的抽象
- 跨平台传输层抽象（Unix Domain Sockets / Named Pipes）
- 支持多个独立服务的注册和调用

## 入口与启动

### 主要类型

**服务端**：
- `Server` - IPC 服务器，管理多个服务的注册和请求路由
- `ServerBuilder` - 构建器模式，用于配置和启动服务器
- `ServiceImpl` - 服务实现 trait，由具体服务实现

**客户端**：
- `Client` - IPC 客户端，连接到服务器并发送请求
- `ServiceCaller` - 类型化的服务调用接口

### 使用示例

```rust
// 服务端
let my_service_impl = MyServiceImpl::new();
let (server, connection_address) = ServerBuilder::default()
    .with_service(my_service_impl)
    .build_and_run(background_executor)
    .expect("Failed to instantiate server");

// 客户端
let client = Arc::new(
    Client::connect(connection_address, background_executor)
        .await
        .expect("Failed to connect client"),
);
let my_service_stub = service_caller::<MyService>(client);
let response = my_service_stub.call(MyServiceRequest { .. }).await;
```

## 对外接口

### 核心 Trait

**`Service` trait**：
```rust
#[async_trait]
pub trait Service: Send + Sync + 'static {
    type Request: Message + 'static;
    type Response: Message + 'static;
}
```

**`ServiceImpl` trait**：
```rust
#[async_trait]
pub trait ServiceImpl: 'static + Send + Sync + Clone {
    type Service: Service;

    async fn handle_request(
        &self,
        request: <<Self as ServiceImpl>::Service as Service>::Request,
    ) -> <<Self as ServiceImpl>::Service as Service>::Response;
}
```

**`ServiceCaller` trait**：
```rust
#[async_trait]
pub trait ServiceCaller<S: Service>: Send + Sync {
    async fn call(&self, request: S::Request) -> Result<S::Response, ClientError>;
}
```

### 关键函数

- `service_caller<S: Service>(client: Arc<Client>) -> Box<dyn ServiceCaller<S>>` - 创建类型化的服务调用器

## 关键依赖与配置

### 依赖项

**核心依赖**：
- `async-trait` - 异步 trait 支持
- `bincode` - 序列化/反序列化
- `futures` - 异步工具
- `serde` - 序列化框架
- `uuid` - 唯一标识符生成

**平台特定依赖**：
- `interprocess` (native) - Unix Domain Sockets 和 Named Pipes
- WASM 支持计划中（使用 WebWorkers MessagePort API）

### 配置

该模块主要通过代码配置，无需外部配置文件。

## 数据模型

### 协议消息

**`ConnectionAddress`** - 连接地址类型（平台特定）

**`Request`** - 请求消息
- `id: RequestId` - 请求唯一标识符
- `service_id: ServiceId` - 目标服务标识符
- `bytes: Vec<u8>` - 序列化的请求体

**`Response`** - 响应消息
- `Success { request_id, bytes }` - 成功响应
- `Failure { request_id, error_message }` - 失败响应

### 错误类型

**`ClientError`**：
- `Initialization(InitializationError)` - 初始化失败
- `Disconnected` - 连接断开
- `InternalProtocol(ProtocolError)` - 协议错误
- `ResponseChannelClosed` - 响应通道关闭
- `PendingRequestInfoChannelClosed` - 请求信息通道关闭

## 测试与质量

### 测试覆盖

⚠️ **单元测试**：当前没有发现单元测试（`#[test]` 计数：0）

✅ **集成测试**：模块设计支持集成测试，通过 `testing` 模块提供测试工具

### 测试策略

**推荐测试方法**：
1. 使用 `testing` 模块提供的工具创建模拟服务器和客户端
2. 测试服务注册和调用流程
3. 测试错误处理场景（断开连接、超时等）
4. 测试并发请求处理

### 测试工具

模块提供了 `pub mod testing`，可能包含测试辅助工具。

## 常见问题 (FAQ)

### Q: IPC 模块支持哪些平台？

**A**:
- ✅ **Native 平台**：完全支持（macOS、Linux、Windows）
  - Unix: Unix Domain Sockets
  - Windows: Named Pipes
- ⚠️ **WASM 平台**：计划支持（使用 WebWorkers MessagePort API）

### Q: 如何添加新的 IPC 服务？

**A**:
1. 定义请求和响应类型（必须实现 `Message` trait）
2. 定义 `Service` trait 的实现，指定请求和响应类型
3. 实现 `ServiceImpl` trait，提供请求处理逻辑
4. 在服务器端使用 `ServerBuilder::with_service()` 注册服务
5. 在客户端使用 `service_caller::<YourService>()` 创建调用器

### Q: IPC 协议是同步的还是异步的？

**A**: 完全异步。所有操作都是异步的，使用 `async/await` 模式。请求和响应通过异步通道传递。

### Q: 如何处理连接断开？

**A**:
- 客户端可以通过 `Client::wait_for_disconnect()` 等待断开事件
- 所有请求操作返回 `Result<ClientError>`，可以捕获 `Disconnected` 错误
- 建议在调用处实现重连逻辑

### Q: IPC 传输层是否安全？

**A**: IPC 模块本身不提供加密或认证。安全性依赖于底层传输：
- Unix Domain Sockets：通过文件系统权限提供隔离
- Named Pipes：通过 Windows 安全描述符提供访问控制
- 建议在应用层实现额外的认证机制

## 相关文件清单

### 核心文件

- `src/lib.rs` - 模块入口，导出公共 API
- `src/service.rs` - 服务 trait 和调用器实现
- `src/client.rs` - 客户端实现
- `src/server.rs` - 服务器实现
- `src/protocol.rs` - 协议消息定义

### 平台特定实现

- `src/native.rs` - 原生平台实现（Unix/Windows）
- `src/wasm.rs` - WASM 平台实现（计划中）

### 测试文件

- `src/testing.rs` - 测试工具模块

## 变更记录

### 2026-05-01

- ✅ 创建模块文档
- ✅ 记录核心接口和使用模式
- ✅ 文档化平台支持情况
- ✅ 添加测试策略建议

---

*本文档由 AI 自动生成和维护。如有问题或建议，请在 issue 中提出。*
