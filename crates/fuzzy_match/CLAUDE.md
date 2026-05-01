# Fuzzy Match - 模糊匹配模块

[根目录](../../CLAUDE.md) > [crates](../) > **fuzzy_match**

> 最后更新：2026-05-01

## 模块职责

Fuzzy Match 模块为 Warp 提供了模糊匹配和通配符模式匹配功能。该模块支持两种匹配模式：传统的模糊匹配（类似 Sublime Text 的 Cmd+P）和通配符模式匹配（类似 glob 模式），特别优化用于文件路径搜索。

**核心功能**：
- 传统模糊匹配（智能大小写）
- 通配符模式匹配（`*` 和 `?`）
- 优化的文件路径搜索
- 渐进式输入支持（如 `*.r` 匹配 `.rs`、`.rb` 等）
- 子字符串匹配（模式可以在路径的任何位置）

## 入口与启动

### 主要函数

**传统模糊匹配**：
- `match_indices(text, query)` - 智能大小写模糊匹配
- `match_indices_case_insensitive(text, query)` - 忽略大小写
- `match_indices_case_insensitive_ignore_spaces(text, query)` - 忽略空格

**通配符匹配**：
- `match_wildcard_pattern(text, pattern)` - 通配符模式匹配
- `match_wildcard_pattern_case_insensitive(text, pattern)` - 忽略大小写
- `contains_wildcards(query)` - 检查是否包含通配符

### 使用示例

```rust
use fuzzy_match::{match_indices, match_wildcard_pattern};

// 传统模糊匹配
let result = match_indices("hello_world", "hw"); // 匹配
assert!(result.is_some());

// 通配符匹配
let result = match_wildcard_pattern("button.rs", "*.rs"); // 匹配
assert!(result.is_some());

// 渐进式输入
let result = match_wildcard_pattern("test.js", "*.j"); // 匹配
assert!(result.is_some());
```

## 对外接口

### 核心数据结构

**`FuzzyMatchResult`**：
```rust
pub struct FuzzyMatchResult {
    pub score: i64,              // 匹配分数（越高越好）
    pub matched_indices: Vec<usize>, // 匹配字符的索引
}
```

### 模糊匹配接口

**`match_indices`**：
```rust
pub fn match_indices(text: &str, query: &str) -> Option<FuzzyMatchResult>
```
- 智能大小写：如果查询中没有大写字母，则忽略大小写
- 如果查询中有大写字母，则区分大小写

**`match_indices_case_insensitive`**：
```rust
pub fn match_indices_case_insensitive(text: &str, query: &str) -> Option<FuzzyMatchResult>
```
始终忽略大小写。

**`match_indices_case_insensitive_ignore_spaces`**：
```rust
pub fn match_indices_case_insensitive_ignore_spaces(
    text: &str,
    query: &str,
) -> Option<FuzzyMatchResult>
```
忽略查询中的空格，用于符号名称匹配。

### 通配符匹配接口

**`match_wildcard_pattern`**：
```rust
pub fn match_wildcard_pattern(text: &str, pattern: &str) -> Option<FuzzyMatchResult>
```
支持 `*`（零个或多个字符）和 `?`（一个字符）。

**`contains_wildcards`**：
```rust
pub fn contains_wildcards(query: &str) -> bool
```
检查查询是否包含通配符。

## 关键依赖与配置

### 依赖项

**核心依赖**：
- `fuzzy-matcher` (v0.3.7) - 底层模糊匹配算法（Skim Matcher V2）

### 配置

该模块无需外部配置，所有行为通过函数参数控制。

## 数据模型

### 匹配分数

**分数范围**：
- `2000` - 精确匹配（无通配符）
- `1000` - 完整通配符匹配
- `800` - 部分匹配（渐进式输入）
- `0` - 无匹配

### 匹配索引

`matched_indices` 包含匹配字符的**字符索引**（非字节索引），用于高亮显示匹配部分。

### 通配符语义

**`*`** - 匹配零个或多个字符
- `*.rs` 匹配 `button.rs`、`test.rs`
- `src/*` 匹配 `src/components`、`src/utils`

**`?`** - 匹配恰好一个字符
- `test?.rs` 匹配 `test1.rs`、`testa.rs`
- 不匹配 `test12.rs`

## 测试与质量

### 测试覆盖

✅ **单元测试**：33 个测试（在 `fuzzy_test.rs`）
- 模糊匹配测试
- 通配符匹配测试
- 边界情况测试
- 性能测试

### 测试策略

**运行测试**：
```bash
cargo test -p fuzzy_match
```

**测试重点**：
1. 匹配正确性
2. 边界情况（空字符串、特殊字符等）
3. 性能（大文本、复杂模式）
4. 渐进式输入

## 常见问题 (FAQ)

### Q: 什么时候使用模糊匹配 vs 通配符匹配？

**A**:
**模糊匹配**：
- 适合：符号搜索、文件名搜索（用户只记得部分字符）
- 示例：`hwr` 匹配 `hello_world.rs`

**通配符匹配**：
- 适合：文件扩展名、目录路径、已知模式的搜索
- 示例：`*.rs` 匹配所有 Rust 文件

### Q: 如何实现"先模糊匹配，后通配符匹配"？

**A**:
```rust
use fuzzy_match::{contains_wildcards, match_indices, match_wildcard_pattern};

fn smart_match(text: &str, query: &str) -> Option<FuzzyMatchResult> {
    if contains_wildcards(query) {
        match_wildcard_pattern(text, query)
    } else {
        match_indices(text, query)
    }
}
```

### Q: 为什么 `match_wildcard_pattern` 支持子字符串匹配？

**A**: 为了更好的文件搜索体验。用户可能输入 `ui/*.rs` 来匹配 `/src/ui/button.rs`，而不需要输入完整路径。

### Q: 如何优化大量文件的搜索性能？

**A**:
1. 使用 `contains_wildcards` 预筛选
2. 先用快速路径（简单前缀/后缀匹配）
3. 对候选结果再进行完整匹配
4. 考虑使用索引或缓存

### Q: 渐进式输入是如何工作的？

**A**:
```rust
// 用户输入 "*.r" 时
match_wildcard_pattern("test.rs", "*.r") // ✅ 匹配，分数 800
match_wildcard_pattern("test.rb", "*.r") // ✅ 匹配，分数 800

// 用户输入 "*.rs" 时
match_wildcard_pattern("test.rs", "*.rs") // ✅ 匹配，分数 1000
match_wildcard_pattern("test.rb", "*.rs") // ❌ 不匹配
```
部分匹配给出较低分数（800），完整匹配给出较高分数（1000）。

### Q: 如何处理大小写敏感性？

**A**:
```rust
// 智能大小写（默认）
match_indices("HelloWorld", "hw")   // ❌ 不匹配（查询有小写）
match_indices("HelloWorld", "HW")   // ✅ 匹配（查询全大写）
match_indices("hello_world", "hw")  // ✅ 匹配（查询全小写，忽略大小写）

// 强制忽略大小写
match_indices_case_insensitive("HelloWorld", "hw") // ✅ 匹配
```

### Q: 匹配索引如何用于高亮显示？

**A**:
```rust
let result = match_indices("hello_world", "hw").unwrap();
for idx in result.matched_indices {
    // idx 是字符索引，不是字节索引
    let ch = "hello_world".chars().nth(idx).unwrap();
    println!("Matched at {}: {}", idx, ch);
}
// 输出：
// Matched at 0: h
// Matched at 6: w
```

## 相关文件清单

### 核心文件

- `src/lib.rs` - 模块入口，所有匹配函数实现

### 测试文件

- `src/fuzzy_test.rs` - 单元测试

## 变更记录

### 2026-05-01

- ✅ 创建模块文档
- ✅ 记录两种匹配模式的差异
- ✅ 文档化渐进式输入特性
- ✅ 添加性能优化建议

---

*本文档由 AI 自动生成和维护。如有问题或建议，请在 issue 中提出。*
