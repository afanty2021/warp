[根目录](../../CLAUDE.md) > **crates/warpui**

# WarpUI 模块

> 最后更新：2026年 5月 1日

## 模块职责

WarpUI 是 Warp 的自定义 UI 框架，采用 Entity-Component-Handle 模式构建。灵感来自 Flutter，但专门为终端和高性能桌面应用优化。

**核心特性**：
- Entity-Component-Handle 架构
- 声明式 UI 描述
- 高效的渲染和更新
- 跨平台支持（macOS、Windows、Linux、WASM）
- GPU 加速渲染（wgpu）

## 入口与启动

### 主要入口点

- `src/lib.rs` - 库入口
- `src/app.rs` - 全局 App 对象
- `src/context.rs` - AppContext 和 ViewContext

### 初始化流程

1. 创建 `App` 实例
2. 初始化全局状态和 handles
3. 创建根视图
4. 启动渲染循环

## 对外接口

### 核心 API

**App 对象**：
```rust
let mut app = App::new();
```

**视图创建**：
```rust
fn build_view(cx: &mut ViewContext) -> View {
    // 创建视图层次结构
}
```

**元素系统**：
```rust
use warpui::elements::*;

// Flex 布局
Flex::column()
    .with_child(child1)
    .with_child(child2)
```

### 关键类型

- `App` - 全局应用对象
- `View` - 视图实体
- `ViewHandle<T>` - 视图句柄
- `AppContext` - 应用级上下文
- `ViewContext` - 视图级上下文
- `Element` - UI 元素描述

## 关键依赖与配置

### 依赖

- `warpui_core` - UI 框架核心
- `wgpu` - GPU 渲染
- `pathfinder` - 文本渲染
- `futures` - 异步支持
- `tokio` - 异步运行时

### 特性标志

- `schema_gen` - 生成 UI 模式
- `settings_value` - 设置值支持
- `test-util` - 测试工具
- `integration_tests` - 集成测试
- `log_named_telemetry_events` - 遥测日志

### 平台特定

- **macOS**: Metal 渲染
- **Windows**: DirectX 12
- **Linux**: Vulkan
- **WASM**: WebGPU

## 数据模型

### Entity-Handle 系统

```rust
pub struct View {
    // 内部状态
}

pub struct ViewHandle<T> {
    entity: Entity,
    _marker: PhantomData<T>,
}
```

### 元素系统

```rust
pub trait Element {
    fn render(self, cx: &mut RenderContext) -> ViewHandle;
}
```

### 状态管理

```rust
pub struct Model<T> {
    entity: Entity,
    _marker: PhantomData<T>,
}
```

## 测试与质量

### 单元测试

测试文件位置：
- `src/*_tests.rs`
- `src/*/mod_test.rs`

运行测试：
```bash
cargo nextest run -p warpui
```

### UI 测试

WarpUI 提供了专门的 UI 测试框架：
- 视图渲染测试
- 交互测试
- 性能测试

### 测试覆盖

当前测试覆盖：
- ✅ 核心渲染系统
- ✅ 元素系统
- ✅ 事件处理
- ✅ 布局系统
- ⚠️ 复杂 UI 场景
- ⚠️ 性能基准测试

## 常见问题 (FAQ)

### Q: ViewHandle 和直接引用有什么区别？

A: ViewHandle 是对视图实体的弱引用，不会影响生命周期。直接引用会创建所有权关系。

### Q: 如何创建自定义元素？

A: 实现 `Element` trait：
```rust
impl Element for MyCustomElement {
    fn render(self, cx: &mut RenderContext) -> ViewHandle {
        // 实现渲染逻辑
    }
}
```

### Q: 如何处理鼠标输入？

A: 创建 `MouseStateHandle` 并在构建期间使用它。内联 `MouseStateHandle::default()` 会导致鼠标交互失效。

### Q: 性能优化建议？

A:
1. 避免不必要的视图重建
2. 使用句柄而非克隆数据
3. 批量更新状态
4. 使用 GPU 加速

## 相关文件清单

### 核心文件

- `src/lib.rs` - 库入口
- `src/app.rs` - App 对象
- `src/context.rs` - 上下文系统
- `src/entity.rs` - Entity 系统
- `src/view.rs` - 视图系统

### 元素系统

- `src/elements/` - 内置元素
- `src/elements/flex.rs` - Flex 布局
- `src/elements/text.rs` - 文本渲染

### 渲染

- `src/render.rs` - 渲染系统
- `src/paint.rs` - 绘制
- `src/layout.rs` - 布局

### 事件处理

- `src/event.rs` - 事件系统
- `src/input.rs` - 输入处理

### 测试

- `src/*_tests.rs` - 单元测试
- `examples/` - 示例代码

## 架构设计

### Entity-Component-Handle 模式

**核心思想**：
1. **Entity**：唯一的视图实例
2. **Component**：附加到 entity 的数据
3. **Handle**：对 entity 的类型化引用

**优势**：
- 解耦视图生命周期
- 灵活的状态共享
- 高效的更新机制

### 渲染流程

1. **构建阶段**：创建视图层次结构
2. **布局阶段**：计算大小和位置
3. **绘制阶段**：GPU 渲染
4. **事件阶段**：处理用户输入

### 状态管理

- **全局状态**：通过 `App` 管理
- **本地状态**：通过 `Model` 管理
- **临时状态**：通过 `Context` 传递

## 性能考虑

### 优化策略

1. **增量渲染**：只重绘变化的部分
2. **GPU 加速**：使用 wgpu 进行硬件加速
3. **批处理**：合并绘制调用
4. **缓存**：缓存计算结果

### 性能监控

- 使用内置的性能分析工具
- 监控帧率和渲染时间
- 识别性能瓶颈

## 变更记录

### 2026-05-01

- 初始化 WarpUI 模块文档
- 记录核心架构和 API
- 添加性能优化建议
