# Warp Ripgrep 模块

[根目录](../../CLRUDE.md) > [crates](../) > **warp_ripgrep**

## 模块职责

提供 ripgrep 的薄包装，用于文件搜索功能。支持工作进程模式，包括父进程监控和自动退出。

## 入口与启动

- **主要入口**：`crates/warp_ripgrep/src/lib.rs`
- **搜索模块**：`search`
- **类型定义**：`types`（非 WASM）

## 对外接口

### 核心功能

```rust
pub mod search;
#[cfg(not(target_family = "wasm"))]
mod types;
```

### Unix 特定功能

```rust
#[cfg(unix)]
pub fn monitor_parent_and_exit_on_change(parent_pid: Option<u32>);
```

### 功能特性

- **文件搜索**：基于 ripgrep 的高性能搜索
- **工作进程模式**：独立进程处理搜索
- **父进程监控**：Unix 系统监控父进程状态
- **自动退出**：父进程退出时自动终止
- **资源清理**：避免僵尸进程

## 关键依赖与配置

### 依赖项

- ripgrep 库
- `nix`（Unix 系统调用）

### 配置要求

- Unix 系统支持父进程监控
- 工作进程模式需要正确配置 PID

## 数据模型

### 搜索结果

详见 `search` 模块和 `types` 模块。

### 父子进程关系

- 主进程启动 ripgrep 工作进程
- 工作进程监控父进程 PID
- 父进程退出时工作进程自动终止

## 测试与质量

### 测试覆盖

- ⚠️ 依赖集成测试
- 通过实际搜索验证

### 代码质量

- 平台特定代码隔离
- 资源管理

## 常见问题 (FAQ)

### Q: 为什么使用工作进程模式？

A: 隔离搜索操作，避免阻塞主进程。

### Q: 父进程监控如何工作？

A: 每秒检查父 PID 是否变化，变化则退出。

### Q: Windows 支持吗？

A: 基本支持，但父进程监控仅限 Unix。

### Q: 如何配置搜索参数？

A: 详见 `search` 模块文档。

### Q: 性能如何？

A: 基于 ripgrep，非常高效。

## 相关文件清单

- `src/lib.rs`：主要实现
- `src/search/`：搜索功能
- `src/types.rs`：类型定义（非 WASM）

## 变更记录

### 2026-05-01 (23:18)

- ✅ 初始化模块文档
- ✅ 添加 ripgrep 包装和搜索说明
- ✅ 添加工作进程和监控文档
