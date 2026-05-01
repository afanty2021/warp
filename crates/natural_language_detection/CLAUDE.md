# Natural Language Detection 模块

[根目录](../../CLRUDE.md) > [crates](../) > **natural_language_detection**

## 模块职责

提供自然语言检测功能，用于区分英语单词、Shell 命令和 StackOverflow 术语。支持词干提取、缩写展开和 Shell 语法检测。

## 入口与启动

- **主要入口**：`crates/natural_language_detection/src/lib.rs`
- **核心函数**：`natural_language_words_score()`、`is_word()`

## 对外接口

### 核心函数

```rust
// 计算自然语言分数
pub fn natural_language_words_score(
    words: Vec<Cow<str>>,
    is_first_token_command: bool
) -> usize;

// 检查单词是否在字典中
pub fn is_word(word: &str, db: WordDb) -> bool;

// 检查 token 是否包含 Shell 语法
pub fn check_if_token_has_shell_syntax(word: &str) -> bool;
```

### 单词数据库

```rust
pub enum WordDb {
    English,        // 英语词典
    StackOverflow,  // StackOverflow 术语
    Command,        // Shell 命令
}
```

### 功能特性

- **词干提取**：使用英语词干提取器（Porter 算法）
- **缩写展开**：自动展开常见缩写（he's → he, mustn't → must）
- **Shell 语法检测**：识别特殊字符（$、=、{、}、>、<、* 等）
- **保留字处理**：特殊处理 "what" 等保留字
- **引用处理**：忽略引号包裹的内容
- **分数计算**：自然语言 tokens - Shell 语法 tokens

## 关键依赖与配置

### 依赖项

- `rust-stemmers`：词干提取库
- `regex`：正则表达式（缩写匹配）
- `lazy_static`：编译时正则初始化

### 单词列表

- `WORD_LIST`：英语单词词典
- `STACK_OVERFLOW_LIST`：StackOverflow 术语
- `COMMAND_LIST`：Shell 命令列表

## 数据模型

### 预处理步骤

1. **转换为小写**
2. **展开缩写**：移除 's、're、n't、't、'm、've、'll
3. **词干提取**：提取词根形式

### 特殊字符列表

`$`, `=`, `{`, `}`, `[`, `]`, `>`, `<`, `*`, `~`, `&`, `(`, `)`, `|`, `/`, `-`

### 保留字

- "what"：特殊处理，避免误判

## 测试与质量

### 测试覆盖

- ⚠️ 无公开单元测试
- 依赖集成测试验证准确率

### 代码质量

- 高效的字符串处理
- 词干提取提高匹配率
- 清晰的评分逻辑

## 常见问题 (FAQ)

### Q: 如何处理命令作为第一个词？

A: 如果第一个词是命令，跳过它（用户可能想运行命令）。

### Q: 为什么使用词干提取？

A: 提高匹配率，如 "running" → "run"。

### Q: 如何处理引号包裹的词？

A: 引号内的词不检查 Shell 语法。

### Q: 分数如何计算？

A: 自然语言 tokens 数量减去包含 Shell 语法的 tokens 数量。

### Q: 准确率如何？

A: 在常见场景下效果良好，但不是完美的分类器。

## 相关文件清单

- `src/lib.rs`：主要实现
- `src/word_list.rs`：单词词典（生成文件）

## 变更记录

### 2026-05-01 (23:18)

- ✅ 初始化模块文档
- ✅ 添加检测算法说明
- ✅ 添加单词数据库和评分说明
- ✅ 添加预处理和特殊处理文档
