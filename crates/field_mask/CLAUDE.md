# Field Mask 模块

[根目录](../../CLAUDE.md) > [crates](../) > **field_mask**

## 模块职责

提供 Protocol Buffer FieldMask 操作的实现，支持选择性更新和追加操作。用于部分更新 protobuf 消息字段。

## 入口与启动

- **主要入口**：`crates/field_mask/src/lib.rs`
- **核心操作**：`FieldMaskOperation`

## 对外接口

### 核心类型

```rust
pub struct FieldMaskOperation<'a, T: prost::Message + Default> {
    message_descriptor: &'static MessageDescriptor,
    mask: FieldMask,
    destination: &'a T,
    source: &'a T,
    op: OperationType,
}
```

### 操作方法

```rust
impl<'a, T: prost::Message + Default> FieldMaskOperation<'a, T> {
    // 更新操作：替换目标字段
    pub fn update(
        message_descriptor: &'static MessageDescriptor,
        destination: &'a T,
        source: &'a T,
        mask: FieldMask,
    ) -> Self;

    // 追加操作：连接字符串字段
    pub fn append(
        message_descriptor: &'static MessageDescriptor,
        destination: &'a T,
        source: &'a T,
        mask: FieldMask,
    ) -> Self;

    // 应用操作
    pub fn apply(self) -> Result<T>;
}
```

### 功能特性

- **部分更新**：只更新 FieldMask 中指定的字段
- **嵌套路径**：支持嵌套消息字段（如 `user.profile.name`）
- **重复字段**：支持重复字段的批量更新
- **追加模式**：支持字符串字段的追加操作
- **类型安全**：基于 prost 的类型安全操作
- **错误处理**：详细的错误信息

## 关键依赖与配置

### 依赖项

- `prost`：Protocol Buffer 实现
- `prost-reflect`：动态消息反射
- `prost-types`：protobuf 标准类型
- `itertools`：迭代器工具

### 配置要求

- 需要有效的 `MessageDescriptor`
- FieldMask 路径必须符合 protobuf 规范

## 数据模型

### 错误类型

```rust
pub enum FieldMaskError {
    Decode(#[from] prost::DecodeError),
    InvalidPath(String),
    UnsupportedAppend(String),
    SetField(#[from] prost_reflect::SetFieldError),
}
```

### 操作类型

- **Update**：替换目标字段的值
- **Append**：将源字段的值追加到目标字段（仅字符串）

### 路径处理

- 支持点分隔的嵌套路径（如 `a.b.c`）
- 支持重复字段的索引（如 `items[0].name`）
- 未知字段自动跳过（向后兼容）

## 测试与质量

### 测试覆盖

- ⚠️ 无公开单元测试
- 依赖集成测试验证功能

### 代码质量

- 使用 `thiserror` 定义错误类型
- 清晰的错误消息
- 类型安全的泛型实现

## 常见问题 (FAQ)

### Q: 如何处理未知字段？

A: 未知字段自动跳过，保持向后兼容性。

### Q: 追加操作支持哪些类型？

A: 目前只支持字符串字段。

### Q: 如何处理重复字段？

A: 重复字段必须长度相同，逐个应用操作。

### Q: 路径格式是什么？

A: 使用点分隔的嵌套路径，如 `user.profile.name`。

### Q: 如何获取 MessageDescriptor？

A: 使用 prost-reflect 从 .proto 文件生成。

## 相关文件清单

- `src/lib.rs`：主要实现

## 变更记录

### 2026-05-01 (23:18)

- ✅ 初始化模块文档
- ✅ 添加 FieldMask 操作说明
- ✅ 添加错误处理文档
- ✅ 添加使用示例
