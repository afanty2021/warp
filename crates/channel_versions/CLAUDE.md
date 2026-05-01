# Channel Versions 模块

[根目录](../../CLAUDE.md) > [crates](../) > **channel_versions**

## 模块职责

管理 Warp 的发布渠道版本信息，包括 Dev、Preview、Stable 三个渠道的版本、更新截止时间和变更日志。支持环境特定的版本覆盖。

## 入口与启动

- **主要入口**：`crates/channel_versions/src/lib.rs`
- **核心类型**：`ChannelVersions`、`ChannelVersion`、`VersionInfo`

## 对外接口

### 核心类型

```rust
pub struct ChannelVersions {
    pub dev: ChannelVersion,
    pub preview: ChannelVersion,
    pub stable: ChannelVersion,
    pub changelogs: Option<ChannelChangelogs>,
}

pub struct ChannelVersion {
    // 内部字段，通过方法访问
}

pub struct VersionInfo {
    pub version: String,
    pub version_for_new_users: Option<String>,
    pub update_by: Option<DateTime<FixedOffset>>,
    pub soft_cutoff: Option<String>,
    pub last_prominent_update: Option<String>,
    pub is_rollback: Option<bool>,
    pub cli_version: Option<String>,
}
```

### 主要方法

```rust
impl ChannelVersion {
    // 应用环境特定的覆盖
    pub fn version_info(&self) -> VersionInfo;
    pub fn version_info_for_execution_context(&self, context: &Context) -> VersionInfo;
}

impl VersionInfo {
    pub fn new(version: String) -> Self;
    pub fn cli_version(&self) -> &str;  // 回退到 app version
}
```

### 变更日志

```rust
pub struct ChannelChangelogs {
    pub dev: HashMap<String, Changelog>,
    pub preview: HashMap<String, Changelog>,
    pub stable: HashMap<String, Changelog>,
}

pub struct Changelog {
    pub date: DateTime<FixedOffset>,
    pub sections: Vec<Section>,
    pub markdown_sections: Vec<MarkdownSection>,
    pub image_url: Option<String>,
    pub oz_updates: Vec<String>,
}
```

### 功能特性

- **三渠道支持**：Dev、Preview、Stable
- **版本覆盖**：环境特定的版本覆盖规则
- **更新提示**：`update_by` 截止时间和警告横幅
- **软性截止**：`soft_cutoff` 最小支持版本
- **CLI 版本**：独立的 CLI 版本管理
- **变更日志**：结构化 Markdown 变更日志
- **Oz 更新**：Oz CLI 相关更新
- **版本解析**：`v{major}.{date}.{patch}` 格式

## 关键依赖与配置

### 依赖项

- `chrono`：日期时间处理
- `serde`：序列化/反序列化
- `regex`：版本解析
- `lazy_static`：编译时正则初始化
- `memo-map`：版本解析缓存

### 配置要求

- 版本格式：`v{major}.{YYYY.MMDD.HH.MM}.{patch}`
- 环境上下文用于覆盖规则

## 数据模型

### 版本格式

```
v{major}.{YYYY.MMDD.HH.MM}.{patch}
```

例如：`v1.2026.05.01.23.18_0`

### 解析缓存

使用 `MemoMap` 缓存已解析的版本，提高性能。

### 覆盖系统

基于 `overrides::Context` 应用环境特定的覆盖规则。

## 测试与质量

### 测试覆盖

- ✅ 单元测试：`src/channel_versions_tests.rs`
- ✅ 覆盖测试：`src/overrides_tests.rs`
- 测试内容：
  - 版本解析和比较
  - 覆盖规则应用
  - 变更日志格式化

### 代码质量

- 版本解析缓存
- 清晰的类型定义
- 环境感知的覆盖系统

## 常见问题 (FAQ)

### Q: 如何获取当前渠道版本？

A: 使用 `ChannelVersion::version_info()` 应用覆盖后获取。

### Q: CLI 版本如何确定？

A: 优先使用 `cli_version` 字段，否则回退到 `version`。

### Q: 变更日志如何格式化？

A: 实现 `Display` trait，输出 Markdown 格式。

### Q: 如何添加新渠道？

A: 修改 `ChannelVersions` 结构体和解析逻辑。

### Q: 覆盖规则如何工作？

A: 基于 `overrides::Context` 从环境变量读取。

### Q: 版本比较如何进行？

A: 按 (major, date, patch) 元组比较。

## 相关文件清单

- `src/lib.rs`：主要实现
- `src/overrides.rs`：覆盖规则
- `src/channel_versions_tests.rs`：单元测试
- `src/overrides_tests.rs`：覆盖测试

## 变更记录

### 2026-05-01 (23:18)

- ✅ 初始化模块文档
- ✅ 添加版本管理和渠道说明
- ✅ 添加覆盖系统和变更日志文档
