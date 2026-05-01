[根目录](../../CLAUDE.md) > [crates](../) > **graphql**

# graphql - GraphQL 客户端

> 最后更新：2026年 5月 1日 星期五 22時40分12秒 CST

## 模块职责

`graphql` crate 提供 Warp 服务器的 GraphQL 客户端实现。它处理所有与 Warp 服务器的 GraphQL 通信，包括查询、变更和订阅。

## 入口与启动

### 主要入口

- **`src/lib.rs`** - 库入口
  - 导出所有 API 模块
  - 导出客户端实现

- **`src/client.rs`** - GraphQL 客户端核心
  - 定义 `Operation` trait
  - 实现 GraphQL 请求发送
  - 处理响应和错误

## 对外接口

### 核心模块

1. **`api/`** - GraphQL API 定义
   - **`mod.rs`** - API 模块入口
   - **`mutations/`** - GraphQL 变更（mutations）
     - 40+ 变更操作（创建、更新、删除等）
   - **`queries/`** - GraphQL 查询
     - 各种查询操作
   - **类型定义** - 各种数据类型
     - `ai.rs` - AI 相关类型
     - `billing.rs` - 计费类型
     - `notebook.rs` - 笔记本类型
     - `object.rs` - 对象类型
     - 等等

2. **`client`** - GraphQL 客户端
   - **`Operation` trait** - GraphQL 操作接口
   - **`GraphQLError`** - 错误类型
   - **`RequestOptions`** - 请求选项
   - **`build_graphql_request`** - 请求构建函数

3. **`managed_secrets`** - 托管密钥管理
   - 密钥的 GraphQL 操作

4. **`scalars`** - 自定义 GraphQL 标量类型
   - 日期、时间等自定义类型

### 核心 Trait

```rust
pub trait Operation<QF> {
    /// 操作名称
    fn operation_name(&self) -> Option<Cow<'_, str>>;

    /// 发送 GraphQL 请求
    fn send_request(
        self,
        client: std::sync::Arc<http_client::Client>,
        options: RequestOptions,
    ) -> BoxFuture<'static, Result<GraphQlResponse<QF>, GraphQLError>>
    where
        Self: Sized;
}
```

### 错误类型

```rust
pub enum GraphQLError {
    RequestError(reqwest::Error),           // 请求发送错误
    StagingAccessBlocked,                   // 未授权访问测试服务器
    HttpError { status: StatusCode, body: String },  // HTTP 错误
    ResponseError(reqwest::Error),          // 响应反序列化错误
}
```

## 关键依赖与配置

### 依赖项

**核心依赖**:
- **`cynic`** (3) - GraphQL 客户端库
  - 查询构建和类型安全
- **`http_client`** - HTTP 客户端（内部 crate）
- **`warp_core`** - 核心功能
- **`warp_graphql_schema`** - GraphQL schema 定义

**异步运行时**:
- **`futures`** - 异步工具
- **`futures-util`** - Future 工具
- **`async-channel`** - 异步通道

**序列化**:
- **`serde`** - 序列化/反序列化
- **`chrono`** - 日期时间处理

**错误处理**:
- **`anyhow`** - 错误处理
- **`thiserror`** - 错误派生宏

**网络**:
- **`reqwest`** - HTTP 客户端
- **`http`** - HTTP 类型

**非 WASM 特定依赖** (`cfg(not(target_family = "wasm"))`):
- **`graphql-ws-client`** - WebSocket GraphQL 客户端
  - features: `client-cynic`, `tungstenite`

**WASM 特定依赖** (`cfg(target_family = "wasm")`):
- **`graphql-ws-client`** - WebSocket GraphQL 客户端
  - features: `client-cynic`, `ws_stream_wasm`
- **`ws_stream_wasm`** (0.7) - WASM WebSocket 流

**构建依赖**:
- **`cynic-codegen`** - GraphQL 代码生成
- **`anyhow`** - 构建时错误处理

### 特性标志

- **`test-util`** - 测试工具支持
  - 启用测试相关的实用功能

## 数据模型

### 请求选项

```rust
pub struct RequestOptions {
    pub auth_token: Option<String>,                    // Bearer token
    pub timeout: Option<Duration>,                     // 请求超时
    pub headers: std::collections::HashMap<String, String>,  // 额外 HTTP 头
    pub path_prefix: Option<String>,                   // URL 路径前缀
}
```

### GraphQL 操作

```rust
// 查询示例
#[derive(cynic::QueryFragment, Debug)]
pub struct GetCloudObject {
    // 查询字段
}

// 变更示例
#[derive(cynic::QueryFragment, Debug)]
pub struct CreateNotebook {
    // 变更字段
}
```

## API 概览

### 主要变更（Mutations）

- **对象管理**
  - `create_folder` - 创建文件夹
  - `create_notebook` - 创建笔记本
  - `delete_object` - 删除对象
  - `move_object` - 移动对象
  - `rename_object` - 重命名对象
  - `trash_object` / `untrash_object` - 回收站操作

- **团队管理**
  - `create_team` - 创建团队
  - `transfer_team_ownership` - 转让团队所有权
  - `set_team_member_role` - 设置成员角色
  - `remove_user_from_team` - 移除成员

- **AI 相关**
  - `generate_code_embeddings` - 生成代码嵌入
  - `generate_commands` - 生成命令
  - `generate_dialogue` - 生成对话
  - `create_agent_task` - 创建代理任务
  - `update_agent_task` - 更新代理任务

- **密钥管理**
  - `generate_api_key` - 生成 API 密钥
  - `expire_api_key` - 过期 API 密钥
  - `create_managed_secret` - 创建托管密钥
  - `delete_managed_secret` - 删除托管密钥

### 主要查询（Queries）

- **对象查询**
  - `get_cloud_object` - 获取云对象
  - `get_blocks_for_user` - 获取用户块

- **用户信息**
  - `get_referral_info` - 获取推荐信息
  - `get_conversation_usage` - 获取对话使用情况

- **集成**
  - `get_integrations_using_environment` - 获取环境集成
  - `get_cloud_environments` - 获取云环境

## 测试与质量

### 测试策略

- 使用 `test-util` 特性标志启用测试工具
- `http_client` 的 `test-util` 特性用于模拟 HTTP 响应
- 集成测试覆盖 GraphQL 操作

### 代码质量

- 遵循 Rust 2021 edition 标准
- 使用 `cynic` 确保类型安全的 GraphQL 查询
- 从 schema 自动生成类型（`cynic-codegen`）
- 完善的错误处理

## 常见问题 (FAQ)

### Q1: 如何创建新的 GraphQL 查询？

1. 在 `src/api/queries/` 中定义查询结构
2. 使用 `cynic::QueryFragment` derive 宏
3. 实现 `Operation` trait
4. 通过客户端发送请求

```rust
use cynic::QueryFragment;

#[derive(QueryFragment, Debug)]
pub struct MyQuery {
    // 字段定义
}

impl Operation<MyQueryFragment> for MyQuery {
    fn operation_name(&self) -> Option<Cow<'_, str>> {
        Some("MyQuery".into())
    }
}
```

### Q2: 如何处理认证？

使用 `RequestOptions`:

```rust
let options = RequestOptions {
    auth_token: Some("your-token".to_string()),
    ..Default::default()
};

let response = operation.send_request(client, options).await?;
```

### Q3: 如何设置请求超时？

```rust
let options = RequestOptions {
    timeout: Some(Duration::from_secs(30)),
    ..Default::default()
};
```

### Q4: WebSocket 支持如何工作？

- 非目标：使用 `graphql-ws-client` + `tungstenite`
- WASM：使用 `graphql-ws-client` + `ws_stream_wasm`
- 通过 `cfg(target_family)` 自动选择

## 相关文件清单

### 核心文件

- `Cargo.toml` - 包配置
- `src/lib.rs` - 库入口
- `src/client.rs` - 客户端实现
- `src/scalars.rs` - 自定义标量类型

### API 定义

- `src/api/mod.rs` - API 模块入口
- `src/api/ai.rs` - AI 类型
- `src/api/billing.rs` - 计费类型
- `src/api/notebook.rs` - 笔记本类型
- `src/api/object.rs` - 对象类型
- `src/api/error.rs` - 错误类型

### 变更操作

- `src/api/mutations/mod.rs` - 变更入口
- `src/api/mutations/create_notebook.rs`
- `src/api/mutations/delete_object.rs`
- `src/api/mutations/update_object_guests.rs`
- 等等（40+ 变更文件）

### 查询操作

- `src/api/queries/mod.rs` - 查询入口
- `src/api/queries/get_cloud_object.rs`
- `src/api/queries/get_referral_info.rs`
- 等等

### 其他模块

- `src/managed_secrets/` - 托管密钥

## 架构说明

### 设计模式

1. **类型安全** - 使用 `cynic` 确保编译时类型检查
2. **自动生成** - 从 schema 自动生成类型
3. **异步优先** - 基于 Tokio 的异步操作
4. **平台抽象** - 统一的 API，自动处理 WASM/原生差异

### 代码生成

使用 `cynic-codegen` 从 `warp_graphql_schema` 生成类型：
- 构建时运行
- 确保类型与 schema 同步
- 避免手动维护类型定义

## 变更记录

### 2026-05-01 (22:40)

- ✅ 初始化模块文档
- ✅ 记录核心 API 和接口
- ✅ 添加使用示例
- ✅ 记录依赖和配置
- ✅ 添加常见问题解答

---

*本文档由 AI 自动生成和维护。如有问题或建议，请在 issue 中提出。*
