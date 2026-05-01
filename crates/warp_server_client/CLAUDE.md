# Warp Server Client 模块

[根目录](../../CLRUDE.md) > [crates](../) > **warp_server_client**

## 模块职责

提供 Warp 服务器通信的客户端功能，包括认证、云对象、Drive 同步和用户 ID 管理。

## 入口与启动

- **主要入口**：`crates/warp_server_client/src/lib.rs`

## 对外接口

### 核心模块

```rust
pub mod auth;              // 认证功能
pub mod cloud_object;      // 云对象管理
pub mod drive;             // Drive 同步
pub mod ids;               // ID 类型定义
#[cfg(not(target_family = "wasm"))]
pub mod persistence;       // 持久化
```

### 主要类型

```rust
pub use auth::UserUid;
```

### 功能特性

- **认证管理**：用户认证和会话管理
- **云对象**：云端对象存储和同步
- **Drive 同步**：Warp Drive 功能
- **持久化**：本地缓存和状态持久化（非 WASM）

## 关键依赖与配置

### 依赖项

- 详见各子模块

### 配置要求

- 服务器端点配置
- 认证凭据管理

## 数据模型

### UserUid

用户唯一标识符，用于跨平台识别用户。

### 云对象

云端存储的对象，支持同步和共享。

## 测试与质量

### 测试覆盖

- ⚠️ 依赖集成测试

### 代码质量

- 模块化设计
- 平台特定代码隔离

## 常见问题 (FAQ)

### Q: 如何配置服务器端点？

A: 通过环境变量或配置文件设置。

### Q: 支持哪些认证方式？

A: 详见 `auth` 模块文档。

### Q: WASM 平台支持吗？

A: 基本支持，但持久化功能不可用。

## 相关文件清单

- `src/auth.rs`
- `src/cloud_object.rs`
- `src/drive.rs`
- `src/ids.rs`
- `src/persistence.rs`（非 WASM）

## 变更记录

### 2026-05-01 (23:18)

- ✅ 初始化模块文档
- ✅ 添加客户端功能说明
- ✅ 添加模块列表和平台支持
