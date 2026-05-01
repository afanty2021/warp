# UI Components 模块

[根目录](../../CLAUDE.md) > [crates](../) > **ui_components**

## 模块职责

提供可复用的 UI 组件库，定义组件设计模式，包括按钮、对话框、开关、工具提示等常用 UI 元素。该模块强调组件的长生命周期和状态管理。

## 入口与启动

- **主要入口**：`crates/ui_components/src/lib.rs`
- **组件列表**：button、dialog、keyboard_shortcut、lightbox、switch、tooltip

## 对外接口

### 核心 Traits

```rust
pub trait Component: Default {
    type Params<'a>: Params;
    fn render<'a>(&self, appearance: &Appearance, params: Self::Params<'a>) -> Box<dyn Element>;
}

pub trait Params {
    type Options<'a>: Options;
}

pub trait Options {
    fn default(appearance: &Appearance) -> Self;
}

pub trait Renderable<'a> {
    fn render(self: Box<Self>, appearance: &Appearance) -> Box<dyn Element>;
}
```

### 组件列表

- **button**：按钮组件（主要、次要、危险等主题）
- **switch**：开关切换组件
- **dialog**：对话框组件
- **tooltip**：工具提示组件
- **keyboard_shortcut**：键盘快捷键显示
- **lightbox**：图片灯箱组件

### 功能特性

- **组件设计模式**：分离组件状态、参数和选项
- **长生命周期**：组件存储为视图字段而非每次渲染创建
- **鼠标状态管理**：内置鼠标悬停状态处理
- **外观适配**：基于主题的默认值
- **灵活渲染**：支持组件或闭包作为渲染参数

## 关键依赖与配置

### 依赖项

- `warpui`：UI 框架核心（Element、EventContext、AppContext）
- `warp_core`：核心功能（Appearance）

### 配置要求

- 组件应存储在视图中，而非每次渲染时创建
- 使用 `MouseStateHandle` 管理鼠标状态

## 数据模型

### 组件模式

```rust
// 组件结构体（持久状态）
pub struct MyComponent {
    mouse_state: MouseStateHandle,
}

// 参数结构体（必需参数）
pub struct Params {
    pub content: String,
    pub options: Options,
}

// 选项结构体（可选参数）
pub struct Options {
    pub disabled: bool,
    pub size: f32,
}
```

### 设计原则

1. **Component**：组件结构体，持有持久状态
2. **Params**：必需参数 + 选项字段
3. **Options**：可选参数，提供外观相关的默认值

## 测试与质量

### 测试覆盖

- ⚠️ 无公开单元测试
- 依赖集成测试和视觉测试

### 代码质量

- 严格的 trait 约束确保类型安全
- 清晰的设计模式和文档
- 生命周期管理防止状态错误

## 常见问题 (FAQ)

### Q: 为什么组件要存储为视图字段？

A: 为了保持鼠标悬停等状态在渲染之间正确更新。每次创建新组件会导致状态丢失。

### Q: 如何实现自定义组件？

A: 实现 `Component`、`Params` 和 `Options` 三个 trait。

### Q: 如何使用现有组件？

A: 将组件实例存储为字段，调用 `render()` 方法传入参数。

### Q: Options 和 Params 有什么区别？

A: Params 包含必需参数，Options 包含可选参数并提供默认值。

## 相关文件清单

- `src/lib.rs`：核心 trait 定义
- `src/button.rs`：按钮组件
- `src/switch.rs`：开关组件
- `src/dialog.rs`：对话框组件
- `src/tooltip.rs`：工具提示组件
- `src/keyboard_shortcut.rs`：键盘快捷键
- `src/lightbox.rs`：灯箱组件

## 变更记录

### 2026-05-01 (23:18)

- ✅ 初始化模块文档
- ✅ 添加组件设计模式说明
- ✅ 添加核心 trait 文档
- ✅ 添加使用示例
