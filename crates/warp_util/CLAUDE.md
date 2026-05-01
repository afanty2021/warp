# Warp Util 模块

[根目录](../../CLRUDE.md) > [crates](../) > **warp_util**

## 模块职责

提供通用工具函数和辅助类，供所有内部 Warp crates 使用。包含文件操作、路径处理、内容版本管理等功能。

## 入口与启动

- **主要入口**：`crates/warp_util/src/lib.rs`

## 对外接口

### 核心模块

```rust
pub mod assets;              // 资源管理
pub mod content_version;     // 内容版本控制
pub mod file;                // 文件操作
pub mod file_type;           // 文件类型检测
pub mod on_cancel;           // 取消处理
pub mod path;                // 路径工具
pub mod standardized_path;   // 标准化路径
pub mod user_input;          // 用户输入处理
pub mod worktree_names;      // Worktree 命名

#[cfg(windows)]
pub mod windows;             // Windows 特定工具
```

### 功能特性

- **资源管理**：加载和管理应用资源
- **版本控制**：跟踪和比较内容版本
- **文件操作**：文件读写、类型检测
- **路径处理**：跨平台路径标准化
- **用户输入**：输入验证和处理
- **取消处理**：异步操作取消
- **Worktree 命名**：Git worktree 命名规范

## 关键依赖与配置

### 依赖项

- 详见各子模块

### 配置要求

- 跨平台兼容性
- Windows 特定代码使用 `cfg(windows)`

## 数据模型

### StandardizedPath

跨平台的路径表示，支持本地和远程路径：
- 本地路径：`/path/to/file`
- 远程路径：`ssh://host/path`

### ContentVersion

内容版本标识，用于缓存失效和增量更新。

## 测试与质量

### 测试覆盖

- ⚠️ 依赖集成测试
- 通过使用 crates 间接验证

### 代码质量

- 模块化设计
- 清晰的职责分离
- 跨平台兼容性

## 常见问题 (FAQ)

### Q: 何时使用 StandardizedPath？

A: 需要跨平台路径表示或支持远程路径时。

### Q: 文件类型检测支持哪些类型？

A: 详见 `file_type` 模块文档。

### Q: 如何处理异步操作取消？

A: 使用 `on_cancel` 模块提供的工具。

### Q: Worktree 命名有什么规则？

A: 详见 `worktree_names` 模块。

## 相关文件清单

- `src/assets.rs`
- `src/content_version.rs`
- `src/file.rs`
- `src/file_type.rs`
- `src/on_cancel.rs`
- `src/path.rs`
- `src/standardized_path.rs`
- `src/user_input.rs`
- `src/worktree_names.rs`
- `src/windows/`（Windows 特定）

## 变更记录

### 2026-05-01 (23:18)

- ✅ 初始化模块文档
- ✅ 添加工具模块列表和说明
- ✅ 添加跨平台和功能说明
