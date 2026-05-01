# Repo Metadata 模块

[根目录](../../CLAUDE.md) > [crates](../) > **repo_metadata**

## 模块职责

提供代码仓库元数据管理功能，包括文件树构建、gitignore 处理、文件监控和本地/远程仓库支持。为 AI 功能提供代码库索引和上下文。

## 入口与启动

- **主要入口**：`crates/repo_metadata/src/lib.rs`
- **核心模型**：`LocalRepoMetadataModel`、`RemoteRepoMetadataModel`
- **包装模型**：`RepoMetadataModel`（统一接口）

## 对外接口

### 核心类型

```rust
pub struct Repository;
pub struct LocalRepoMetadataModel;
pub struct RemoteRepoMetadataModel;
pub struct RepoMetadataModel; // 包装器

pub enum RepoMetadataEvent {
    // 仓库元数据更新事件
}

pub struct DirectoryWatcher;
pub struct DirectoryEntry;
pub struct FileId;
pub struct FileMetadata;
```

### 主要功能

```rust
pub fn is_in_repo(path: &str, app: &warpui::AppContext) -> bool;
```

### 功能特性

- **仓库检测**：自动检测 Git 仓库
- **文件树构建**：高效的文件树索引
- **gitignore 支持**：正确处理 `.gitignore` 规则
- **文件监控**：实时监控仓库变化
- **本地/远程支持**：本地文件系统和远程服务器
- **路径规范化**：`CanonicalizedPath` 确保路径一致性
- **事件流**：`RepoMetadataEvent` 流式更新

## 关键依赖与配置

### 依赖项

- `warpui`：UI 框架集成（SingletonEntity）
- `warp_util`：工具函数（StandardizedPath）
- `thiserror`：错误处理

### 配置要求

- 需要有效的 Git 仓库
- 文件系统监控权限
- 路径编码必须匹配本地 OS

## 数据模型

### 仓库标识符

```rust
pub enum RepositoryIdentifier {
    Local(PathBuf),
    Remote(RemoteRepositoryIdentifier),
}

pub struct RemoteRepositoryIdentifier {
    // 远程仓库标识
}
```

### 目录条目

```rust
pub struct DirectoryEntry {
    pub path: PathBuf,
    pub metadata: FileMetadata,
}

pub struct FileMetadata {
    pub is_dir: bool,
    pub is_file: bool,
    pub size: Option<u64>,
    // ...
}
```

### 文件树

- **FileTreeEntry**：文件树节点
- **FileTreeStore**：文件树存储
- **RepoContent**：仓库内容抽象

## 测试与质量

### 测试覆盖

- ✅ 单元测试：`crates/repo_metadata/src/repository_tests.rs`
- ✅ 命名空间测试：`src/namespace_tests.rs`
- 测试内容：
  - 文件树构建
  - gitignore 处理
  - 路径规范化
  - 仓库检测

### 代码质量

- 使用 `thiserror` 定义错误类型
- 清晰的模块分离（local/remote/wrapper）
- 事件驱动架构

## 常见问题 (FAQ)

### Q: 如何检测路径是否在仓库中？

A: 使用 `is_in_repo(path, app)` 函数。

### Q: 如何处理大型仓库？

A: 文件树构建使用增量更新和懒加载。

### Q: gitignore 规则是如何处理的？

A: 使用 `matches_gitignores()` 和 `should_ignore_git_path()` 函数。

### Q: 如何监控仓库变化？

A: 使用 `DirectoryWatcher` 监控文件系统事件。

### Q: 支持远程仓库吗？

A: 是的，通过 `RemoteRepoMetadataModel` 支持远程服务器。

## 相关文件清单

- `src/lib.rs`：主要导出和类型定义
- `src/repository.rs`：仓库抽象
- `src/local_model.rs`：本地仓库模型
- `src/remote_model.rs`：远程仓库模型
- `src/wrapper_model.rs`：统一包装器
- `src/entry.rs`：目录条目和文件树
- `src/file_tree_store.rs`：文件树存储
- `src/watcher.rs`：文件系统监控
- `src/repository_identifier.rs`：仓库标识符
- `src/repositories/`：仓库检测和管理

## 变更记录

### 2026-05-01 (23:18)

- ✅ 初始化模块文档
- ✅ 添加仓库模型和接口说明
- ✅ 添加文件树和监控说明
- ✅ 添加本地/远程支持文档
