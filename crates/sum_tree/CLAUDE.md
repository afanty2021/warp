# Sum Tree 模块

[根目录](../../CLRUDE.md) > [crates](../) > **sum_tree**

## 模块职责

提供基于 B 树的持久化数据结构，支持高效的范围查询、求和和更新操作。用于编辑器缓冲区、文本布局等场景。

## 入口与启动

- **主要入口**：`crates/sum_tree/src/lib.rs`
- **核心类型**：`SumTree<T>`
- **游标类型**：`Cursor`、`FilterCursor`

## 对外接口

### 核心 Traits

```rust
pub trait Item: Clone + fmt::Debug {
    type Summary: for<'a> AddAssign<&'a Self::Summary> + Default + Clone + fmt::Debug;
    fn summary(&self) -> Self::Summary;
}

pub trait KeyedItem: Item {
    type Key: for<'a> Dimension<'a, Self::Summary> + Ord;
    fn key(&self) -> Self::Key;
}

pub trait Dimension<'a, Summary: Default>: 'a + Clone + fmt::Debug + Default {
    fn add_summary(&mut self, summary: &'a Summary);
}
```

### 主要类型

```rust
pub struct SumTree<T: Item>(Arc<Node<T>>);

pub enum Node<T: Item> {
    Internal {
        height: u8,
        summary: T::Summary,
        child_summaries: ArrayVec<T::Summary, { 2 * TREE_BASE }>,
        child_trees: ArrayVec<SumTree<T>, { 2 * TREE_BASE }>,
    },
    Leaf {
        summary: T::Summary,
        items: ArrayVec<T, { 2 * TREE_BASE }>,
        item_summaries: ArrayVec<T::Summary, { 2 * TREE_BASE }>,
    },
}
```

### 主要方法

```rust
impl<T: Item> SumTree<T> {
    pub fn new() -> Self;
    pub fn from_item(item: T) -> Self;
    pub fn push(&mut self, item: T);
    pub fn extend<I>(&mut self, iter: I) where I: IntoIterator<Item = T>;
    pub fn is_empty(&self) -> bool;
    pub fn summary(&self) -> T::Summary;
    pub fn update_last(&mut self, f: impl FnOnce(&mut T));

    // 游标操作
    pub fn cursor<'a, S, U>(&'a self) -> Cursor<'a, T, S, U>;
    pub fn filter<'a, F, U>(&'a self, filter_node: F) -> FilterCursor<'a, F, T, U>;
}

impl<T: KeyedItem> SumTree<T> {
    pub fn insert(&mut self, item: T);
    pub fn edit(&mut self, edits: &mut [Edit<T>]);
}
```

### 功能特性

- **持久化**：使用 `Arc` 共享节点，不可变更新
- **平衡树**：自动平衡，保证 O(log n) 操作
- **范围查询**：通过游标高效遍历
- **增量求和**：每个节点存储子树摘要
- **过滤游标**：基于摘要条件过滤
- **批量编辑**：高效的批量插入/删除

## 关键依赖与配置

### 依赖项

- `arrayvec`：栈分配的向量
- 详见各子模块

### 配置要求

- `TREE_BASE`：测试环境 2，生产环境 6

## 数据模型

### 树结构

- **内部节点**：最多 2 * TREE_BASE 个子节点
- **叶子节点**：最多 2 * TREE_BASE 个项目
- **高度**：内部节点的高度，叶子节点为 0

### 摘要计算

每个节点存储子树摘要，支持：
- 总和计算
- 范围查询
- 条件过滤

### 编辑操作

```rust
pub enum Edit<T: KeyedItem> {
    Insert(T),
    Remove(T),
}
```

## 测试与质量

### 测试覆盖

- ✅ 单元测试：`src/lib_test.rs`
- 测试内容：
  - 基本操作（push、extend）
  - 游标操作
  - 编辑操作
  - 边界情况

### 代码质量

- 泛型设计
- 不可变数据结构
- 高效的摘要计算

## 常见问题 (FAQ)

### Q: 与 B 树有何不同？

A: 不可变更新，使用 `Arc` 共享节点。

### Q: 如何实现范围查询？

A: 使用游标 API，基于摘要导航。

### Q: 性能特点？

A: 插入 O(log n)，范围查询 O(log n + k)。

### Q: 内存占用？

A: 每个节点存储摘要，适度增加。

### Q: 适合什么场景？

A: 编辑器缓冲区、文本布局、日志存储。

### Q: 如何定义自定义 Item？

A: 实现 `Item` trait，定义 `Summary` 类型。

## 相关文件清单

- `src/lib.rs`：主要实现
- `src/cursor.rs`：游标实现
- `src/lib_test.rs`：单元测试

## 变更记录

### 2026-05-01 (23:18)

- ✅ 初始化模块文档
- ✅ 添加数据结构和 API 说明
- ✅ 添加游标和编辑文档
