[根目录](../../CLAUDE.md) > **crates/markdown_parser**

# Warp Markdown Parser 模块

> 最后更新：2026年 5月 1日

## 模块职责

Warp Markdown Parser 模块提供 Markdown 和 HTML 解析功能，使 Warp 能够：

- **Markdown 解析**：完整的 CommonMark 和 GFM 支持
- **HTML 解析**：HTML 片段解析
- **格式化文本**：富文本格式化
- **增量更新**：高效的增量文本更新
- **代码块**：语法高亮的代码块
- **表格**：GFM 表格支持
- **超链接**：可点击的链接
- **图片**：图片渲染

## 入口与启动

### 主要入口点

- `src/lib.rs` - 库入口，导出所有公共 API
- `src/markdown_parser.rs` - Markdown 解析实现
- `src/html_parser.rs` - HTML 解析实现
- `src/weight.rs` - 字体权重处理

### 核心组件

1. **MarkdownParser** - Markdown 解析器
2. **HtmlParser** - HTML 解析器
3. **FormattedText** - 格式化文本
4. **FormattedTextLine** - 格式化文本行
5. **FormattedTextDelta** - 增量更新

## 对外接口

### 核心 API

**Markdown 解析**：
```rust
pub fn parse_markdown(
    input: &str,
) -> FormattedText;

pub fn parse_markdown_with_gfm_tables(
    input: &str,
) -> FormattedText;

pub fn parse_inline_markdown(
    input: &str,
) -> FormattedText;
```

**HTML 解析**：
```rust
pub fn parse_html(
    input: &str,
) -> FormattedText;
```

**图片解析**：
```rust
pub fn parse_image_prefix(
    input: &str,
) -> Option<(ImagePrefix, usize)>;

pub fn parse_image_run_line(
    input: &str,
) -> Option<Formatted<InlineImage>>;
```

### 格式化文本

```rust
pub struct FormattedText {
    pub lines: VecDeque<FormattedTextLine>,
}

pub enum FormattedTextLine {
    Text(Vec<Formatted<InlineStyle>>),
    CodeBlock(CodeBlock),
    Image(Image),
    Table(Table),
}
```

### 增量更新

```rust
pub struct FormattedTextDelta {
    pub common_prefix_lines: usize,
    pub old_suffix_formatted_text_lines: usize,
    pub new_suffix: VecDeque<FormattedTextLine>,
}

pub fn compute_formatted_text_delta(
    old: FormattedText,
    new: FormattedText,
) -> FormattedTextDelta;
```

## 关键依赖与配置

### 依赖

- `nom` - 解析器组合子
- `html5ever` - HTML 解析
- `markup5ever_rcdom` - HTML DOM
- `itertools` - 迭代器工具
- `serde_yaml` - YAML 解析（代码块元数据）
- `thiserror` - 错误处理

### 特性标志

无特殊特性标志。

## 数据模型

### 内联样式

```rust
pub struct InlineStyle {
    pub text: String,
    pub attributes: Vec<Attribute>,
}

pub enum Attribute {
    Bold,
    Italic,
    Code,
    Link(String),
    Color(Color),
    BackgroundColor(Color),
}
```

### 代码块

```rust
pub struct CodeBlock {
    pub language: String,
    pub code: String,
    pub metadata: Option<CodeBlockMetadata>,
}
```

### 图片

```rust
pub struct InlineImage {
    pub src: String,
    pub alt: String,
    pub title: Option<String>,
}
```

### 表格

```rust
pub struct Table {
    pub headers: Vec<TableCell>,
    pub rows: Vec<Vec<TableCell>>,
}
```

## 测试与质量

### 单元测试

测试文件位置：
- `src/markdown_parser_test.rs` - Markdown 解析测试
- `src/html_parser_test.rs` - HTML 解析测试

运行测试：
```bash
cargo nextest run -p markdown_parser
```

### 测试覆盖

当前测试覆盖：
- ✅ 基本 Markdown 语法
- ✅ GFM 扩展
- ✅ 代码块
- ✅ 表格
- ✅ 链接和图片
- ✅ HTML 片段
- ⚠️ 复杂嵌套
- ⚠️ 边界情况

## 常见问题 (FAQ)

### Q: 如何解析 Markdown？

A: 使用 `parse_markdown`：
```rust
let formatted = parse_markdown("# Hello World\n\nThis is **bold**.");
```

### Q: 如何解析 GFM 表格？

A: 使用 `parse_markdown_with_gfm_tables`：
```rust
let formatted = parse_markdown_with_gfm_tables(
    "| Header 1 | Header 2 |\n|----------|----------|\n| Cell 1   | Cell 2   |"
);
```

### Q: 如何处理增量更新？

A: 使用 `compute_formatted_text_delta`：
```rust
let delta = compute_formatted_text_delta(old, new);
if !delta.is_noop() {
    // 应用增量更新
}
```

### Q: 如何解析代码块元数据？

A: 代码块第一行可以是 YAML 元数据：
``````markdown
```python path=/path/to/file.py start=1
print("Hello")
```
``````

## 相关文件清单

### 核心文件

- `src/lib.rs` - 库入口
- `src/markdown_parser.rs` - Markdown 解析
- `src/html_parser.rs` - HTML 解析
- `src/weight.rs` - 字体权重

### 测试

- `src/markdown_parser_test.rs`
- `src/html_parser_test.rs`

## 高级功能

### GFM 扩展

支持 GitHub Flavored Markdown 扩展：
- 表格
- 删除线
- 任务列表
- 自动链接

### 代码块元数据

支持代码块元数据：
``````markdown
```rust path=/path/to/file.rs start=10
fn main() {
    println!("Hello");
}
```
``````

### 自定义渲染

支持自定义渲染属性：
```rust
pub enum Attribute {
    Color(Color),
    BackgroundColor(Color),
    Link(String),
    // ... 更多属性
}
```

## 性能考虑

### 优化策略

1. **增量解析**：只重新解析变化的部分
2. **缓存结果**：缓存解析结果
3. **延迟计算**：按需计算
4. **高效迭代**：使用迭代器

### 性能指标

- 解析时间
- 内存使用
- 增量更新效率

## 集成点

### 与其他模块的集成

- **editor** - 编辑器集成
- **markdown_parser** - Markdown 渲染
- **warpui** - UI 渲染

### 使用场景

1. **笔记本** - Markdown 笔记本
2. **文档** - 文档查看
3. **README** - README 渲染
4. **注释** - 代码注释

## 支持的语法

### Markdown

- 标题（`#`）
- 粗体（`**bold**`）
- 斜体（`*italic*`）
- 代码（`` `code` ``）
- 链接（`[text](url)`）
- 图片（`![alt](url)`）
- 列表（`-`、`1.`）
- 引用（`>`）
- 代码块（``` ``` ```）

### GFM 扩展

- 表格
- 删除线（`~~strike~~`）
- 任务列表（`- [ ]`）
- 自动链接

### HTML

- 内联 HTML
- HTML 块
- HTML 注释

## 错误处理

### 错误类型

```rust
pub enum Error {
    InvalidMarkdown,
    InvalidHtml,
    ParseError,
}
```

### 错误恢复

解析错误不会阻止渲染：
```rust
// 即使有语法错误，仍然尝试渲染
let formatted = parse_markdown(input)?;
```

## 最佳实践

### 使用建议

1. **增量更新** - 优先使用增量更新
2. **缓存管理** - 缓存解析结果
3. **错误处理** - 优雅处理语法错误
4. **性能监控** - 监控解析时间

### 常见模式

```rust
// 解析 Markdown
let formatted = parse_markdown("# Title\n\nContent");

// 增量更新
let delta = compute_formatted_text_delta(old, new);

// 渲染
for line in formatted.lines {
    render_line(line);
}
```

## 变更记录

### 2026-05-01

- 初始化 Markdown Parser 模块文档
- 记录核心功能和 API
- 添加 GFM 支持信息
- 记录性能优化
