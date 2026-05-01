# Computer Use 模块

[根目录](../../CLAUDE.md) > [crates](../) > **computer_use**

## 模块职责

提供跨平台的计算机控制功能，支持鼠标、键盘、截图等操作，用于 AI Agent 与系统交互。支持 macOS、Windows、Linux（X11/Wayland）。

## 入口与启动

- **主要入口**：`crates/computer_use/src/lib.rs`
- **Actor 创建**：`create_actor() -> Box<dyn Actor>`

## 对外接口

### 核心 Trait

```rust
#[async_trait]
pub trait Actor: Send + Sync + 'static {
    fn platform(&self) -> Option<Platform>;

    async fn perform_actions(
        &mut self,
        actions: &[Action],
        options: Options,
    ) -> Result<ActionResult, String>;
}
```

### 支持的操作

```rust
pub enum Action {
    Wait(Duration),
    MouseDown { button: MouseButton, at: Vector2I },
    MouseUp { button: MouseButton },
    MouseMove { to: Vector2I },
    MouseWheel { at: Vector2I, direction: ScrollDirection, distance: ScrollDistance },
    TypeText { text: String },
    KeyDown { key: Key },
    KeyUp { key: Key },
}
```

### 平台支持

```rust
pub enum Platform {
    Mac,
    Windows,
    LinuxX11,
    LinuxWayland,
}
```

### 功能特性

- **鼠标控制**：点击、移动、滚动
- **键盘输入**：按键、文本输入
- **截图功能**：全屏或区域截图，支持缩放
- **跨平台**：macOS、Windows、Linux（X11/Wayland）
- **异步操作**：支持批量操作执行
- **错误处理**：详细的错误信息
- **测试模式**：`feature = "test-util"` 启用 noop 实现

## 关键依赖与配置

### 依赖项

- `async-trait`：异步 trait 支持
- `serde`：序列化/反序列化
- `pathfinder_geometry`：向量数学

### 平台特定实现

- **macOS**：`src/mac/mod.rs`
- **Windows**：`src/windows/mod.rs`
- **Linux**：`src/linux/mod.rs`（X11/Wayland）
- **Noop**：`src/noop.rs`（测试模式）

### 配置要求

- 需要辅助功能权限（macOS/Windows）
- Linux 需要相应的显示服务器支持
- 截图功能需要屏幕访问权限

## 数据模型

### 按键定义

```rust
pub enum Key {
    Keycode(i32),  // 平台特定虚拟键码
    Char(char),    // 字符键
}
```

### 鼠标按钮

```rust
pub enum MouseButton {
    Left,
    Right,
    Middle,
    Back,    // 鼠标按钮 3（后退）
    Forward, // 鼠标按钮 4（前进）
}
```

### 截图参数

```rust
pub struct ScreenshotParams {
    pub max_long_edge_px: Option<usize>,
    pub max_total_px: Option<usize>,
    pub region: Option<ScreenshotRegion>,
}

pub struct ScreenshotRegion {
    pub top_left: Vector2I,
    pub bottom_right: Vector2I,
}
```

### 操作结果

```rust
pub struct ActionResult {
    pub screenshot: Option<Screenshot>,
    pub cursor_position: Option<Vector2I>,
}
```

## 测试与质量

### 测试覆盖

- ⚠️ 无公开单元测试
- 使用 `test-util` feature 进行集成测试

### 代码质量

- 平台抽象通过 cfg attributes
- 清晰的错误处理
- 序列化支持用于远程执行

## 常见问题 (FAQ)

### Q: 如何获取平台支持？

A: 调用 `is_supported_on_current_platform()` 检查。

### Q: 如何处理权限问题？

A: macOS/Windows 需要辅助功能权限，Linux 需要 X11/Wayland 访问。

### Q: 截图大小限制如何工作？

A: 使用 `max_long_edge_px` 或 `max_total_px` 限制，保持纵横比缩放。

### Q: 如何在测试中使用？

A: 启用 `test-util` feature 使用 noop 实现。

### Q: 支持哪些 Linux 显示服务器？

A: X11 和 Wayland 都支持。

### Q: 如何执行批量操作？

A: 传入 `actions` 数组，按顺序执行。

## 相关文件清单

- `src/lib.rs`：核心类型和 trait 定义
- `src/mac/mod.rs`：macOS 实现
- `src/windows/mod.rs`：Windows 实现
- `src/linux/mod.rs`：Linux 实现
- `src/noop.rs`：测试用 noop 实现
- `src/screenshot_utils.rs`：截图工具函数

## 变更记录

### 2026-05-01 (23:18)

- ✅ 初始化模块文档
- ✅ 添加跨平台支持说明
- ✅ 添加操作和接口文档
- ✅ 添加权限和配置说明
