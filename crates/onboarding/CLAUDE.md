# Onboarding 模块

[根目录](../../CLAUDE.md) > [crates](../) > **onboarding**

## 模块职责

管理 Warp 的首次启动用户体验，包括意图选择、设置配置、AI 功能介绍和登录流程。提供可定制的入门引导流程。

## 入口与启动

- **主要入口**：`crates/onboarding/src/lib.rs`
- **初始化函数**：`init(app: &mut warpui::AppContext)`
- **主要视图**：`AgentOnboardingView`、`OnboardingCalloutView`

## 对外接口

### 核心类型

```rust
pub enum OnboardingIntention {
    Terminal,
    AgentDrivenDevelopment,
}

pub enum SessionDefault {
    Agent,
    Terminal,
}

pub struct AgentOnboardingView;
pub struct OnboardingCalloutView;
```

### 主要事件

```rust
pub enum AgentOnboardingEvent {
    // 用户交互事件
}
```

### 功能特性

- **意图选择**：Terminal 或 Agent 驱动开发
- **功能介绍**：AI 功能和 Warp Drive 功能列表
- **设置定制**：`SelectedSettings`、`UICustomizationSettings`
- **遥测支持**：`OnboardingEvent` 事件跟踪
- **跳过登录**：支持跳过登录的确认对话框
- **视觉元素**：幻灯片、标注和视觉提示

## 关键依赖与配置

### 依赖项

- `warpui`：UI 框架
- `warp_completer`：自动补全集成

### 配置要求

- 需要在应用启动时调用 `init()`
- 遥测提供者（feature-gated）

## 数据模型

### 意图和功能

```rust
pub const AI_FEATURES: &[&str] = &[
    "Warp agents",
    "Oz cloud agents platform",
    "Next command predictions",
    "Prompt suggestions",
    "Codebase context",
    "Remote control with Claude Code, Codex, and other agents",
    "Agents over SSH",
];

pub const WARP_DRIVE_FEATURES: &[&str] = &[
    "Warp Drive",
    "Session Sharing",
];
```

### 状态模型

- **OnboardingAuthState**：认证状态
- **ProjectOnboardingSettings**：项目设置
- **SelectedSettings**：用户选择的设置

## 测试与质量

### 测试覆盖

- ⚠️ 无公开单元测试
- 依赖手动测试和用户反馈

### 代码质量

- 清晰的枚举和常量定义
- 模块化组件设计
- 遥测事件跟踪

## 常见问题 (FAQ)

### Q: 如何自定义入门流程？

A: 修改 `slides` 和 `components` 模块中的视图。

### Q: 如何添加新的功能介绍？

A: 更新 `AI_FEATURES` 或 `WARP_DRIVE_FEATURES` 常量。

### Q: 遥测数据如何使用？

A: 通过 `OnboardingEvent` 发送到遥测系统，用于产品改进。

### Q: 如何处理跳过登录？

A: 显示确认对话框，列出将启用的功能。

## 相关文件清单

- `src/lib.rs`：主要导出和类型定义
- `src/agent_onboarding_view.rs`：Agent 入门视图
- `src/model.rs`：状态模型
- `src/slides/`：幻灯片组件
- `src/callout.rs`：标注视图
- `src/telemetry.rs`：遥测事件
- `src/visuals.rs`：视觉元素
- `src/components/`：可复用组件

## 变更记录

### 2026-05-01 (23:18)

- ✅ 初始化模块文档
- ✅ 添加意图和功能说明
- ✅ 添加组件和事件文档
- ✅ 添加遥测和定制说明
