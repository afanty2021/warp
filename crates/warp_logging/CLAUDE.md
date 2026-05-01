# Warp Logging 模块

[根目录](../../CLRUDE.md) > [crates](../) > **warp_logging**

## 模块职责

提供统一的日志系统，支持文件输出、stderr 输出、日志轮转、崩溃恢复日志和日志打包。区分 CLI 和 GUI 应用的日志行为。

## 入口与启动

- **主要入口**：`crates/warp_logging/src/lib.rs`
- **初始化函数**：`init(config: LogConfig)`

## 对外接口

### 核心类型

```rust
pub enum LogDestination {
    File,    // 写入文件
    Stderr,  // 写入 stderr
}

pub struct LogConfig {
    pub is_cli: bool,  // 是否为 CLI
    pub log_destination: Option<LogDestination>,  // 输出目标
}
```

### 主要函数

```rust
// 初始化日志系统
pub fn init(config: LogConfig);

// 非 WASM 平台额外功能
#[cfg(not(target_family = "wasm"))]
pub fn log_directory() -> PathBuf;
pub fn log_file_path() -> PathBuf;
pub fn rotate_log_files();
pub fn create_log_bundle_zip() -> Result<PathBuf>;

// 崩溃恢复相关
pub fn init_for_crash_recovery_process();
pub fn init_logging_for_unit_tests();
pub fn on_crash_recovery_process_killed();
pub fn on_parent_process_crash();
```

### 功能特性

- **双目标输出**：文件或 stderr
- **日志轮转**：自动轮转旧日志
- **CLI 区分**：CLI 使用单独子目录和更高轮转限制
- **崩溃恢复**：专门的崩溃恢复日志
- **日志打包**：打包为 ZIP 便于分享
- **单元测试支持**：测试环境日志配置
- **跨平台**：Native 和 WASM 平台支持

## 关键依赖与配置

### 依赖项

- `log`：日志 facade
- `env_logger` / `simplelog`：日志实现
- 详见平台特定实现

### 配置要求

- 日志目录写权限
- CLI 标识（影响日志位置）

## 数据模型

### 日志文件组织

- **GUI 应用**：`logs/warp-gui.log`
- **CLI 应用**：`logs/cli/warp-cli.log`
- **崩溃日志**：`logs/crash_recovery.log`

### 日志轮转

- 自动轮转旧日志文件
- 保留最近的日志文件
- CLI 使用更高的轮转限制

## 测试与质量

### 测试覆盖

- ⚠️ 依赖集成测试
- 单元测试专用初始化函数

### 代码质量

- 平台抽象（cfg attributes）
- 清晰的错误处理
- 资源管理

## 常见问题 (FAQ)

### Q: 如何获取日志文件路径？

A: 使用 `log_directory()` 和 `log_file_path()` 函数。

### Q: CLI 日志有何不同？

A: 存储在单独子目录，轮转限制更高。

### Q: 如何手动触发日志轮转？

A: 调用 `rotate_log_files()`。

### Q: 崩溃日志如何工作？

A: 使用 `init_for_crash_recovery_process()` 初始化。

### Q: WASM 平台支持吗？

A: 基本支持，但文件相关功能不可用。

### Q: 如何分享日志？

A: 使用 `create_log_bundle_zip()` 创建 ZIP 包。

## 相关文件清单

- `src/lib.rs`：主要导出
- `src/native.rs`：原生平台实现
- `src/wasm.rs`：WASM 平台实现

## 变更记录

### 2026-05-01 (23:18)

- ✅ 初始化模块文档
- ✅ 添加日志系统和配置说明
- ✅ 添加平台支持和崩溃恢复文档
