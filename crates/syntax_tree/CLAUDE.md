[根目录](../../CLAUDE.md) > **crates/syntax_tree**

# Warp Syntax Tree 模块

> 最后更新：2026年 5月 1日

## 模块职责

Warp Syntax Tree 模块提供语法树解析和装饰功能，使 Warp 能够：

- **语法高亮**：基于 tree-sitter 的语法高亮
- **缩进计算**：自动计算代码缩进
- **装饰层**：管理编辑器装饰
- **多语言支持**：支持 40+ 种编程语言
- **增量更新**：高效的增量语法树更新
- **缓存优化**：智能缓存高亮结果

## 入口与启动

### 主要入口点

- `src/lib.rs` - 库入口，包含核心实现
- `src/queries/mod.rs` - 查询模块
- `src/queries/highlight_query.rs` - 高亮查询
- `src/queries/indent_query.rs` - 缩进查询

### 核心组件

1. **DecorationState** - 装饰状态管理
2. **HighlightQuery** - 高亮查询
3. **IndentQuery** - 缩进查询
4. **LanguageQueries** - 语言查询集合
5. **HighlightCache** - 高亮缓存

## 对外接口

### 核心 API

**装饰状态**：
```rust
pub struct DecorationState {
    pub decoration_layer: DecorationLayer,
    pub language_id: Option<arborium::tree_sitter::Language>,
}

impl DecorationState {
    pub fn new() -> Self;
    pub fn set_language(&mut self, language: &Language);
    pub fn sync_decorations(&mut self, buffer: &Buffer);
}
```

**高亮查询**：
```rust
pub struct HighlightQuery {
    pub query: tree_sitter::Query,
    pub color_map: ColorMap,
}

impl HighlightQuery {
    pub fn new(language: &Language) -> Result<Self>;
    pub fn highlights_for_range(
        &self,
        tree: &Tree,
        range: Range<ByteOffset>,
    ) -> Vec<(Range<ByteOffset>, ColorU)>;
}
```

**缩进查询**：
```rust
pub fn indentation_delta(
    tree: &Tree,
    line: usize,
    indent_unit: &IndentUnit,
) -> IndentDelta;
```

### 事件处理

```rust
pub enum DecorationStateEvent {
    DecorationUpdated { version: BufferVersion },
}
```

## 关键依赖与配置

### 依赖

- `arborium` - tree-sitter 绑定
- `languages` - 语言定义
- `warp_editor` - 编辑器集成
- `warpui` - UI 框架
- `rangemap` - 范围映射
- `parking_lot` - 高性能锁
- `futures` - 异步支持

### 特性标志

- **test-util** - 测试工具

### 配置

最大语法树缓存：
```rust
const MAX_SYNTAX_TREES: usize = 3;
```

## 数据模型

### 语言查询

```rust
struct LanguageQueries {
    language: Arc<Language>,
    syntax_query: HighlightQuery,
}
```

### 高亮缓存

```rust
struct HighlightCache {
    key: HighlightCacheKey,
    highlights: RangeMap<CharOffset, ColorU>,
}

struct HighlightCacheKey {
    version: BufferVersion,
    ranges: RangeSet<CharOffset>,
    language_id: Option<arborium::tree_sitter::Language>,
}
```

### 缩进增量

```rust
pub struct IndentDelta {
    pub delta: i32,
    pub is_indent_guide: bool,
}
```

## 测试与质量

### 单元测试

测试文件位置：
- `src/queries/indent_query_tests.rs` - 缩进查询测试

运行测试：
```bash
cargo nextest run -p syntax_tree
```

### 测试覆盖

当前测试覆盖：
- ✅ 缩进计算
- ✅ 语言查询
- ⚠️ 高亮查询
- ⚠️ 增量更新
- ⚠️ 缓存失效

## 常见问题 (FAQ)

### Q: 如何设置语法高亮？

A: 创建 `DecorationState` 并设置语言：
```rust
let mut state = DecorationState::new();
state.set_language(&Language::Rust);
state.sync_decorations(&buffer);
```

### Q: 如何计算缩进？

A: 使用 `indentation_delta`：
```rust
let delta = indentation_delta(&tree, line, &IndentUnit::Spaces(4));
let new_indent = current_indent + delta.delta;
```

### Q: 如何优化性能？

A: 使用缓存和增量更新：
```rust
// 只更新变化的部分
state.sync_decorations(&buffer);
```

### Q: 支持哪些语言？

A: 支持 40+ 种语言，包括：
- Rust, C, C++, Go, Java
- Python, JavaScript, TypeScript
- HTML, CSS, JSON, YAML
- 等等...

## 相关文件清单

### 核心文件

- `src/lib.rs` - 库入口和核心实现
- `src/queries/mod.rs` - 查询模块
- `src/queries/highlight_query.rs` - 高亮查询
- `src/queries/indent_query.rs` - 缩进查询

### 测试

- `src/queries/indent_query_tests.rs`

## 高级功能

### 增量更新

使用增量更新优化性能：
```rust
// 只重新解析变化的部分
let edit = InputEdit {
    start_byte: old_range.start,
    old_end_byte: old_range.end,
    new_end_byte: new_range.end,
    start_position: old_start_point,
    old_end_position: old_end_point,
    new_end_position: new_end_point,
};
tree.edit(&edit);
```

### 缓存策略

单条目缓存：
```rust
struct HighlightCache {
    key: HighlightCacheKey,
    highlights: RangeMap<CharOffset, ColorU>,
}
```

缓存匹配检查：
```rust
impl HighlightCacheKey {
    fn matches(
        &self,
        version: BufferVersion,
        ranges: &RangeSet<CharOffset>,
        language_id: &Option<arborium::tree_sitter::Language>,
    ) -> bool;
}
```

### 多线程

使用线程本地存储：
```rust
thread_local! {
    static PARSER: RefCell<Parser> = RefCell::new(Parser::new());
}
```

## 性能考虑

### 优化策略

1. **增量解析**：只重新解析变化的部分
2. **缓存结果**：缓存高亮计算
3. **延迟计算**：按需计算装饰
4. **范围限制**：只计算可见范围

### 性能指标

- 解析时间
- 高亮计算时间
- 内存使用
- 缓存命中率

## 平台支持

### 支持的平台

- ✅ macOS
- ✅ Linux
- ✅ Windows
- ✅ WASM（有限支持）

### WASM 限制

- tree-sitter 在 WASM 上有限制
- 某些语言可能不可用

## 集成点

### 与其他模块的集成

- **editor** - 编辑器集成
- **languages** - 语言定义
- **warpui** - UI 渲染

### 使用场景

1. **语法高亮** - 代码高亮显示
2. **自动缩进** - 智能缩进
3. **代码折叠** - 基于语法的折叠
4. **代码导航** - 跳转到定义

## 高亮颜色

### 颜色映射

```rust
pub struct ColorMap {
    pub colors: HashMap<String, ColorU>,
}
```

### 颜色主题

支持自定义颜色主题：
- 语法高亮颜色
- 背景色
- 前景色

## 缩进计算

### 缩进规则

```rust
pub fn indentation_delta(
    tree: &Tree,
    line: usize,
    indent_unit: &IndentUnit,
) -> IndentDelta {
    // 计算相对于前一行的缩进变化
}
```

### 缩进单位

```rust
pub enum IndentUnit {
    Spaces(usize),
    Tabs,
}
```

## 错误处理

### 错误类型

```rust
pub enum Error {
    InvalidLanguage,
    ParseError,
    QueryError,
}
```

### 错误恢复

语法错误不会阻止高亮：
```rust
// 即使有语法错误，仍然尝试高亮
let highlights = query.highlights_for_range(&tree, range)?;
```

## 最佳实践

### 使用建议

1. **语言检测** - 自动检测文件语言
2. **缓存管理** - 定期清理缓存
3. **性能监控** - 监控解析时间
4. **错误处理** - 优雅处理语法错误

### 常见模式

```rust
// 设置语法高亮
let mut state = DecorationState::new();
state.set_language(&Language::Rust);

// 同步装饰
state.sync_decorations(&buffer);

// 计算缩进
let delta = indentation_delta(&tree, line, &IndentUnit::Spaces(4));
```

## 变更记录

### 2026-05-01

- 初始化 Syntax Tree 模块文档
- 记录核心功能和 API
- 添加性能优化信息
- 记录语言支持
