# prevent_sleep - 系统休眠预防

[根目录](../../CLAUDE.md) > [crates](../) > **prevent_sleep**

> 最后更新：2026年 5月 1 日

## 模块职责

提供跨平台的系统休眠预防功能。当执行重要任务（如 Agent Mode 请求、文件传输等）时，防止系统进入睡眠模式。

## 入口与启动

### 主要入口

- **文件**：`src/lib.rs`
- **核心函数**：`prevent_sleep(reason: &'static str) -> Guard`
- **类型**：Rust 库（非可执行文件）

### 平台特定实现

| 平台 | 实现文件 | 机制 |
|------|----------|------|
| macOS | `src/mac.rs` | `NSProcessInfo` beginActivityWithOptions |
| Windows | `src/windows.rs` | `SetThreadExecutionState` API |
| 其他 | `src/noop.rs` | 无操作（no-op） |

## 对外接口

### 1. 防止休眠

```rust
pub fn prevent_sleep(reason: &'static str) -> Guard
```

创建一个防止系统休眠的守卫对象。

**参数**：
- `reason` - 防止休眠的原因（可能出现在日志中，应使用用户可见的描述）

**返回值**：
- `Guard` - RAII 风格的守卫对象

**行为**：
- 守卫存在时阻止系统休眠
- 守卫被丢弃时自动恢复休眠
- 原因应描述为用户可见（如 "Agent Mode request in-progress"）

### 2. 流程包装器

```rust
pub struct Stream<S> {
    inner: S,
    guard: Option<Guard>,
}

impl<S> Stream<S> {
    pub fn wrap(inner: S, guard: Option<Guard>) -> Self
}
```

包装一个异步流程，在轮询期间保持休眠预防。

**使用场景**：
- 长时间运行的异步操作
- 需要保持系统唤醒的流程

### 3. 守卫对象

```rust
pub struct Guard { /* 私有字段 */ }
```

RAII 风格的守卫对象，实现 `Drop` trait。

**行为**：
- **创建**：开始防止休眠
- **丢弃**：停止防止休眠，恢复系统正常行为

## 关键依赖与配置

### Cargo.toml 依赖

```toml
[dependencies]
cfg-if = "1.0"
futures = "0.3"
log = "0.4"
pin-project = "1.0.10"

[target.'cfg(windows)'.dependencies]
itertools = "0.14.0"
parking_lot = "0.12.1"
windows = { version = "0.62.2", features = ["Win32_System_Power"] }

[target.'cfg(target_os = "macos")'.dependencies]
objc2 = "0.6.3"
objc2-foundation = { version = "0.3", features = [
    "NSProcessInfo",
    "NSString",
] }

[build-dependencies]
cfg_aliases = "0.2.1"
```

### 平台特性

**macOS**：
- 使用 `NSProcessInfo.beginActivityWithOptions`
- 选项：`NSActivityOptions::UserInitiated`
- 原因通过 `NSString` 传递

**Windows**：
- 使用 `SetThreadExecutionState` API
- 标志：`ES_CONTINUOUS | ES_AWAYMODE_REQUIRED | ES_SYSTEM_REQUIRED`
- 专用后台线程管理多个任务
- 支持多个并发休眠预防请求

**其他平台**：
- 无操作实现
- 编译通过但不产生效果

## 数据模型

### macOS 实现

```rust
pub struct Guard {
    process_info: Retained<NSProcessInfo>,
    activity_token: Retained<ProtocolObject<dyn NSObjectProtocol>>,
    reason: Retained<NSString>,
}
```

**特性**：
- 使用 Objective-C2 绑定
- 线程安全（`Send + Sync`）
- 自动清理活动标记

### Windows 实现

```rust
pub struct Guard {
    task_id: u64,
    update_tx: mpsc::Sender<StateUpdate>,
}

enum StateUpdate {
    AddTask { task_id: u64, reason: &'static str },
    RemoveTask { task_id: u64 },
}
```

**特性**：
- 全局状态管理（`LazyLock<State>`）
- 专用后台线程（`"prevent_sleep"`）
- 支持多个并发任务
- 自动任务 ID 生成

## 测试与质量

### 测试覆盖

- **单元测试**：❌ 无
- **集成测试**：❌ 无
- **文档测试**：❌ 无

### 代码质量特性

- **RAII 模式**：自动资源管理
- **线程安全**：macOS 和 Windows 实现都是线程安全的
- **日志记录**：记录休眠预防的开始和结束
- **错误处理**：Windows 实现处理线程通信失败

## 常见问题 (FAQ)

### Q: 为什么需要防止休眠？

A: 某些长时间运行的任务（如 AI 请求、大文件传输）可能因系统休眠而中断。防止休眠确保任务完成。

### Q: 守卫对象何时停止防止休眠？

A: 当守卫对象被丢弃时（离开作用域），自动停止防止休眠。这是 RAII 模式。

### Q: 可以有多个并发休眠预防吗？

A: 可以。Windows 实现支持多个任务，所有任务完成后才恢复休眠。macOS 实现使用引用计数。

### Q: 如何选择好的原因字符串？

A: 使用描述性、用户可见的文本，例如：
- "Agent Mode request in-progress"
- "File transfer in progress"
- "Running build command"

### Q: 在 Linux 上有效吗？

A: Linux 上使用 noop 实现，不会产生效果。如需 Linux 支持，需要使用 DBus 抑制接口。

### Q: Stream 包装器的用途是什么？

A: 为异步流程提供便捷的休眠预防。只要流程被轮询，守卫就会保持活动。

## 使用示例

### 基本用法

```rust
use prevent_sleep::prevent_sleep;

{
    let _guard = prevent_sleep("Agent Mode request in-progress");
    // 执行长时间运行的任务
    // 系统不会休眠
    // ...
} // 守卫被丢弃，系统可以休眠
```

### 异步任务

```rust
use prevent_sleep::prevent_sleep;

async fn long_running_task() {
    let _guard = prevent_sleep("Processing large file");
    // 异步工作...
}
```

### 流程包装器

```rust
use prevent_sleep::Stream;
use futures::stream;

let guard = Some(prevent_sleep("Streaming data"));
let stream = Stream::wrap(my_stream, guard);

// 使用 stream...
```

### 多个并发任务（Windows）

```rust
let guard1 = prevent_sleep("Task 1");
let guard2 = prevent_sleep("Task 2");

// 系统保持唤醒
drop(guard1); // 仍然唤醒（guard2 还在）
drop(guard2); // 现在可以休眠
```

## 相关文件清单

```
crates/prevent_sleep/
├── Cargo.toml              # 包配置（平台特定依赖）
├── build.rs                # 构建脚本（cfg 别名）
├── src/
│   ├── lib.rs              # 主入口（48 行）
│   ├── mac.rs              # macOS 实现（43 行）
│   ├── windows.rs          # Windows 实现（166 行）
│   └── noop.rs             # 无操作实现
└── CLAUDE.md               # 本文档
```

## 变更记录

### 2026-05-01

- ✅ 创建初始模块文档
- ✅ 记录三个平台的实现差异
- ✅ 添加 RAII 模式和守卫对象说明
- ✅ 添加使用示例和常见问题
- ✅ 添加导航面包屑

---

*此模块文档是 Warp AI 上下文的一部分*
