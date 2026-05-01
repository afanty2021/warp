[根目录](../../CLAUDE.md) > **crates/node_runtime**

# Warp Node Runtime 模块

> 最后更新：2026年 5月 1日

## 模块职责

Warp Node Runtime 模块提供 Node.js 和 npm 运行时管理功能，使 Warp 能够：

- **自动安装 Node.js**：下载并安装指定版本的 Node.js
- **多平台支持**：支持 macOS、Linux、Windows（x64、arm64）
- **npm 包管理**：查询和管理 npm 包信息
- **版本检测**：检测系统已安装的 Node.js 版本
- **本地文件系统**：管理 Node.js 安装和缓存

## 入口与启动

### 主要入口点

- `src/lib.rs` - 库入口，包含所有公共 API

### 核心功能

1. **Node.js 安装** - 自动下载和安装 Node.js
2. **版本检测** - 检测系统 Node.js 版本
3. **npm 查询** - 查询 npm 包信息
4. **平台支持** - 跨平台二进制管理

## 对外接口

### 核心 API

**Node.js 版本信息**：
```rust
#[cfg(feature = "local_fs")]
const NODE_VERSION: &str = "v22.12.0";

#[cfg(feature = "local_fs")]
const MIN_NODE_VERSION: Version = Version::new(20, 0, 0);
```

**npm 包信息**：
```rust
#[derive(Debug, Deserialize)]
pub struct NpmInfo {
    #[serde(default)]
    dist_tags: NpmInfoDistTags,
}

impl NpmInfo {
    pub fn latest_version(&self) -> Option<&str>;
}
```

**归档类型**：
```rust
#[cfg(feature = "local_fs")]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ArchiveType {
    TarGz,  // macOS/Linux
    Zip,    // Windows
}
```

### 平台检测

```rust
#[cfg(feature = "local_fs")]
struct NodeDistribution {
    os: &'static str,
    arch: &'static str,
    archive_type: ArchiveType,
}
```

## 关键依赖与配置

### 依赖

- `async-fs` - 异步文件系统操作
- `command` - 命令执行
- `flate2` - gzip 解压
- `tar` - tar 归档处理
- `zip` - zip 归档处理
- `semver` - 语义版本管理
- `sha2` - SHA256 校验和
- `http_client` - HTTP 下载

### 特性标志

- **local_fs** - 启用本地文件系统功能
- **平台特定** - 自动检测目标平台

### 配置

Node.js 安装配置：
- 固定版本：v22.12.0
- 最低版本：20.0.0
- 平台特定二进制路径

## 数据模型

### 平台二进制路径

```rust
cfg_if::cfg_if! {
    if #[cfg(all(feature = "local_fs", windows))] {
        const NODE_BINARY_PATH: &str = "node.exe";
        const NPM_BINARY_PATH: &str = "node_modules/npm/bin/npm-cli.js";
    } else if #[cfg(feature = "local_fs")] {
        const NODE_BINARY_PATH: &str = "bin/node";
        const NPM_BINARY_PATH: &str = "bin/npm";
    }
}
```

### npm 包信息

```rust
#[derive(Debug, Deserialize)]
pub struct NpmInfoDistTags {
    latest: Option<String>,
}
```

## 测试与质量

### 单元测试

当前测试覆盖：
- ⚠️ 版本检测
- ⚠️ npm 包查询
- ⚠️ 下载和安装流程
- ⚠️ 平台特定逻辑

运行测试：
```bash
cargo nextest run -p node_runtime
```

## 常见问题 (FAQ)

### Q: 如何获取 npm 包的最新版本？

A: 使用 `NpmInfo::latest_version`：
```rust
let info: NpmInfo = fetch_npm_info("package-name").await?;
if let Some(version) = info.latest_version() {
    println!("Latest version: {}", version);
}
```

### Q: 如何检测系统 Node.js 版本？

A: 执行 `node --version` 命令并解析输出：
```rust
let version = detect_node_version().await?;
if version >= MIN_NODE_VERSION {
    println!("Node.js is compatible");
}
```

### Q: 如何安装 Node.js？

A: 下载对应平台的二进制包并解压：
```rust
install_nodejs().await?;
```

### Q: 支持哪些平台？

A:
- macOS (x64, arm64)
- Linux (x64, arm64)
- Windows (x64)

## 相关文件清单

### 核心文件

- `src/lib.rs` - 库入口和所有实现

### 构建依赖

- `build.rs` - 构建脚本（如果需要）

## 高级功能

### 平台检测

自动检测运行平台并选择对应的 Node.js 二进制：
```rust
cfg_if::cfg_if! {
    if #[cfg(target_os = "macos")] {
        // macOS 特定逻辑
    } else if #[cfg(target_os = "linux")] {
        // Linux 特定逻辑
    } else if #[cfg(windows)] {
        // Windows 特定逻辑
    }
}
```

### 版本管理

使用 `semver` crate 进行版本比较：
```rust
use semver::Version;

let version = Version::parse("20.0.0")?;
if version >= MIN_NODE_VERSION {
    // 版本兼容
}
```

### 异步下载

使用 `http_client` 异步下载 Node.js 二进制：
```rust
async fn download_nodejs(url: &str) -> Result<Vec<u8>> {
    let client = HttpClient::new();
    let response = client.get(url).await?;
    Ok(response.bytes().await?)
}
```

## 平台支持

### 支持的平台

- ✅ macOS（x64, arm64）
- ✅ Linux（x64, arm64）
- ✅ Windows（x64）
- ❌ WASM（文件系统访问受限）

### 平台特定配置

```rust
#[cfg(all(feature = "local_fs", windows))]
// Windows 特定代码

#[cfg(all(feature = "local_fs", not(windows)))]
// Unix 特定代码
```

## 性能考虑

### 优化策略

1. **缓存安装**：避免重复下载
2. **增量下载**：支持断点续传
3. **并行解压**：加速安装过程
4. **版本复用**：优先使用系统安装的 Node.js

### 性能指标

- 下载速度
- 安装时间
- 磁盘空间使用

## 安全考虑

### 校验和验证

使用 SHA256 校验和验证下载的文件：
```rust
use sha2::{Sha256, Digest};

fn verify_checksum(data: &[u8], expected: &str) -> bool {
    let mut hasher = Sha256::new();
    hasher.update(data);
    let result = hasher.finalize();
    format!("{:x}", result) == expected
}
```

### 最佳实践

1. 验证下载的校验和
2. 使用官方源
3. 限制文件权限
4. 清理临时文件

## 集成点

### 与其他模块的集成

- **warp_files** - 文件系统操作
- **http_client** - HTTP 下载
- **command** - 命令执行

### 使用场景

1. **AI Agent** - 执行 Node.js 脚本
2. **代码执行** - 运行 JavaScript 代码
3. **包管理** - 安装和管理 npm 包
4. **开发工具** - 提供完整的 Node.js 开发环境

## 变更记录

### 2026-05-01

- 初始化 Node Runtime 模块文档
- 记录核心功能和 API
- 添加平台支持信息
- 记录安全考虑
