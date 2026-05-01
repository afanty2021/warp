[根目录](../../CLAUDE.md) > [crates](../) > **warp_cli**

# warp_cli - CLI 工具

> 最后更新：2026年 5月 1日 星期五 22時40分12秒 CST

## 模块职责

`warp_cli` 是 Warp 的命令行接口（CLI）工具，负责解析和处理命令行参数。它为 Warp 提供了 Oz CLI 的实现，用于运行、管理和编排编码代理。

## 入口与启动

### 主要入口

- **`src/lib.rs`** - 库入口，导出所有 CLI 功能
  - 定义全局选项和命令行参数解析器
  - 导出所有子模块（agent、completions、config_file 等）

### CLI 命令结构

```rust
// 主要命令结构（通过 clap derive）
#[derive(Debug, Default, Parser, Clone)]
#[command(name = "oz", display_name = "Oz")]
pub struct OzArgs {
    #[command(subcommand)]
    pub subcommand: OzSubcommand,
}
```

## 对外接口

### 核心模块

1. **`agent`** - AI 代理相关命令
   - `OutputFormat` - 输出格式枚举（JSON、NDJSON、Pretty、Text）
   - `Prompt` 和 `PromptArg` - 提示词参数
   - `ComputerUseArgs` - 计算机使用能力控制

2. **`completions`** - Shell 自动补全生成
   - 支持多种 shell（bash、zsh、fish 等）

3. **`config_file`** - 配置文件管理
   - `ConfigFileArgs` - 配置文件参数

4. **`environment`** - 环境管理
   - `EnvironmentCreateArgs` - 环境创建参数

5. **`mcp`** - MCP（Model Context Protocol）支持
   - `MCPSpec` - MCP 规范定义

6. **`model`** - 模型相关参数
   - `ModelArgs` - 模型选择和配置

7. **`scope`** - 对象作用域
   - `ObjectScope` - 作用域定义

8. **`share`** - 分享功能
   - `ShareArgs` - 分享参数

9. **`skill`** - 技能管理
   - `SkillSpec` - 技能规范

10. **`task`** - 任务管理
    - 任务执行和查询

11. **`secret`** - 密钥管理
    - 安全存储和访问

12. **`provider`** - 提供商配置
    - AI 服务提供商设置

13. **`schedule`** - 调度功能
    - 定时任务和调度

14. **`integration`** - 集成管理
    - 第三方服务集成

15. **`federate`** - 联合功能
    - 跨服务协调

16. **`harness_support`** - 测试工具支持
    - 测试工具集成

17. **`artifact`** - 产物管理
    - 文件和内容产物

18. **`json_filter`** - JSON 过滤
    - 使用 jaq 进行 JSON 查询和过滤

### 全局选项

```rust
pub struct GlobalOptions {
    pub api_key: Option<String>,        // API 密钥
    pub output_format: OutputFormat,    // 输出格式
}
```

### 环境变量

- `WARP_API_KEY` - API 密钥
- `WARP_OUTPUT_FORMAT` - 输出格式
- `OZ_RUN_ID` - 运行 ID
- `OZ_PARENT_RUN_ID` - 父运行 ID
- `OZ_CLI` - CLI 标识
- `OZ_HARNESS_ENV` - 工具环境标识
- `WARP_SERVER_ROOT_URL` - 服务器 URL 覆盖
- `WARP_WS_SERVER_URL` - WebSocket 服务器 URL 覆盖
- `WARP_SESSION_SHARING_SERVER_URL` - 会话共享服务器 URL 覆盖

## 关键依赖与配置

### 依赖项

- **`clap`** (4.5) - 命令行参数解析
  - features: `derive`, `env`
- **`warp_core`** - 核心功能
  - ChannelState（通道状态）
  - FeatureFlag（特性标志）
- **`warp_util`** - 工具函数
- **`chrono`** - 时间处理
- **`url`** - URL 解析
- **`uuid`** - UUID 生成
- **`jaq-all`** - JSON 查询和过滤
- **`anyhow`** - 错误处理
- **`color-print`** - 彩色输出
- **`clap_complete`** - Shell 补全生成

### 特性标志

- **`plugin_host`** - 插件主机功能
- **`integration_tests`** - 集成测试
- **`api_key_authentication`** - API 密钥认证

### 平台特定依赖

**Windows**:
- `windows` - Win32 API 绑定
  - features: `Win32_Foundation`

## 数据模型

### 核心数据结构

```rust
// 输出格式
pub enum OutputFormat {
    Json,      // JSON 格式
    Ndjson,    // 换行分隔的 JSON
    Pretty,    // 人类可读格式（默认）
    Text,      // 纯文本
}

// 提示词
pub enum Prompt {
    PlainText(String),     // 纯文本提示
    SavedPrompt(String),   // 已保存的提示 ID
}

// 父进程选项
pub struct ParentOpts {
    pub pid: Option<u32>,                    // 父进程 ID
    #[cfg(windows)]
    pub handle: Option<ProcessHandle>,       // Windows 进程句柄
}

// 远程服务器身份参数
pub struct RemoteServerIdentityArgs {
    pub identity_key: String,  // 身份密钥
}
```

## 测试与质量

### 测试文件

- **`src/lib_tests.rs`** - 库测试
- **`src/json_filter_tests.rs`** - JSON 过滤测试
- **`src/mcp_tests.rs`** - MCP 测试
- **`src/share_tests.rs`** - 分享功能测试
- **`src/skill_tests.rs`** - 技能测试
- **`src/task_tests.rs`** - 任务测试

### 测试策略

- 使用 `serial_test` 确保测试顺序执行
- 单元测试覆盖核心功能
- 集成测试覆盖 CLI 交互

### 代码质量

- 遵循 Rust 2021 edition 标准
- 使用 clap derive 宏进行类型安全的参数解析
- 完善的错误处理（`anyhow`、`thiserror`）

## 常见问题 (FAQ)

### Q1: 如何添加新的 CLI 子命令？

1. 在相应的模块中定义参数结构
2. 使用 clap derive 宏添加 `#[derive(Args, Subcommand)]`
3. 在主 `OzSubcommand` 枚举中添加新变体
4. 实现命令处理逻辑

### Q2: 如何使用环境变量配置 CLI？

```bash
# 设置 API 密钥
export WARP_API_KEY="your-api-key"

# 设置输出格式
export WARP_OUTPUT_FORMAT="json"

# 设置服务器 URL
export WARP_SERVER_ROOT_URL="http://localhost:8080"
```

### Q3: 如何生成 Shell 补全？

```bash
# 生成 bash 补全
oz completions bash

# 生成 zsh 补全
oz completions zsh

# 生成 fish 补全
oz completions fish
```

### Q4: 如何集成新的 AI 提供商？

在 `provider.rs` 中添加新的提供商配置：
```rust
pub struct ProviderArgs {
    #[arg(long = "provider")]
    pub provider: Option<String>,
    // 其他提供商特定参数
}
```

## 相关文件清单

### 核心文件

- `Cargo.toml` - 包配置
- `src/lib.rs` - 库入口
- `src/agent.rs` - 代理命令
- `src/completions.rs` - 补全生成
- `src/config_file.rs` - 配置文件
- `src/environment.rs` - 环境管理
- `src/mcp.rs` - MCP 支持
- `src/model.rs` - 模型参数
- `src/skill.rs` - 技能管理
- `src/task.rs` - 任务管理

### 测试文件

- `src/lib_tests.rs`
- `src/json_filter_tests.rs`
- `src/mcp_tests.rs`
- `src/share_tests.rs`
- `src/skill_tests.rs`
- `src/task_tests.rs`

### 其他文件

- `src/scope.rs` - 作用域
- `src/share.rs` - 分享
- `src/secret.rs` - 密钥
- `src/provider.rs` - 提供商
- `src/schedule.rs` - 调度
- `src/integration.rs` - 集成
- `src/federate.rs` - 联合
- `src/harness_support.rs` - 工具支持
- `src/artifact.rs` - 产物
- `src/json_filter.rs` - JSON 过滤

## 变更记录

### 2026-05-01 (22:40)

- ✅ 初始化模块文档
- ✅ 记录核心模块和接口
- ✅ 添加测试覆盖说明
- ✅ 记录依赖和配置
- ✅ 添加常见问题解答

---

*本文档由 AI 自动生成和维护。如有问题或建议，请在 issue 中提出。*
