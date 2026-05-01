[根目录](../../CLAUDE.md) > **crates/warpui_core**

# WarpUI Core 模块

> 最后更新：2026年 5月 1日

## 模块职责

WarpUI Core 是 WarpUI 框架的核心组件，提供：

- **Entity 系统**：实体管理和生命周期
- **Component 系统**：组件数据管理
- **事件系统**：事件分发和处理
- **状态管理**：全局和本地状态
- **渲染上下文**：渲染所需的核心上下文
- **平台抽象**：跨平台接口

## 入口与启动

### 主要入口点

- `src/lib.rs` - 库入口
- `src/entity.rs` - Entity 系统
- `src/context.rs` - 上下文系统
- `src/app.rs` - 应用核心

### 初始化流程

1. 创建全局 App 实例
2. 初始化 Entity 系统
3. 设置事件循环
4. 初始化渲染上下文

## 对外接口

### Entity API

**实体创建**：
```rust
pub struct Entity {
    pub id: EntityId,
    pub generation: Generation,
}

impl Entity {
    pub fn new() -> Self;
    pub fn is_valid(&self) -> bool;
}
```

**实体管理**：
```rust
pub struct EntityManager {
    // 实体存储
}

impl EntityManager {
    pub fn create(&mut self) -> Entity;
    pub fn destroy(&mut self, entity: Entity);
    pub fn is_alive(&self, entity: Entity) -> bool;
}
```

### Component API

**组件定义**：
```rust
pub trait Component: Any + Send + Sync {
    fn type_name() -> &'static str;
}
```

**组件存储**：
```rust
pub struct ComponentStore<T: Component> {
    // 组件数据
}

impl<T: Component> ComponentStore<T> {
    pub fn insert(&mut self, entity: Entity, component: T);
    pub fn get(&self, entity: Entity) -> Option<&T>;
    pub fn get_mut(&mut self, entity: Entity) -> Option<&mut T>;
    pub fn remove(&mut self, entity: Entity) -> Option<T>;
}
```

### 事件 API

**事件定义**：
```rust
pub trait Event: Any + Send + Sync {
    fn type_name() -> &'static str;
}
```

**事件总线**：
```rust
pub struct EventBus {
    // 事件队列和处理器
}

impl EventBus {
    pub fn subscribe<E: Event>(&mut self, handler: impl Fn(&E) + 'static);
    pub fn publish<E: Event>(&mut self, event: E);
}
```

## 关键依赖与配置

### 依赖

- `dashmap` - 并发哈希表
- `parking_lot` - 高性能锁
- `async-channel` - 异步通道
- `sum_tree` - 增量数据结构
- `serde` - 序列化

### 特性标志

- `traces` - 性能追踪
- `integration_tests` - 集成测试
- `log_named_telemetry_events` - 遥测日志
- `schema_gen` - 模式生成
- `settings_value` - 设置值支持
- `test-util` - 测试工具

### 平台特定

**macOS**：
- Metal 渲染支持
- Core Text 文本渲染
- 原生窗口管理

**Linux/Windows**：
- Vulkan/DirectX 渲染
- fontconfig/DirectWrite 文本
- X11/Win32 窗口

**WASM**：
- WebGPU/WebGL
- Web 文本 API
- 浏览器窗口

## 数据模型

### Entity

```rust
pub struct Entity {
    pub id: EntityId,
    pub generation: Generation,
}

pub type EntityId = u32;
pub type Generation = u32;
```

### Component

```rust
pub trait Component: Any + Send + Sync {
    fn type_name() -> &'static str;
}
```

### Event

```rust
pub trait Event: Any + Send + Sync {
    fn type_name() -> &'static str;
}
```

### Context

```rust
pub struct Context {
    pub entity_manager: EntityManager,
    pub event_bus: EventBus,
    pub resources: ResourceMap,
}
```

## 测试与质量

### 单元测试

测试文件位置：
- `src/*_tests.rs`
- `src/*/mod_test.rs`

运行测试：
```bash
cargo nextest run -p warpui_core
```

### 测试覆盖

当前测试覆盖：
- ✅ Entity 生命周期
- ✅ Component 存储
- ✅ 事件分发
- ✅ 资源管理
- ✅ 并发安全性
- ⚠️ 性能测试
- ⚠️ 压力测试

## 常见问题 (FAQ)

### Q: Entity 和 Component 的关系？

A:
- Entity 是唯一的 ID
- Component 是附加到 Entity 的数据
- 一个 Entity 可以有多个 Component
- Component 通过 Entity ID 关联

### Q: 如何处理并发访问？

A:
1. 使用 `dashmap` 进行并发访问
2. 使用 `parking_lot` 的高性能锁
3. 使用 `async-channel` 进行异步通信
4. 避免死锁

### Q: 事件系统如何工作？

A:
1. 订阅事件类型
2. 发布事件到总线
3. 总线分发给订阅者
4. 订阅者处理事件

### Q: 如何优化性能？

A:
1. 使用增量更新
2. 批量处理操作
3. 缓存计算结果
4. 减少锁竞争

## 相关文件清单

### 核心文件

- `src/lib.rs` - 库入口
- `src/entity.rs` - Entity 系统
- `src/component.rs` - Component 系统
- `src/event.rs` - 事件系统
- `src/context.rs` - 上下文系统
- `src/app.rs` - 应用核心

### 存储

- `src/storage/` - 存储实现
- `src/storage/dashmap_store.rs` - DashMap 存储

### 事件

- `src/event/bus.rs` - 事件总线
- `src/event/handler.rs` - 事件处理器

### 资源

- `src/resource.rs` - 资源管理

### 测试

- `src/*_tests.rs` - 单元测试

## 架构设计

### ECS 模式

**Entity-Component-System (ECS)**：
- **Entity**: 唯一标识符
- **Component**: 数据
- **System**: 逻辑（在 WarpUI 中通过 View 实现）

### 优势

1. **性能**：数据局部性好
2. **灵活性**：易于组合
3. **可扩展性**：易于添加新功能
4. **并发性**：易于并行处理

### 事件驱动

**事件循环**：
1. 接收事件
2. 更新状态
3. 渲染
4. 重复

## 性能考虑

### 优化策略

1. **增量更新**：只更新变化的部分
2. **批量处理**：合并操作
3. **缓存**：缓存计算结果
4. **并行处理**：利用多核

### 性能指标

- 帧率（FPS）
- 事件延迟
- 内存使用
- CPU 使用率

## 变更记录

### 2026-05-01

- 初始化 WarpUI Core 模块文档
- 记录核心架构和 API
- 添加性能优化信息
