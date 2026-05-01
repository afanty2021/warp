# isolation_platform - 隔离平台检测

[根目录](../../CLAUDE.md) > [crates](../) > **isolation_platform**

> 最后更新：2026年 5月 1 日

## 模块职责

检测和管理 Warp 在不同隔离平台中的运行环境。支持 Docker、Kubernetes、Namespace 和 Docker Sandbox 等环境，并提供工作负载身份令牌（Workload Identity Tokens）。

## 入口与启动

### 主要入口

- **文件**：`src/lib.rs`
- **核心函数**：
  - `detect()` - 检测当前隔离平台
  - `issue_workload_token()` - 颁发工作负载令牌

### 平台检测模块

| 平台 | 检测文件 | 支持令牌 |
|------|----------|----------|
| Docker | `src/docker.rs` | ❌ |
| Docker Sandbox | `src/docker_sandbox.rs` | ✅ |
| Kubernetes | `src/kubernetes.rs` | ❌ |
| Namespace | `src/namespace.rs` | ✅ |

## 对外接口

### 1. 平台检测

```rust
pub fn detect() -> Option<IsolationPlatformType>
```

检测当前运行的隔离平台类型。

**返回值**：
- `Some(IsolationPlatformType)` - 检测到的平台
- `None` - 未检测到隔离平台

**特性**：
- 结果在进程生命周期内缓存（`OnceLock`）
- 优先信任服务器明确指定的平台（环境变量）
- 按优先级检测：Namespace → Kubernetes → Docker

### 2. 工作负载令牌

```rust
pub async fn issue_workload_token(
    duration: Option<Duration>
) -> Result<WorkloadToken, IsolationPlatformError>
```

为当前隔离平台颁发工作负载身份令牌。

**参数**：
- `duration` - 令牌有效期（可选）

**返回值**：
- `Ok(WorkloadToken)` - 包含令牌字符串和过期时间
- `Err(IsolationPlatformError)` - 令牌颁发失败

### 3. 平台类型

```rust
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum IsolationPlatformType {
    Docker,           // 自托管 Docker 容器
    DockerSandbox,    // Warp 托管的 Docker Sandbox
    Kubernetes,       // Kubernetes Pod（自托管）
    Namespace,        // Namespace 实例（Warp 托管）
}
```

### 4. 工作负载令牌

```rust
pub struct WorkloadToken {
    pub token: String,                        // 令牌字符串
    pub expires_at: Option<DateTime<Utc>>,    // 过期时间（可选）
}
```

## 关键依赖与配置

### Cargo.toml 依赖

```toml
[dependencies]
anyhow = "1.0"
base64 = "0.22"
chrono = { version = "0.4.38", features = ["serde"] }
log = "0.4"
serde = "1.0"
serde_json = "1.0"
thiserror = "2.0.17"
command = { path = "../command" }
warp_core = { path = "../warp_core" }
```

### 环境变量

| 变量名 | 用途 | 格式 |
|--------|------|------|
| `WARP_ISOLATION_PLATFORM` | 服务器明确指定的平台 | `docker` / `docker_sandbox` / `kubernetes` / `namespace` |
| `WARP_WORKLOAD_TOKEN` | 平台无关的通用工作负载令牌 | 任意字符串 |

## 数据模型

### 平台检测策略

1. **环境变量优先**：如果 `WARP_ISOLATION_PLATFORM` 设置，信任其值
2. **Namespace 检测**：
   - 检查 `NSC_TOKEN_FILE` 环境变量
   - 检查 `/var/run/nsc/token.json` 文件存在性
3. **Kubernetes 检测**：
   - 检查 `/var/run/secrets/kubernetes.io/serviceaccount/token` 文件
4. **Docker 检测**：
   - 检查 `/.dockerenv` 标记文件

### 令牌颁发策略

| 平台 | 令牌来源 | 过期时间 |
|------|----------|----------|
| Docker Sandbox | 平台 API | 有过期时间 |
| Namespace | `nsc auth issue-id-token` 命令 | 可配置 |
| Kubernetes | 通用令牌环境变量 | 无过期 |
| Docker | 通用令牌环境变量 | 无过期 |

## 测试与质量

### 测试覆盖

- **单元测试**：
  - ✅ `namespace_tests.rs` - Namespace 平台测试
- **集成测试**：❌ 无
- **文档测试**：❌ 无

### 代码质量特性

- **缓存优化**：平台检测结果在进程生命周期内缓存
- **错误处理**：使用 `thiserror` 提供结构化错误
- **日志记录**：记录检测到的平台和令牌颁发状态
- **平台特定编译**：使用 `cfg(not(target_family = "wasm"))` 条件编译

## 常见问题 (FAQ)

### Q: 为什么需要隔离平台检测？

A: Warp 可能在各种隔离环境中运行（自托管代理、Warp 托管代理等）。不同平台有不同的身份认证和资源访问方式，需要适配。

### Q: 工作负载令牌的用途是什么？

A: 用于向 Warp 服务器认证代理的身份，允许安全地访问服务器资源（如 Drive、AI 功能等）。

### Q: 如何在本地测试隔离平台功能？

A: 可以设置环境变量模拟：
```bash
export WARP_ISOLATION_PLATFORM=docker
export WARP_WORKLOAD_TOKEN=test-token
```

### Q: 为什么集成测试被排除？

A: 集成测试通常不在隔离平台中运行，因此会返回 `None`，避免误判。

### Q: 如何添加新的隔离平台？

A:
1. 在 `IsolationPlatformType` 枚举中添加新变体
2. 创建新的检测模块（如 `src/new_platform.rs`）
3. 在 `detect()` 函数中添加检测逻辑
4. 如果支持令牌，在 `issue_workload_token()` 中添加处理

## 使用示例

### 检测平台

```rust
use isolation_platform::{detect, IsolationPlatformType};

if let Some(platform) = detect() {
    match platform {
        IsolationPlatformType::Docker => {
            println!("Running in Docker");
        }
        IsolationPlatformType::Kubernetes => {
            println!("Running in Kubernetes");
        }
        _ => {}
    }
}
```

### 颁发令牌

```rust
use isolation_platform::issue_workload_token;
use std::time::Duration;

match issue_workload_token(Some(Duration::from_secs(3600))).await {
    Ok(token) => {
        println!("Token: {}", token.token);
        if let Some(expires) = token.expires_at {
            println!("Expires: {}", expires);
        }
    }
    Err(e) => {
        eprintln!("Failed to issue token: {}", e);
    }
}
```

## 相关文件清单

```
crates/isolation_platform/
├── Cargo.toml              # 包配置
├── src/
│   ├── lib.rs              # 主入口（188 行）
│   ├── docker.rs           # Docker 检测（11 行）
│   ├── docker_sandbox.rs   # Docker Sandbox 支持
│   ├── kubernetes.rs       # Kubernetes 检测
│   ├── namespace.rs        # Namespace 检测和令牌
│   └── namespace_tests.rs  # Namespace 测试
└── CLAUDE.md               # 本文档
```

## 变更记录

### 2026-05-01

- ✅ 创建初始模块文档
- ✅ 记录四个支持的平台及其检测方式
- ✅ 添加工作负载令牌颁发流程说明
- ✅ 添加环境变量和错误处理文档
- ✅ 添加使用示例和常见问题
- ✅ 添加导航面包屑

---

*此模块文档是 Warp AI 上下文的一部分*
