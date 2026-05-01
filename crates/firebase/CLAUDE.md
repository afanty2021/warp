# Firebase 模块

[根目录](../../CLRUDE.md) > [crates](../) > **firebase**

## 模块职责

提供 Firebase Authentication API 的类型定义和数据结构，用于用户认证、令牌管理和账户信息查询。支持 Google Identity Platform。

## 入口与启动

- **主要入口**：`crates/firebase/src/lib.rs`
- **核心类型**：`FirebaseError`、`AccountInfo`、`GetAccountInfoResponse`

## 对外接口

### 错误类型

```rust
pub struct FirebaseError {
    pub code: i32,
    pub message: String,
}
```

### 账户信息

```rust
pub struct AccountInfo {
    pub local_id: String,
    pub photo_url: Option<String>,
    pub screen_name: Option<String>,
    pub display_name: Option<String>,
    pub email: Option<String>,
}

impl AccountInfo {
    pub fn from_profile(
        firebase_uid: String,
        photo_url: Option<String>,
        display_name: Option<String>,
        email: Option<String>,
    ) -> Self;

    pub fn display_name(&self) -> Option<&str>;
    pub fn email(&self) -> Result<&str>;
    pub fn has_sso_link(&self) -> bool;
}
```

### API 响应

```rust
pub enum GetAccountInfoResponse {
    Success(GetAccountInfoResponsePayload),
    Error { error: FirebaseError },
}

pub enum FetchAccessTokenResponse {
    Success {
        expires_in: String,
        id_token: String,
        refresh_token: String,
    },
    Error {
        error: FirebaseError,
    },
}
```

### 功能特性

- **账户查询**：`GET /v1/accounts/lookup` 响应解析
- **令牌交换**：刷新令牌和自定义令牌转换为访问令牌
- **SSO 支持**：检测 WorkOS OIDC 链接
- **错误处理**：标准化的 Firebase 错误格式
- **序列化支持**：serde 序列化/反序列化

## 关键依赖与配置

### 依赖项

- `serde`：序列化/反序列化
- `anyhow`：错误处理

### API 端点

- **账户查询**：`https://identitytoolkit.googleapis.com/v1/accounts:lookup`
- **令牌交换**：Firebase Auth REST API
- **参考文档**：
  - https://cloud.google.com/identity-platform/docs/use-rest-api
  - https://firebase.google.com/docs/reference/rest/auth

## 数据模型

### 错误格式

遵循 Google API v1 错误格式：
- `code`：HTTP 状态码
- `message`：错误描述

### 用户信息

- `local_id`：Firebase UID
- `photo_url`、`display_name`、`email`：用户资料
- `provider_user_info`：第三方提供商信息

### 令牌响应

支持 camelCase 和 snake_case（不同端点使用不同约定）。

## 测试与质量

### 测试覆盖

- ⚠️ 无公开单元测试
- 依赖集成测试和 API 测试

### 代码质量

- 标准化的错误格式
- 清晰的类型定义
- Serde 驱动的序列化

## 常见问题 (FAQ)

### Q: 如何处理缺失的邮箱？

A: `email()` 方法返回 `Result`，调用方处理错误。

### Q: 如何检测 SSO 登录？

A: 使用 `has_sso_link()` 检查是否有 WorkOS OIDC 提供商。

### Q: 为什么同时支持 camelCase 和 snake_case？

A: 不同 Firebase 端点使用不同约定。

### Q: 如何从用户资料创建 AccountInfo？

A: 使用 `AccountInfo::from_profile()` 静态方法。

## 相关文件清单

- `src/lib.rs`：主要实现和类型定义

## 变更记录

### 2026-05-01 (23:18)

- ✅ 初始化模块文档
- ✅ 添加 Firebase API 类型说明
- ✅ 添加错误处理和响应格式文档
- ✅ 添加 SSO 支持说明
