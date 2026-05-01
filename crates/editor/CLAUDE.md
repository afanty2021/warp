[根目录](../../CLAUDE.md) > **crates/editor**

# Warp Editor 模块

> 最后更新：2026年 5月 1日

## 模块职责

Warp Editor 是 Warp 的文本编辑功能模块，提供：

- **多行编辑**：支持多行文本编辑和操作
- **语法高亮**：基于语言的高亮显示
- **搜索和替换**：强大的搜索和替换功能
- **选择管理**：复杂的选择和光标管理
- **Vim 模式**：Vim 编辑器支持
- **差异显示**：代码差异可视化
- **性能优化**：大型文件的高效处理

## 入口与启动

### 主要入口点

- `src/lib.rs` - 库入口
- `src/editor.rs` - 核心编辑器实现
- `src/model.rs` - 编辑器模型
- `src/multiline.rs` - 多行编辑支持

### 核心组件

1. **Editor** - 主编辑器组件
2. **EditorModel** - 编辑器状态模型
3. **Content** - 内容管理
4. **Selection** - 选择系统
5. **Search** - 搜索功能

## 对外接口

### 核心 API

**编辑器模型**：
```rust
pub struct EditorModel {
    pub buffer: Buffer,
    pub selections: Vec<Selection>,
    pub cursor: Cursor,
}

impl EditorModel {
    pub fn new() -> Self;
    pub fn insert(&mut self, text: &str);
    pub fn delete(&mut self, range: Range);
    pub fn replace(&mut self, text: &str);
}
```

**缓冲区**：
```rust
pub struct Buffer {
    pub content: Rope,
    pub edits: EditTracker,
}

impl Buffer {
    pub fn len(&self) -> usize;
    pub fn line_count(&self) -> usize;
    pub fn line(&self, idx: usize) -> &str;
}
```

### 选择 API

```rust
pub struct Selection {
    pub start: Point,
    pub end: Point,
    pub mode: SelectionMode,
}

pub enum SelectionMode {
    Normal,
    Line,
    Block,
}
```

### 搜索 API

```rust
pub struct Search {
    pub query: String,
    pub case_sensitive: bool,
    pub regex: bool,
}

impl Search {
    pub fn find_next(&self, buffer: &Buffer) -> Option<Match>;
    pub fn find_all(&self, buffer: &Buffer) -> Vec<Match>;
    pub fn replace(&mut self, buffer: &mut Buffer, replacement: &str);
}
```

## 关键依赖与配置

### 依赖

- `sum_tree` - 增量数据结构
- `string-offset` - 字符串偏移管理
- `rayon` - 并行处理
- `regex` - 正则表达式
- `vim` - Vim 模式支持
- `markdown_parser` - Markdown 解析
- `mermaid_to_svg` - Mermaid 图表渲染

### 特性标志

- `test-util` - 测试工具

### 性能优化

- **增量更新**：使用 sum_tree 进行增量更新
- **并行处理**：使用 rayon 进行并行操作
- **高效存储**：使用 Rope 数据结构
- **缓存**：缓存计算结果

## 数据模型

### 缓冲区

```rust
pub struct Buffer {
    pub content: Rope,
    pub edits: EditTracker,
    pub history: History,
}
```

### 编辑

```rust
pub struct Edit {
    pub range: Range<usize>,
    pub new_text: String,
    pub old_text: String,
    pub timestamp: DateTime,
}
```

### 光标

```rust
pub struct Cursor {
    pub position: Point,
    pub anchor: Option<Point>,
    pub preferred_column: Option<usize>,
}
```

### 装饰

```rust
pub struct Decoration {
    pub range: Range<usize>,
    pub style: DecorationStyle,
}
```

## 测试与质量

### 单元测试

测试文件位置：
- `src/*_tests.rs`
- `src/*/mod_test.rs`

运行测试：
```bash
cargo nextest run -p warp_editor
```

### 基准测试

```bash
cargo bench -p warp_editor
```

### 测试覆盖

当前测试覆盖：
- ✅ 基本编辑操作
- ✅ 选择管理
- ✅ 搜索和替换
- ✅ Vim 模式
- ✅ 大文件处理
- ⚠️ 复杂编辑场景
- ⚠️ 性能回归测试

## 常见问题 (FAQ)

### Q: 如何处理大型文件？

A:
1. 使用 Rope 数据结构
2. 增量更新
3. 延迟加载
4. 虚拟化渲染

### Q: 如何实现语法高亮？

A: 使用语法树和增量高亮：
```rust
pub fn highlight(buffer: &Buffer, syntax: &Syntax) -> Vec<Decoration>;
```

### Q: Vim 模式如何工作？

A: Vim 模式在 `src/vim/` 中实现，提供：
- 正常/插入/可视模式
- 动作和文本对象
- 寄存器
- 宏

### Q: 如何优化编辑性能？

A:
1. 使用增量数据结构
2. 批量更新
3. 异步处理
4. 缓存策略

## 相关文件清单

### 核心文件

- `src/lib.rs` - 库入口
- `src/editor.rs` - 编辑器实现
- `src/model.rs` - 模型
- `src/content.rs` - 内容管理
- `src/multiline.rs` - 多行编辑

### 选择和搜索

- `src/selection.rs` - 选择系统
- `src/search.rs` - 搜索功能

### Vim 支持

- `src/vim/` - Vim 模式实现

### 渲染

- `src/render.rs` - 渲染逻辑
- `src/decoration.rs` - 装饰系统

### 测试

- `src/*_tests.rs` - 单元测试
- `benches/` - 基准测试

## 高级功能

### 多光标编辑

```rust
pub struct MultiCursor {
    pub cursors: Vec<Cursor>,
}

impl MultiCursor {
    pub fn add_cursor(&mut self, position: Point);
    pub fn apply_edit(&mut self, edit: &Edit);
}
```

### 代码折叠

```rust
pub struct Fold {
    pub range: Range<usize>,
    pub collapsed: bool,
}
```

### 自动补全

```rust
pub struct Completion {
    pub items: Vec<CompletionItem>,
    pub active_index: usize,
}
```

## 性能考虑

### 优化策略

1. **增量更新**：只更新变化的部分
2. **并行处理**：使用 rayon 进行并行操作
3. **延迟计算**：按需计算
4. **缓存**：缓存计算结果

### 性能指标

- 编辑延迟
- 渲染帧率
- 内存使用
- 文件加载时间

## 变更记录

### 2026-05-01

- 初始化 Editor 模块文档
- 记录核心功能和 API
- 添加性能优化信息
