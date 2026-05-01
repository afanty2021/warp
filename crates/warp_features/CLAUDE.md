# Warp Features 模块

[根目录](../../CLAUDE.md) > [crates](../) > **warp_features**

## 模块职责

定义和管理 Warp 的所有功能标志（Feature Flags），支持运行时切换、用户偏好设置和测试覆盖。控制功能发布的不同阶段（Dogfood、Preview、Release）。

## 入口与启动

- **主要入口**：`crates/warp_features/src/lib.rs`
- **核心类型**：`FeatureFlag` 枚举
- **初始化**：`mark_initialized()`

## 对外接口

### 核心方法

```rust
impl FeatureFlag {
    // 检查功能是否启用
    pub fn is_enabled(&self) -> bool;

    // 设置全局状态（仅非测试代码）
    pub fn set_enabled(self, enabled: bool);

    // 设置用户偏好（优先级高于全局状态）
    pub fn set_user_preference(self, enabled: bool);

    // 测试覆盖（feature-gated）
    #[cfg(feature = "test-util")]
    pub fn override_enabled(self, enabled: bool) -> OverrideGuard;

    // 获取功能描述（用于变更日志）
    pub fn flag_description(&self) -> Option<&'static str>;
}
```

### 功能分组

#### Dogfood Flags（开发团队）
- `AgentModeWorkflows`、`AgentModeAnalytics`
- `LazySceneBuilding`、`MultiWorkspace`
- `OzPlatformSkills`、`AgentHarness`
- 约 60+ 标志

#### Preview Flags（功能预览）
- `Orchestration`、`MarkdownTables`
- `OzIdentityFederation`、`GitOperationsInCodeReview`
- 约 5 个标志

#### Release Flags（正式发布）
- `Autoupdate`、`Changelog`、`CrashReporting`
- `ImeMarkedText`（macOS）、`SshRemoteServer`（非 Windows）

#### Runtime Flags（运行时切换）
- 当前为空（`RUNTIME_FEATURE_FLAGS`）

### 功能特性

- **原子状态**：使用 `AtomicBool` 保证线程安全
- **三级优先级**：测试覆盖 > 用户偏好 > 全局状态
- **类型安全**：枚举类型 + 编译时检查
- **描述系统**：为 UI 和变更日志提供描述
- **测试隔离**：线程本地覆盖，不影响其他线程

## 关键依赖与配置

### 依赖项

- `enum-iterator`：枚举迭代和计数
- 详见 `warp_core` 重新导出

### 特性标志

- `test-util`：启用测试覆盖功能

### 配置要求

- 必须在使用前调用 `mark_initialized()`
- Debug 模式下强制检查初始化状态

## 数据模型

### 功能标志枚举

```rust
pub enum FeatureFlag {
    // 示例标志
    AgentMode,
    AgentPredict,
    SettingsFile,
    VimCodeEditor,
    // ... 约 300+ 个标志
}
```

### 状态存储

```rust
static FLAG_STATES: [AtomicBool; N];  // 全局状态
static USER_PREFERENCE_MAP: [AtomicTriState; N];  // 用户偏好
```

### 测试覆盖

```rust
#[cfg(feature = "test-util")]
thread_local! {
    static FLAG_OVERRIDES: RefCell<HashMap<FeatureFlag, bool>>;
}
```

## 测试与质量

### 测试覆盖

- ✅ 单元测试：`src/features_test.rs`
- 测试内容：
  - 功能启用/禁用
  - 用户偏好设置
  - 测试覆盖隔离
  - 多线程安全

### 代码质量

- 原子操作保证线程安全
- 清晰的优先级系统
- 完善的测试隔离

## 常见问题 (FAQ)

### Q: 如何添加新功能标志？

A: 在 `FeatureFlag` 枚举中添加新变体，根据发布阶段添加到相应常量。

### Q: 用户偏好如何工作？

A: 通过 `set_user_preference()` 设置，优先级高于全局状态。

### Q: 如何在测试中使用？

A: 使用 `override_enabled()` 创建 RAII guard，自动清理。

### Q: 功能描述如何使用？

A: `flag_description()` 返回 UI 文本，用于变更日志和设置界面。

### Q: 运行时切换安全吗？

A: 使用原子操作，线程安全。但当前 `RUNTIME_FEATURE_FLAGS` 为空。

### Q: 如何调试功能状态？

A: 使用 `DEBUG_FLAGS`（`DebugMode`、`RuntimeFeatureFlags`）。

## 相关文件清单

- `src/lib.rs`：主要实现
- `src/features_test.rs`：单元测试

## 变更记录

### 2026-05-01 (23:18)

- ✅ 初始化模块文档
- ✅ 添加功能标志系统说明
- ✅ 添加状态管理和优先级文档
- ✅ 添加测试和覆盖说明
