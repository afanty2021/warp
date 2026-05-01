[根目录](../../CLAUDE.md) > **crates/persistence**

# Persistence 模块

> 最后更新：2026年 5月 1日

## 模块职责

Persistence 模块负责 Warp 的数据持久化，提供：

- **数据库管理**：SQLite 数据库连接和配置
- **ORM 集成**：Diesel ORM 集成
- **迁移管理**：数据库 schema 迁移
- **模型定义**：数据模型和序列化
- **查询接口**：类型安全的查询接口
- **事务支持**：数据库事务

## 入口与启动

### 主要入口点

- `src/lib.rs` - 库入口
- `src/schema.rs` - 数据库 schema
- `migrations/` - 数据库迁移

### 初始化流程

1. 建立数据库连接
2. 运行待处理的迁移
3. 初始化连接池
4. 设置事务隔离级别

## 对外接口

### 数据库连接

**连接管理**：
```rust
pub struct DbPool {
    // 连接池
}

impl DbPool {
    pub fn new(database_url: &str) -> Result<Self>;
    pub fn get(&self) -> Result<DbConnection>;
    pub fn connect(&mut self) -> Result<()>;
}
```

### 模型 API

**基本模型**：
```rust
#[derive(Queryable, Insertable, Serialize, Deserialize)]
pub struct Model {
    pub id: i32,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
```

**查询**：
```rust
pub trait Repository<T> {
    fn find(&self, id: i32) -> Result<Option<T>>;
    fn find_all(&self) -> Result<Vec<T>>;
    fn save(&self, item: &T) -> Result<()>;
    fn delete(&self, id: i32) -> Result<()>;
}
```

### 迁移 API

**迁移运行**：
```rust
pub fn run_migrations(connection: &mut DbConnection) -> Result<()> {
    // 运行所有待处理的迁移
}
```

## 关键依赖与配置

### 依赖

- `diesel` - ORM 框架
- `diesel_migrations` - 迁移管理
- `chrono` - 时间处理
- `serde` - 序列化
- `warp_multi_agent_api` - Multi-agent API 类型

### 特性标志

- `local_fs` - 本地文件系统访问

### 配置

**数据库 URL**：
- 开发环境：`dev.db`
- 生产环境：通过环境变量配置

**Diesel 配置**：
- `diesel.toml` - Diesel 配置文件
- `print_schema` - Schema 生成配置

## 数据模型

### Schema 定义

```rust
// src/schema.rs (自动生成)
table! {
    models (id) {
        id -> Integer,
        created_at -> Timestamp,
        updated_at -> Timestamp,
    }
}
```

### 模型示例

**对话模型**：
```rust
#[derive(Queryable, Insertable, Serialize, Deserialize)]
pub struct Conversation {
    pub id: i32,
    pub title: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
    pub updated_at: chrono::DateTime<chrono::Utc>,
}
```

**消息模型**：
```rust
#[derive(Queryable, Insertable, Serialize, Deserialize)]
pub struct Message {
    pub id: i32,
    pub conversation_id: i32,
    pub role: String,
    pub content: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
```

### 关联

```rust
#[derive(Queryable, Serialize, Deserialize)]
pub struct ConversationWithMessages {
    pub conversation: Conversation,
    pub messages: Vec<Message>,
}
```

## 测试与质量

### 单元测试

测试文件位置：
- `src/*_tests.rs`
- `src/*/mod_test.rs`
- `tests/` - 集成测试

运行测试：
```bash
cargo nextest run -p persistence
```

### 测试数据库

使用内存数据库进行测试：
```rust
#[tokio::test]
async fn test_repository() {
    let mut connection = establish_test_connection().await?;
    // 测试逻辑
    Ok(())
}
```

### 测试覆盖

当前测试覆盖：
- ✅ 基本 CRUD 操作
- ✅ 关联查询
- ✅ 事务处理
- ✅ 迁移
- ⚠️ 复杂查询
- ⚠️ 并发访问
- ⚠️ 性能测试

## 常见问题 (FAQ)

### Q: 如何创建新的迁移？

A:
```bash
diesel migration generate migration_name
```

这会在 `migrations/` 中创建新的迁移文件。

### Q: 如何处理数据库错误？

A: 使用 `Result` 类型处理错误：
```rust
pub fn find_user(id: i32) -> Result<Option<User>, Error> {
    // 查询逻辑
}
```

### Q: 如何优化查询性能？

A:
1. 使用索引
2. 避免 N+1 查询
3. 使用连接加载
4. 缓存查询结果

### Q: 如何处理事务？

A:
```rust
pub fn transfer_funds(conn: &mut DbConnection, from: i32, to: i32, amount: f64) -> Result<()> {
    conn.transaction::<(), Error, _>(|conn| {
        // 转账逻辑
        Ok(())
    })
}
```

## 相关文件清单

### 核心文件

- `src/lib.rs` - 库入口
- `src/schema.rs` - Schema 定义（自动生成）
- `migrations/` - 数据库迁移
- `src/models.rs` - 数据模型

### 迁移

- `migrations/*/up.sql` - 升级脚本
- `migrations/*/down.sql` - 降级脚本

### 测试

- `src/*_tests.rs` - 单元测试
- `tests/` - 集成测试

### 配置

- `diesel.toml` - Diesel 配置

## 最佳实践

### 查询优化

1. **使用索引**：
```sql
CREATE INDEX index_name ON table_name(column_name);
```

2. **避免 N+1**：
```rust
// 不好：N+1 查询
for conversation in conversations {
    let messages = find_messages(conversation.id)?;
}

// 好：使用加载
let with_messages = conversations.load::<ConversationWithMessages>(conn)?;
```

3. **批量操作**：
```rust
// 批量插入
 diesel::insert_into(messages)
    .values(&message_batch)
    .execute(conn)?;
```

### 错误处理

```rust
pub enum Error {
    Database(diesel::result::Error),
    NotFound,
    Validation(String),
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Error::Database(err)
    }
}
```

## 性能考虑

### 优化策略

1. **连接池**：复用数据库连接
2. **批量操作**：减少往返次数
3. **索引**：加速查询
4. **缓存**：减少数据库访问

### 性能指标

- 查询延迟
- 连接池使用率
- 事务吞吐量
- 数据库大小

## 变更记录

### 2026-05-01

- 初始化 Persistence 模块文档
- 记录核心功能和 API
- 添加最佳实践
