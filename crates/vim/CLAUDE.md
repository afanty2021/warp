[根目录](../../CLAUDE.md) > **crates/vim**

# Warp Vim 模块

> 最后更新：2026年 5月 1日

## 模块职责

Warp Vim 模块提供完整的 Vim 编辑器模式支持，使 Warp 能够：

- **Vim 键绑定**：完整的 Vim 键盘绑定支持
- **文本对象**：Vim 文本对象（单词、段落、引号、块等）
- **动作命令**：Vim 动作（删除、修改、复制等）
- **寄存器**：Vim 寄存器系统
- **搜索和跳转**：字符搜索、括号匹配
- **光标移动**：基于单词、段落的光标移动

## 入口与启动

### 主要入口点

- `src/lib.rs` - 库入口，导出所有公共 API
- `src/vim.rs` - Vim 模式实现

### 核心组件

1. **文本对象** - Vim 文本对象（`src/text_objects/`）
2. **匹配括号** - 括号匹配（`src/matching_brackets.rs`）
3. **段落迭代** - 段落导航（`src/paragraph_iterator.rs`）
4. **单词迭代** - 单词导航（`src/word_iterator.rs`）
5. **字符查找** - 字符查找（`src/find_char.rs`）
6. **寄存器** - Vim 寄存器（`src/register.rs`）

## 对外接口

### 核心 API

**括号匹配**：
```rust
pub fn vim_find_matching_bracket(
    text: &str,
    offset: usize,
    open: char,
    close: char,
) -> Option<usize>;
```

**段落导航**：
```rust
pub fn find_next_paragraph_end(text: &str, offset: usize) -> Option<usize>;
pub fn find_previous_paragraph_start(text: &str, offset: usize) -> Option<usize>;
```

**单词迭代**：
```rust
pub fn vim_word_iterator_from_offset(
    text: &str,
    offset: usize,
) -> impl Iterator<Item = (usize, usize)>;
```

**字符查找**：
```rust
pub fn vim_find_char_on_line(
    line: &str,
    offset: usize,
    c: char,
    count: usize,
    backward: bool,
) -> Option<usize>;
```

### 文本对象

**单词对象**：
```rust
pub fn vim_word_object(
    text: &str,
    offset: usize,
    inner: bool,
) -> Option<(usize, usize)>;
```

**段落对象**：
```rust
pub fn vim_paragraph_object(
    text: &str,
    offset: usize,
    inner: bool,
) -> Option<(usize, usize)>;
```

**引号对象**：
```rust
pub fn vim_quote_object(
    text: &str,
    offset: usize,
    quote_char: char,
    inner: bool,
) -> Option<(usize, usize)>;
```

**块对象**：
```rust
pub fn vim_block_object(
    text: &str,
    offset: usize,
    open: char,
    close: char,
    inner: bool,
) -> Option<(usize, usize)>;
```

## 关键依赖与配置

### 依赖

- `warp_core` - Warp 核心功能
- `warpui` - Warp UI 框架
- `string-offset` - 字符串偏移管理
- `itertools` - 迭代器工具
- `unindent` - 文本缩进处理

### 特性标志

无特殊特性标志。

## 数据模型

### 寄存器

```rust
pub struct Register {
    pub content: String,
    pub register_type: RegisterType,
}

pub enum RegisterType {
    Character,
    Line,
    Block,
}
```

### 文本对象范围

```rust
pub struct TextObjectRange {
    pub start: usize,
    pub end: usize,
    pub inner: bool,
}
```

## 测试与质量

### 单元测试

测试文件位置：
- `src/matching_brackets_tests.rs` - 括号匹配测试
- `src/paragraph_iterator_tests.rs` - 段落迭代测试
- `src/word_iterator_tests.rs` - 单词迭代测试
- `src/text_objects/word_tests.rs` - 单词对象测试
- `src/text_objects/paragraph_tests.rs` - 段落对象测试
- `src/text_objects/quote_tests.rs` - 引号对象测试
- `src/text_objects/block_tests.rs` - 块对象测试

运行测试：
```bash
cargo nextest run -p vim
```

### 测试覆盖

当前测试覆盖：
- ✅ 括号匹配
- ✅ 段落导航
- ✅ 单词迭代
- ✅ 单词对象
- ✅ 段落对象
- ✅ 引号对象
- ✅ 块对象
- ⚠️ 复杂场景
- ⚠️ 边界情况

## 常见问题 (FAQ)

### Q: 如何匹配括号？

A: 使用 `vim_find_matching_bracket`：
```rust
if let Some(match_pos) = vim_find_matching_bracket(text, offset, '(', ')') {
    println!("Matching bracket at: {}", match_pos);
}
```

### Q: 如何选择单词对象？

A: 使用 `vim_word_object`：
```rust
if let Some((start, end)) = vim_word_object(text, offset, true) {
    let word = &text[start..end];
    println!("Word: {}", word);
}
```

### Q: 如何在行中查找字符？

A: 使用 `vim_find_char_on_line`：
```rust
if let Some(pos) = vim_find_char_on_line(line, offset, 'a', 1, false) {
    println!("Found 'a' at: {}", pos);
}
```

### Q: inner 和 outer 对象有什么区别？

A:
- **inner**：只选择内容，不包含定界符
- **outer**：包含内容和定界符

例如：`"hello world"`
- inner 对象：`hello world`
- outer 对象：`"hello world"`

## 相关文件清单

### 核心文件

- `src/lib.rs` - 库入口
- `src/vim.rs` - Vim 模式实现
- `src/register.rs` - 寄存器系统

### 导航功能

- `src/matching_brackets.rs` - 括号匹配
- `src/paragraph_iterator.rs` - 段落导航
- `src/word_iterator.rs` - 单词导航
- `src/find_char.rs` - 字符查找

### 文本对象

- `src/text_objects/mod.rs` - 文本对象模块
- `src/text_objects/word.rs` - 单词对象
- `src/text_objects/paragraph.rs` - 段落对象
- `src/text_objects/quote.rs` - 引号对象
- `src/text_objects/block.rs` - 块对象

### 测试

- `src/matching_brackets_tests.rs`
- `src/paragraph_iterator_tests.rs`
- `src/word_iterator_tests.rs`
- `src/text_objects/*_tests.rs`

## 高级功能

### 文本对象组合

支持文本对象的组合使用：
```rust
// 结合单词对象和引号对象
let inner_quote = vim_quote_object(text, offset, '"', true)?;
let outer_word = vim_word_object(text, inner_quote.start, false)?;
```

### 寄存器操作

支持多个寄存器：
- 无名寄存器（`"`）
- 删除寄存器（`"`）
- 复制寄存器（`0`）
- 命名寄存器（`a`-`z`）
- 系统寄存器（`+`、`*`）

### 多重复命令

支持计数前缀：
```rust
// 删除 3 个单词
vim_delete_word_object(text, offset, 3)?;
```

## 性能考虑

### 优化策略

1. **增量解析**：只解析需要部分
2. **缓存结果**：缓存常用操作
3. **高效迭代**：使用迭代器而非分配

### 性能指标

- 查找延迟
- 大文件处理
- 内存使用

## Vim 兼容性

### 支持的功能

- ✅ 基本移动（h、j、k、l）
- ✅ 单词移动（w、b、e）
- ✅ 文本对象（ciw、da"、yab）
- ✅ 搜索（f、F、t、T）
- ✅ 括号匹配（%、[({ 等）
- ✅ 寄存器（""、"a-"z）
- ⚠️ 宏（部分支持）
- ⚠️ 标记（部分支持）

### 不支持的功能

- ❌ Vim 脚本
- ❌ 插件系统
- ❌ 窗口管理
- ❌ 标签页

## 集成点

### 与其他模块的集成

- **editor** - 编辑器集成
- **warpui** - UI 事件处理
- **warp_terminal** - 终端 Vim 模式

### 使用场景

1. **终端编辑** - 在终端中使用 Vim 模式
2. **代码编辑** - 编辑代码时使用 Vim 键绑定
3. **快速导航** - 使用 Vim 动作快速移动

## 最佳实践

### 使用建议

1. **学习曲线** - Vim 模式有学习曲线，建议从基本功能开始
2. **自定义** - 可以自定义键绑定
3. **混合模式** - 可以与其他编辑模式混合使用

### 常见模式

```rust
// 删除单词（diw）
let range = vim_word_object(text, offset, true)?;
delete_range(range)?;

// 修改引号内内容（ci"）
let range = vim_quote_object(text, offset, '"', true)?;
replace_range(range, new_text)?;

// 复制整个段落（yap）
let range = vim_paragraph_object(text, offset, false)?;
copy_to_register(range, '"')?;
```

## 变更记录

### 2026-05-01

- 初始化 Vim 模块文档
- 记录核心功能和 API
- 添加文本对象文档
- 记录 Vim 兼容性
