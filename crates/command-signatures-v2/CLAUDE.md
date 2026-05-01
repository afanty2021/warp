# Command Signatures V2 模块

[根目录](../../CLAUDE.md) > [crates](../) > **command-signatures-v2**

## 模块职责

提供命令签名的 JavaScript 绑定，用于命令自动补全和语法高亮。将预编译的 JS 代码嵌入 Rust 二进制文件。

## 入口与启动

- **主要入口**：`crates/command-signatures-v2/src/lib.rs`
- **嵌入资源**：`CommandSignaturesJs`

## 对外接口

### 核心类型

```rust
#[derive(RustEmbed)]
#[folder = "js/build"]
pub struct CommandSignaturesJs;

pub static COMMAND_SIGNATURES_JS: CommandSignaturesJs = CommandSignaturesJs;
```

### 功能特性

- **资源嵌入**：使用 `rust-embed` 嵌入预编译 JS
- **静态访问**：通过 `COMMAND_SIGNATURES_JS` 静态变量访问
- **编译时包含**：JS 代码在编译时嵌入二进制
- **运行时访问**：可通过 `RustEmbed` API 访问文件

## 关键依赖与配置

### 依赖项

- `rust-embed`：资源嵌入

### 配置要求

- JS 代码必须预先编译到 `js/build/` 目录
- 详见 `crates/command-signatures-v2/js/`

## 数据模型

### 文件结构

```
js/build/
  ├── (预编译的 JS 文件)
```

### 访问方式

```rust
// 获取文件内容
COMMAND_SIGNATURES_JS::get("file.js")
```

## 测试与质量

### 测试覆盖

- ⚠️ 依赖集成测试
- JS 代码本身有独立测试

### 代码质量

- 编译时验证
- 零运行时开销（嵌入二进制）

## 常见问题 (FAQ)

### Q: 如何重新编译 JS 代码？

A: 在 `js/` 目录运行构建命令。

### Q: 支持哪些命令？

A: 详见 JS 代码和配置。

### Q: 如何添加新命令签名？

A: 修改 JS 源码并重新编译。

### Q: 文件大小如何？

A: 取决于命令数量，编译时优化。

## 相关文件清单

- `src/lib.rs`：主要实现
- `js/`：JavaScript 源码和构建脚本

## 变更记录

### 2026-05-01 (23:18)

- ✅ 初始化模块文档
- ✅ 添加资源嵌入和访问说明
