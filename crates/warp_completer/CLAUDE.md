[根目录](../../CLAUDE.md) > [crates](../) > **warp_completer**

# warp_completer - 自动补全引擎

> 最后更新：2026年 5月 1日 星期五 22時40分12秒 CST

## 模块职责

`warp_completer` 是 Warp 的补全引擎，负责提供智能的命令行自动补全功能。它支持多种补全类型（命令、参数、路径、变量等）和两个版本实现（legacy 和 v2）。

## 入口与启动

### 主要入口

- **`src/lib.rs`** - 库入口
  - 导出核心补全功能
  - 定义 `ParsedCommandsSnapshot` 和 `ParsedTokensSnapshot`

### 核心数据结构

```rust
// 解析命令快照
pub struct ParsedCommandsSnapshot<'a, T: CompletionContext> {
    buffer_text: String,
    parsed_commands: Vec<parsers::LiteCommand>,
    completion_context: &'a T,
}

// 解析 token 快照
pub struct ParsedTokensSnapshot {
    pub buffer_text: String,
    pub parsed_tokens: Vec<ParsedTokenData>,
}

// 单个 token 数据
pub struct ParsedTokenData {
    pub token: meta::Spanned<String>,
    pub token_index: usize,
    pub token_description: Option<completer::Description>,
}
```

## 对外接口

### 核心模块

1. **`completer/`** - 补全引擎核心
   - **`mod.rs`** - 补全引擎定义
   - **`engine/`** - 补全引擎实现
     - `mod.rs` - 引擎入口
     - `legacy.rs` - 旧版引擎
     - `v2.rs` - v2 引擎
     - `command.rs` - 命令补全
     - `argument/` - 参数补全
       - `mod.rs`
       - `legacy.rs`
       - `v2.rs`
     - `flag/` - 标志补全
       - `mod.rs`
       - `legacy.rs`
       - `v2.rs`
     - `path.rs` - 路径补全
     - `variable.rs` - 变量补全
   - **`suggest/`** - 补全建议生成
     - `mod.rs` - 建议入口
     - `legacy.rs` - 旧版建议
     - `v2.rs` - v2 建议
     - `alias.rs` - 别名建议
     - `priority/` - 优先级处理
   - **`context/`** - 补全上下文
     - `mod.rs` - 上下文定义
     - `v2.rs` - v2 上下文
   - **`coalesce.rs`** - 结果合并
   - **`describe.rs`** - 补全描述
   - **`matchers.rs`** - 匹配器

2. **`parsers/`** - 命令行解析
   - **`mod.rs`** - 解析器入口
   - **`simple/`** - 简单解析器
     - `mod.rs`
     - `lexer.rs` - 词法分析
     - `parser.rs` - 语法分析
     - `token.rs` - token 定义
     - `convert.rs` - 转换
     - `iter.rs` - 迭代器
   - **`legacy.rs`** - 旧版解析器
   - **`v2.rs` - v2 解析器
   - **`hir/`** - 高级中间表示

3. **`signatures/`** - 命令签名
   - **`mod.rs`** - 签名入口
   - **`legacy/`** - 旧版签名
     - `mod.rs`
     - `registry.rs` - 签名注册表
   - **`v2/`** - v2 签名
     - `mod.rs`
     - `registry.rs` - v2 注册表
     - `js.rs` - JavaScript 签名
     - `lookup.rs` - 签名查找
   - **`clap.rs`** - Clap 集成
   - **`testing/`** - 测试工具

4. **`meta`** - 元数据处理
5. **`util`** - 工具函数

### 补全类型

- **命令补全** - 可用命令列表
- **参数补全** - 命令参数建议
- **标志补全** - 命令标志（选项）
- **路径补全** - 文件系统路径
- **变量补全** - 环境变量和 shell 变量

## 关键依赖与配置

### 依赖项

**核心依赖**:
- **`warp_core`** - 核心功能
- **`warp_cli`** - CLI 类型
- **`warpui`** - UI 集成
- **`warp_util`** - 工具函数
- **`fuzzy_match`** - 模糊匹配
- **`string-offset`** - 字符串偏移计算

**解析器**:
- **`regex`** - 正则表达式
- **`shellexpand`** - shell 扩展

**序列化**:
- **`serde`** - 序列化
- **`bincode`** - 二进制序列化

**数据结构**:
- **`smol_str`** - 小字符串优化
- **`itertools`** - 迭代器工具
- **`memo-map`** - 记忆化映射

**类型系统**:
- **`typed-path`** - 类型化路径
- **`getset`** - getter/setter 派生

**错误处理**:
- **`anyhow`** - 错误处理
- **`thiserror`** - 错误派生

**异步**:
- **`futures`** - 异步工具
- **`async-trait`** - 异步 trait
- **`async-recursion`** - 异步递归

**非 WASM 特定依赖** (`cfg(not(target_family = "wasm"))`):
- **`command`** - 命令执行
- **`warp-command-signatures`** - 命令签名（embed-signatures）
- **`rquickjs`** - QuickJS 引擎（可选，v2 功能）

**WASM 特定依赖** (`cfg(target_family = "wasm")`):
- **`warp-command-signatures`** - 命令签名（无 embed）

### 特性标志

- **`v2`** - 启用 v2 补全引擎（基于 JS）
  - 依赖：`warp_js`, `rquickjs`
  - 仅非 WASM 目标
- **`test-util`** - 测试工具支持
  - 启用测试相关的实用功能

## 数据模型

### 核心概念

1. **补全上下文（CompletionContext）**
   - 提供补全所需的环境信息
   - 包含当前状态、历史等

2. **补全建议（Suggestion）**
   - 可补全的候选项
   - 包含显示文本、插入文本、优先级等

3. **补全描述（Description）**
   - token 的类型和语义信息
   - 用于确定如何补全

4. **签名（Signature）**
   - 命令的参数和选项定义
   - 用于智能补全

### 解析流程

```
用户输入 → Lexer → Tokens → Parser → Commands → Completion Engine → Suggestions
```

## 测试与质量

### 测试文件

- **`completer/engine/test.rs`** - 引擎测试
- **`completer/engine/path_test.rs`** - 路径补全测试
- **`completer/tests.rs`** - 补全测试
- **`completer/describe_test.rs`** - 描述测试
- **`parsers/simple/lexer_test.rs`** - 词法分析测试
- **`parsers/simple/parser_test.rs`** - 语法分析测试
- **`parsers/test.rs`** - 解析器测试
- **`signatures/legacy/registry_test.rs`** - 旧版签名测试
- **`signatures/v2/lookup_test.rs`** - v2 查找测试
- **`signatures/v2/signatures_test.rs`** - v2 签名测试
- **`signatures/testing/legacy.rs`** - 旧版测试工具
- **`signatures/testing/v2.rs`** - v2 测试工具
- **`suggest/alias_test.rs`** - 别名测试
- **`suggest/priority/priority_test.rs`** - 优先级测试
- **`suggest/test.rs`** - 建议测试
- **`meta_test.rs`** - 元数据测试

### 测试策略

- 单元测试覆盖每个组件
- 集成测试覆盖端到端流程
- 使用 `test-util` 特性启用测试工具
- 测试文件命名：`*_test.rs` 或 `tests.rs`

### 代码质量

- 遵循 Rust 2021 edition 标准
- 模块化设计，职责分离
- 支持两个版本（legacy 和 v2）的并行开发

## 常见问题 (FAQ)

### Q1: Legacy 和 v2 引擎有什么区别？

**Legacy 引擎**:
- 纯 Rust 实现
- 基于预定义的命令签名
- 更稳定，但扩展性有限

**V2 引擎**:
- 基于 JavaScript（通过 QuickJS）
- 动态加载命令规范
- 更灵活，支持更多自定义

### Q2: 如何添加新的补全类型？

1. 在 `completer/engine/` 中实现补全逻辑
2. 在 `completer/suggest/` 中实现建议生成
3. 注册到补全引擎
4. 添加测试

### Q3: 如何集成新的命令签名？

**Legacy**:
```rust
// 在 signatures/legacy/registry.rs 中注册
register_command("mycommand", signature);
```

**V2**:
```rust
// 在 signatures/v2/js.rs 中添加 JS 规范
// 或通过动态加载
```

### Q4: 路径补全如何工作？

- 使用 `fuzzy_match` 进行模糊匹配
- 支持相对路径和绝对路径
- 考虑 shell 扩展（`~`、`$HOME` 等）
- 在 `completer/engine/path.rs` 中实现

### Q5: 如何调试补全问题？

1. 启用详细日志
2. 检查 `ParsedTokensSnapshot`
3. 验证 token 描述是否正确
4. 测试匹配器和建议生成器

## 相关文件清单

### 核心文件

- `Cargo.toml` - 包配置
- `src/lib.rs` - 库入口
- `src/meta.rs` - 元数据处理
- `src/util.rs` - 工具函数

### 补全引擎

- `src/completer/mod.rs` - 补全入口
- `src/completer/coalesce.rs` - 结果合并
- `src/completer/describe.rs` - 描述
- `src/completer/matchers.rs` - 匹配器
- `src/completer/context/mod.rs` - 上下文
- `src/completer/context/v2.rs` - v2 上下文
- `src/completer/testing/mod.rs` - 测试工具
- `src/completer/testing/v2.rs` - v2 测试工具

### 引擎实现

- `src/completer/engine/mod.rs` - 引擎入口
- `src/completer/engine/legacy.rs` - 旧版引擎
- `src/completer/engine/v2.rs` - v2 引擎
- `src/completer/engine/command.rs` - 命令补全
- `src/completer/engine/path.rs` - 路径补全
- `src/completer/engine/variable.rs` - 变量补全
- `src/completer/engine/argument/mod.rs` - 参数补全
- `src/completer/engine/argument/legacy.rs` - 旧版参数
- `src/completer/engine/argument/v2.rs` - v2 参数
- `src/completer/engine/flag/mod.rs` - 标志补全
- `src/completer/engine/flag/legacy.rs` - 旧版标志
- `src/completer/engine/flag/v2.rs` - v2 标志

### 建议生成

- `src/completer/suggest/mod.rs` - 建议入口
- `src/completer/suggest/legacy.rs` - 旧版建议
- `src/completer/suggest/v2.rs` - v2 建议
- `src/completer/suggest/alias.rs` - 别名建议
- `src/completer/suggest/priority/mod.rs` - 优先级
- `src/completer/suggest/priority/v2.rs` - v2 优先级

### 解析器

- `src/parsers/mod.rs` - 解析器入口
- `src/parsers/legacy.rs` - 旧版解析器
- `src/parsers/v2.rs` - v2 解析器
- `src/parsers/hir/mod.rs` - 高级中间表示
- `src/parsers/simple/mod.rs` - 简单解析器
- `src/parsers/simple/lexer.rs` - 词法分析
- `src/parsers/simple/parser.rs` - 语法分析
- `src/parsers/simple/token.rs` - token
- `src/parsers/simple/convert.rs` - 转换
- `src/parsers/simple/iter.rs` - 迭代器

### 签名

- `src/signatures/mod.rs` - 签名入口
- `src/signatures/clap.rs` - Clap 集成
- `src/signatures/legacy/mod.rs` - 旧版签名
- `src/signatures/legacy/registry.rs` - 旧版注册表
- `src/signatures/v2/mod.rs` - v2 签名
- `src/signatures/v2/registry.rs` - v2 注册表
- `src/signatures/v2/js.rs` - JS 签名
- `src/signatures/v2/lookup.rs` - 查找
- `src/signatures/testing/mod.rs` - 测试工具
- `src/signatures/testing/legacy.rs` - 旧版测试
- `src/signatures/testing/v2.rs` - v2 测试

### 测试文件

（见上文测试文件列表）

## 架构说明

### 设计原则

1. **模块化** - 每个组件职责单一
2. **可扩展** - 支持自定义补全类型
3. **向后兼容** - 保持 legacy 和 v2 并行
4. **性能优化** - 使用缓存和记忆化

### 补全流程

```
1. 解析输入（Lexer + Parser）
2. 构建 token 快照
3. 确定 token 类型（描述）
4. 选择合适的补全引擎
5. 生成补全建议
6. 排序和过滤
7. 返回结果
```

### 性能优化

- **缓存** - `ParsedTokensSnapshot` 缓存解析结果
- **记忆化** - `memo-map` 缓存计算结果
- **模糊匹配** - 快速字符串匹配
- **惰性求值** - 按需生成建议

## 变更记录

### 2026-05-01 (22:40)

- ✅ 初始化模块文档
- ✅ 记录核心架构和模块
- ✅ 添加使用示例
- ✅ 记录依赖和配置
- ✅ 添加常见问题解答

---

*本文档由 AI 自动生成和维护。如有问题或建议，请在 issue 中提出。*
