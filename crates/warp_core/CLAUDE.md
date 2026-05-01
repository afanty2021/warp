[根目录](../../CLAUDE.md) > [crates](../) > **warp_core**

# warp_core - 核心工具和特性标志

## 模块职责

`warp_core` 是 Warp 项目的基础核心库，提供跨模块共享的核心工具、类型系统和特性标志系统。它是整个 Warp 应用程序的"基础设施层"，为其他模块提供通用功能。

## 入口与启动

- **主入口文件**: `src/lib.rs`
- **核心功能模块**:
  - `features.rs` - 特性标志系统（重新导出 `warp_features`）
  - `paths.rs` - 路径和配置目录管理
  - `channel.rs` - 发布渠道管理（Stable/Dev/OSS/Integration）
  - `app_id.rs` - 应用程序标识符
  - `host_id.rs` / `session_id.rs` - 主机和会话标识

## 对外接口

### 特性标志系统 (Feature Flags)

通过 `warp_features` 重新导出，提供运行时特性门控：

```rust
use warp_core::FeatureFlag;

// 检查特性是否启用
if FeatureFlag::AgentMode.is_enabled() {
    // 启用 Agent Mode 功能
}

// 切换特性状态
FeatureFlag::AgentMode.set_enabled(true);
```

**主要特性标志类别**:
- **AI 功能**: `AgentMode`, `AgentPredict`, `AIRules`
- **UI 功能**: `ThinStrokes`, `Ligatures`, `MinimalistUI`
- **编辑器功能**: `RectSelection`, `RichTextMultiselect`, `SelectablePrompt`
- **终端功能**: `ResizeFix`, `SSHTmuxWrapper`, `ContextChips`
- **云同步**: `CloudObjects`, `SharedWithMe`
- **实验性功能**: `RuntimeFeatureFlags`, `DebugMode`

### 路径管理

提供跨平台的配置和数据目录管理：

```rust
use warp_core::paths;

// 获取 Warp 主配置目录 (~/.warp, ~/.warp-dev, 等)
let config_dir = paths::warp_home_config_dir();

// 获取技能目录
let skills_dir = paths::warp_home_skills_dir();

// 获取 MCP 配置文件路径
let mcp_config = paths::warp_home_mcp_config_file_path();
```

**路径管理原则**:
- 便携数据（可跨机器复制）vs 非便携数据（包含本地路径引用）
- 渠道隔离（Stable/Dev/OSS 使用不同目录）
- 数据档案支持（可选的数据配置文件）

### 发布渠道管理

```rust
use warp_core::channel::{Channel, ChannelState};

// 获取当前发布渠道
let channel = ChannelState::channel();

// 检查渠道类型
match channel {
    Channel::Stable => { /* 稳定版 */ },
    Channel::Dev => { /* 开发版 */ },
    Channel::Oss => { /* 开源版 */ },
    Channel::Integration => { /* 集成测试版 */ },
}
```

### 错误处理

提供统一的错误类型和宏：

- `errors.rs` - 核心错误类型
- `anyhow.rs` - 与 `anyhow` 库的集成
- `reqwest.rs` - HTTP 错误封装
- `tokio.rs` - 异步运行时错误
- `websocket.rs` - WebSocket 错误

## 关键依赖与配置

### 依赖关系

```toml
[dependencies]
warp_features = { workspace = true }  # 特性标志实现
settings = { workspace = true }        # 设置管理
dirs = { workspace = true }            # 跨平台目录
directories = { workspace = true }     # 系统目录
serde = { workspace = true }           # 序列化
parking_lot = { workspace = true }     # 高性能锁
```

### 特性标志

- `crash_reporting` - 启用 Sentry 崩溃报告
- `local_fs` - 本地文件系统功能
- `test-util` - 测试工具（使用 `mockito`）
- `integration_tests` - 集成测试支持
- `release_bundle` - 发布打包

### 平台特定依赖

- **macOS**: `cocoa`, `objc`, `objc2-foundation`
- **跨平台**: `tokio`（非 WASM）

## 数据模型

### AppId
应用程序唯一标识符，用于标识不同的 Warp 实例。

### HostId
主机标识符，用于区分不同的物理机器。

### SessionId
会话标识符，用于标识单个 Warp 会话。

### Channel
发布渠道枚举：
- `Stable` - 稳定发布版
- `Preview` - 预览版（与 Stable 共享配置目录）
- `Oss` - 开源版
- `Dev` - 开发版
- `Integration` - 集成测试版
- `Local` - 本地构建版

### FeatureFlag
特性标志枚举（60+ 标志），支持：
- 运行时查询和切换
- 基于用户/团队的启用逻辑
- A/B 测试支持
- 功能开关

## 测试与质量

### 测试文件

- `app_id_test.rs` - AppId 测试
- `channel/state_tests.rs` - 渠道状态测试
- `interval_timer_tests.rs` - 间隔计时器测试
- `paths_tests.rs` - 路径功能测试
- `sync_queue_tests.rs` - 同步队列测试
- `ui/color/color_tests.rs` - 颜色处理测试
- `ui/theme/theme_tests.rs` - 主题测试

### 运行测试

```bash
# 运行 warp_core 测试
cargo test -p warp_core

# 使用 nextest 运行
cargo nextest run -p warp_core
```

## 常见问题 (FAQ)

### Q: 如何添加新的特性标志？

1. 在 `warp_features/src/lib.rs` 的 `FeatureFlag` 枚举中添加新变体
2. （可选）在 `DOGFOOD_FLAGS` 中为 dogfood 构建默认启用
3. 使用 `FeatureFlag::YourFlag.is_enabled()` 门控代码路径
4. 对于预览或发布推出，分别添加到 `PREVIEW_FLAGS` 或 `RELEASE_FLAGS`

### Q: 路径管理如何处理跨平台差异？

使用 `dirs` 和 `directories` crate 获取平台标准目录：
- **macOS**: `~/Library/Application Support/Warp`
- **Windows**: `%APPDATA%\Warp`
- **Linux**: `~/.config/warp`（遵循 XDG 规范）

### Q: 为什么渠道配置要隔离？

隔离配置目录可以：
- 防止开发/测试版本污染稳定版配置
- 支持并行运行不同版本的 Warp
- 便于测试和开发工作流

### Q: 如何在集成测试中使用路径功能？

不要直接在集成测试中使用 `paths.rs`。应通过特定用例的辅助函数暴露路径，以避免测试依赖内部实现细节。

## 相关文件清单

### 核心模块
- `src/lib.rs` - 主入口，重新导出核心类型
- `src/features.rs` - 特性标志系统（重新导出）
- `src/paths.rs` - 路径和配置目录管理
- `src/channel/` - 发布渠道管理

### 标识符
- `src/app_id.rs` - 应用程序标识符
- `src/host_id.rs` - 主机标识符
- `src/session_id.rs` - 会话标识符

### 工具和实用程序
- `src/command.rs` - 命令处理工具
- `src/context_flag.rs` - 上下文标志
- `src/errors/` - 错误类型定义
- `src/execution_mode.rs` - 执行模式
- `src/interval_timer.rs` - 间隔计时器
- `src/semantic_selection/` - 语义选择
- `src/sync_queue.rs` - 同步队列

### UI 相关
- `src/ui/` - UI 工具（颜色、主题、图标）
- `src/ui/color/` - 颜色处理（混合、对比度）
- `src/ui/theme/` - 主题系统

### 平台特定
- `src/macos.rs` - macOS 特定功能
- `src/operating_system_info.rs` - 操作系统信息
- `src/platform.rs` - 平台检测

### 测试
- `src/*_test.rs` - 各模块的单元测试

### 配置
- `Cargo.toml` - 依赖和特性标志配置

## 变更记录 (Changelog)

### 2026-05-01 (22:54)

- ✅ 创建 warp_core 模块文档
- ✅ 记录特性标志系统使用方法
- ✅ 记录路径管理 API
- ✅ 记录发布渠道管理
- ✅ 添加导航面包屑
- ✅ 记录测试策略和常见问题

---

*本文档由 AI 自动生成和维护。如有问题或建议，请在 issue 中提出。*
