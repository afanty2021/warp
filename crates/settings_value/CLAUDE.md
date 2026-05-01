# Settings Value 模块

[根目录](../../CLAUDE.md) > [crates](../) > **settings_value**

## 模块职责

定义设置值的序列化/反序列化接口，提供用户友好的 TOML 设置文件格式。支持自定义文件格式、JSON Schema 生成和派生宏。

## 入口与启动

- **主要入口**：`crates/settings_value/src/lib.rs`
- **核心 Trait**：`SettingsValue`
- **派生宏**：`#[derive(SettingsValue)]`（feature-gated）

## 对外接口

### 核心 Trait

```rust
pub trait SettingsValue: Serialize + DeserializeOwned {
    // 转换为文件格式（JSON → TOML）
    fn to_file_value(&self) -> Value;

    // 从文件格式恢复（TOML → JSON）
    fn from_file_value(value: &Value) -> Option<Self>
    where
        Self: Sized;

    // JSON Schema 生成
    fn file_schema(gen: &mut schemars::SchemaGenerator) -> schemars::Schema
    where
        Self: schemars::JsonSchema;
}
```

### 功能特性

- **自定义序列化**：独立于 serde 的文件格式
- **派生宏**：自动实现（枚举转 snake_case，递归字段）
- **Passthrough**：委托给 serde（空 impl）
- **手动实现**：完全自定义格式
- **泛型支持**：Vec、Option、HashMap、HashSet
- **Schema 生成**：集成 schemars

## 关键依赖与配置

### 依赖项

- `serde`：标准序列化
- `serde_json`：JSON 中间格式
- `schemars`：JSON Schema 生成
- `settings_value_derive`：派生宏（可选）
- `chrono`：DateTime 支持

### 特性标志

- `derive`：启用派生宏

### 配置要求

- TOML 后端处理 JSON 表示
- 云同步和原生存储继续使用 serde

## 数据模型

### 实现策略

#### 1. 派生宏（推荐）

```rust
#[derive(SettingsValue)]
enum MyEnum {
    VariantOne,
    VariantTwo,
}
```

- 枚举变体 → snake_case
- 递归处理字段
- **绕过 serde**，不调用 `serde_json::to_value`

#### 2. Passthrough（serde 委托）

```rust
impl SettingsValue for MyType {}
```

- 使用现有 `Serialize`/`Deserialize` impls
- 适用于自定义 serde 格式或外部类型

#### 3. 手动实现

```rust
impl SettingsValue for Duration {
    fn to_file_value(&self) -> Value {
        Value::Number(self.as_secs().into())
    }
    // ...
}
```

- 完全自定义格式
- 用于特殊需求（如 Duration → 整数秒）

### 预定义实现

- **原始类型**：bool、数字、String、PathBuf、DateTime
- **集合**：Vec、Option、HashSet、HashMap（递归）
- **特殊**：Duration（整数秒）

## 测试与质量

### 测试覆盖

- ✅ 单元测试：`src/tests.rs`
- 测试内容：
  - Duration 往返转换
  - Vec 递归序列化
  - Option 处理
  - HashMap 往返转换
  - 原始类型 passthrough

### 代码质量

- 类型安全
- 清晰的错误处理
- 完善的测试覆盖

## 常见问题 (FAQ)

### Q: 何时使用派生宏？

A: 大多数情况，特别是新类型。

### Q: 何时使用 passthrough？

A: 已有自定义 serde impls 或外部类型。

### Q: 何时手动实现？

A: 派生宏和 serde 都不符合需求时。

### Q: Duration 如何序列化？

A: 整数秒（而非 `{ secs, nanos }`）。

### Q: 云同步使用这个 trait 吗？

A: 不，云同步继续使用 serde。

### Q: 如何处理外部类型？

A: 使用 `impl_snake_case!` 宏或 passthrough。

## 相关文件清单

- `src/lib.rs`：主要实现和测试

## 变更记录

### 2026-05-01 (23:18)

- ✅ 初始化模块文档
- ✅ 添加 trait 和实现策略说明
- ✅ 添加派生宏和 passthrough 文档
- ✅ 添加测试覆盖说明
