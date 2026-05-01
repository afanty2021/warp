# Managed Secrets 模块

[根目录](../../CLRUDE.md) > [crates](../) > **managed_secrets**

## 模块职责

提供机密信息管理功能，支持 GCP Workload Identity Federation、任务身份令牌和加密信封。用于安全地管理和访问云服务凭据。

## 入口与启动

- **主要入口**：`crates/managed_secrets/src/lib.rs`
- **核心管理器**：`ManagedSecretManager`
- **客户端**：`TaskIdentityToken`

## 对外接口

### 核心类型

```rust
pub struct ManagedSecretManager;
pub struct TaskIdentityToken;

pub struct GcpCredentials;
pub struct GcpWorkloadIdentityFederationToken;
pub struct UploadKey;

pub enum ManagedSecretValue;
```

### 主要组件

- **client**：令牌客户端
- **envelope**：加密信封
- **gcp**：GCP 凭据和联邦认证
- **manager**：机密管理器
- **secret_value**：机密值类型

### 功能特性

- **GCP 联邦认证**：Workload Identity Federation 支持
- **令牌管理**：自动刷新和管理访问令牌
- **加密信封**：信封加密模式
- **安全存储**：安全存储和访问敏感信息
- **跨平台**：支持不同平台的密钥管理

## 关键依赖与配置

### 依赖项

- 详见各子模块

### 配置要求

- GCP 凭据配置
- 联邦认证设置
- 加密密钥管理

## 数据模型

### GCP 相关类型

```rust
pub struct GcpCredentials;
pub struct GcpFederationConfig;
pub struct GcpWorkloadIdentityFederationToken;

pub enum PrepareGcpCredentialsError;
pub enum GcpWorkloadIdentityFederationError;
```

### 机密值

```rust
pub enum ManagedSecretValue {
    // 不同类型的机密值
}
```

### 错误类型

- `PrepareGcpCredentialsError`：GCP 凭据准备错误
- `GcpWorkloadIdentityFederationError`：联邦认证错误

## 测试与质量

### 测试覆盖

- ✅ 单元测试：`src/envelope_tests.rs`
- 测试内容：
  - 加密信封操作
  - 令牌生成和验证
  - 错误处理

### 代码质量

- 类型安全的错误处理
- 清晰的模块分离
- 安全的密钥管理

## 常见问题 (FAQ)

### Q: 如何配置 GCP 联邦认证？

A: 使用 `GcpFederationConfig` 配置认证参数。

### Q: 令牌如何刷新？

A: `ManagedSecretManager` 自动处理令牌刷新。

### Q: 什么是信封加密？

A: 使用数据加密密钥（DEK）和密钥加密密钥（KEK）的加密模式。

### Q: 如何获取任务身份令牌？

A: 使用 `TaskIdentityToken` 客户端。

### Q: 支持哪些云平台？

A: 目前主要支持 GCP，可扩展到其他平台。

## 相关文件清单

- `src/lib.rs`：主要导出
- `src/client.rs`：令牌客户端
- `src/envelope.rs`：加密信封
- `src/gcp.rs`：GCP 集成
- `src/manager.rs`：机密管理器
- `src/secret_value.rs`：机密值类型
- `src/envelope_tests.rs`：单元测试

## 变更记录

### 2026-05-01 (23:18)

- ✅ 初始化模块文档
- ✅ 添加机密管理功能说明
- ✅ 添加 GCP 集成和认证文档
- ✅ 添加错误处理和安全说明
