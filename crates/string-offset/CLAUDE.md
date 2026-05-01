# String Offset 模块

[根目录](../../CLRUDE.md) > [crates](../) > **string-offset**

## 模块职责

提供基于偏移量的文本位置表示，支持字符偏移（CharOffset）和字节偏移（ByteOffset）。用于文本编辑和导航。

## 入口与启动

- **主要入口**：`crates/string-offset/src/lib.rs`
- **核心类型**：`CharOffset`、`ByteOffset`

## 对外接口

### 核心类型

```rust
#[derive(Clone, Copy, Debug, Default, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct CharOffset(usize);

#[derive(Clone, Copy, Debug, Default, Hash, Eq, Ord, PartialEq, PartialOrd)]
pub struct ByteOffset(usize);
```

### 主要方法

```rust
impl CharOffset {
    pub const fn zero() -> Self;
    pub fn as_usize(self) -> usize;
    pub fn empty_range(self) -> Range<Self>;
    pub fn add_signed(self, rhs: isize) -> Self;  // Debug 下 panic 溢出
    pub fn range(range: Range<usize>) -> Range<Self>;
}

impl ByteOffset {
    // 相同的方法签名
}
```

### 运算符支持

```rust
// 加法
impl Add<Self> for CharOffset;
impl Add<usize> for CharOffset;
impl AddAssign for CharOffset;
impl AddAssign<usize> for CharOffset;

// 减法
impl Sub<Self> for CharOffset;
impl Sub<usize> for CharOffset;
impl SubAssign for CharOffset;
impl SubAssign<usize> for CharOffset;

// 饱和减法
impl SaturatingSub for CharOffset;
```

### 字符计数器

```rust
pub struct CharCounter<'a> {
    current_offset: CharOffset,
    char_indices: std::str::CharIndices<'a>,
}

impl<'a> CharCounter<'a> {
    pub fn new(str: &'a str) -> Self;
    pub fn char_offset(&mut self, byte_offset: impl Into<ByteOffset>) -> Option<CharOffset>;
}
```

### 功能特性

- **双偏移量类型**：字符和字节偏移
- **运算符重载**：支持算术运算
- **溢出检查**：Debug 模式下检测溢出
- **字符计数**：字节到字符偏移转换
- **范围支持**：创建零长度范围
- **序列化支持**：serde 序列化

## 关键依赖与配置

### 依赖项

- `serde`：序列化支持
- `get-size`：大小计算

### 配置要求

- 无特殊配置

## 数据模型

### 偏移量表示

- **CharOffset**：字符数（Rust char 定义）
- **ByteOffset**：字节数

### 转换规则

- UTF-8 编码
- 多字节字符正确处理
- 无效字节偏移返回 `None`

## 测试与质量

### 测试覆盖

- ✅ 单元测试：`src/lib_tests.rs`
- 测试内容：
  - 偏移量运算
  - 字符计数
  - 边界情况
  - 溢出检测

### 代码质量

- 运算符重载
- 泛型宏实现
- 清晰的错误处理

## 常见问题 (FAQ)

### Q: CharOffset 和 ByteOffset 有何区别？

A: CharOffset 是字符数，ByteOffset 是字节数（UTF-8）。

### Q: 如何处理多字节字符？

A: 使用 `CharCounter` 转换字节偏移到字符偏移。

### Q: 溢出如何处理？

A: Debug 模式下 panic，Release 模式忽略。

### Q: 如何创建范围？

A: 使用 `empty_range()` 或 `range()` 方法。

### Q: 支持负数偏移吗？

A: 通过 `add_signed()` 支持，检查溢出。

### Q: 序列化格式？

A: 直接序列化内部 `usize` 值。

## 相关文件清单

- `src/lib.rs`：主要实现和测试

## 变更记录

### 2026-05-01 (23:18)

- ✅ 初始化模块文档
- ✅ 添加偏移量类型和运算符说明
- ✅ 添加字符计数和转换文档
