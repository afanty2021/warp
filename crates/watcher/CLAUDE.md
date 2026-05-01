# Watcher - 文件系统监控模块

[根目录](../../CLAUDE.md) > [crates](../) > **watcher**

> 最后更新：2026-05-01

## 模块职责

Watcher 模块为 Warp 提供了文件系统变化监控功能。它使用 `notify` 库跨平台文件监控 API，实现了事件去抖动（debouncing）和批量事件合并，为上层应用提供简洁的文件变化事件流。

**核心功能**：
- 跨平台文件系统监控（macOS、Linux、Windows）
- 事件去抖动，避免频繁触发
- 批量事件合并，将多个相关事件合并为单个更新
- 递归和非递归监控模式
- 支持监控路径的动态注册和注销

## 入口与启动

### 主要类型

**`BulkFilesystemWatcher`** - 批量文件系统监控器
- 监控指定目录下的所有文件变化
- 提供去抖动和批量事件合并
- 通过 WarpUI 的 Entity 模型集成

**`HomeDirectoryWatcher`** - 用户主目录监控器
- 专门用于监控用户主目录（非递归）
- 单例实体（SingletonEntity）
- 基于 `BulkFilesystemWatcher` 实现

### 使用示例

```rust
// 创建监控器
let watcher = BulkFilesystemWatcher::new(
    Duration::from_millis(500),
    ctx
);

// 注册监控路径
watcher.update(ctx, |watcher, ctx| {
    let result = watcher.register_path(
        &PathBuf::from("/path/to/watch"),
        WatchFilter::accept_all(),
        RecursiveMode::Recursive,
    );
    // 等待注册完成
    smol::block_on(result).unwrap();
});

// 订阅事件
ctx.subscribe_to_model(&watcher, |this, event, ctx| {
    match event {
        BulkFilesystemWatcherEvent { added, modified, deleted, moved } => {
            // 处理文件变化
        }
    }
});
```

## 对外接口

### 核心接口

**`BulkFilesystemWatcher::new`**：
```rust
pub fn new(debounce_duration: Duration, ctx: &mut ModelContext<Self>) -> Self
```
创建新的监控器，指定去抖动持续时间。

**`BulkFilesystemWatcher::register_path`**：
```rust
pub fn register_path(
    &mut self,
    path: &Path,
    watch_filter: WatchFilter,
    recursive_mode: RecursiveMode,
) -> impl Future<Output = Result<()>>
```
注册新的监控路径。

**`BulkFilesystemWatcher::unregister_path`**：
```rust
pub fn unregister_path(&mut self, path: &Path) -> impl Future<Output = Result<()>>
```
注销监控路径。

### 事件类型

**`BulkFilesystemWatcherEvent`**：
```rust
pub struct BulkFilesystemWatcherEvent {
    pub added: HashSet<PathBuf>,      // 新创建的文件
    pub modified: HashSet<PathBuf>,   // 修改的文件
    pub deleted: HashSet<PathBuf>,    // 删除的文件
    pub moved: HashMap<PathBuf, PathBuf>, // 重命名映射 (目标 -> 源)
}
```

### 实体实现

**`Entity` trait**：
- `type Event = BulkFilesystemWatcherEvent`
- 通过 `ctx.emit()` 发送事件

## 关键依赖与配置

### 依赖项

**核心依赖**：
- `notify-debouncer-full` - 文件监控和去抖动
- `futures` - 异步工具
- `async-channel` - 异步通道
- `anyhow` - 错误处理
- `warpui` - WarpUI 框架集成

### 配置

**去抖动持续时间**：
- 默认：500 毫秒
- 可通过构造函数参数配置
- 过短的持续时间可能导致大量事件
- 过长的持续时间可能导致延迟感知

**监控过滤器（WatchFilter）**：
- `WatchFilter::accept_all()` - 接受所有事件
- 可配置为只监听特定类型的事件

## 数据模型

### 后台架构

**后台线程**：
- 监控器在独立线程中运行
- 通过通道（channel）与主线程通信
- 避免阻塞主线程

**命令类型**：
```rust
enum BackgroundFileWatcherCommand {
    AddPath {
        path: PathBuf,
        filter: WatchFilter,
        response: oneshot::Sender<Result<()>>,
        recursive_mode: RecursiveMode,
    },
    RemovePath {
        path: PathBuf,
        response: oneshot::Sender<Result<()>>,
    },
}
```

### 事件处理

**事件去重和合并**：
- 同一文件的创建+修改 → 创建
- 同一文件的修改+删除 → 删除
- 同一文件的创建+删除 → 忽略
- 重命名事件配对 → 单个移动事件

## 测试与质量

### 测试覆盖

⚠️ **单元测试**：当前没有发现单元测试（`#[test]` 计数：0）

### 测试策略

**推荐测试方法**：
1. 使用临时目录创建测试文件结构
2. 触发文件系统操作（创建、修改、删除、重命名）
3. 验证事件流是否正确
4. 测试去抖动功能
5. 测试并发监控路径注册/注销

**测试辅助**：
- `BulkFilesystemWatcher::new_for_test()` - 创建不带后台线程的测试实例

### 集成测试

建议在集成测试中验证：
- 与 WarpUI 模型系统的集成
- 事件订阅和分发
- 多路径监控
- 长时间运行的稳定性

## 常见问题 (FAQ)

### Q: 如何调整去抖动持续时间？

**A**:
```rust
// 较短的去抖动（100ms）- 更快响应，更多事件
let watcher = BulkFilesystemWatcher::new(
    Duration::from_millis(100),
    ctx
);

// 较长的去抖动（2000ms）- 更少事件，更多延迟
let watcher = BulkFilesystemWatcher::new(
    Duration::from_secs(2),
    ctx
);
```

### Q: 为什么有些文件事件被忽略？

**A**: 事件合并逻辑可能会过滤某些事件：
- 文件创建后立即删除 → 被过滤
- 文件修改后被删除 → 只报告删除
- 这是设计行为，避免无意义的临时事件

### Q: 如何监控特定类型的文件？

**A**: 使用 `WatchFilter`：
```rust
use notify::event::EventKind;

let filter = WatchFilter::new(|event| {
    matches!(event.kind, EventKind::Create(_) | EventKind::Modify(_))
});

watcher.register_path(path, filter, RecursiveMode::Recursive).await;
```

### Q: 监控大量文件会影响性能吗？

**A**:
- ✅ 去抖动减少了事件数量
- ✅ 批量处理提高了效率
- ✅ 后台线程避免阻塞主线程
- ⚠️ 监控数万个文件可能仍会影响性能
- 建议：只监控需要的目录

### Q: 如何处理监控错误？

**A**:
```rust
watcher.update(ctx, |watcher, ctx| {
    let result = watcher.register_path(path, filter, recursive_mode);
    match smol::block_on(result) {
        Ok(()) => log::info!("Registered watcher for {:?}", path),
        Err(e) => log::error!("Failed to register watcher: {:?}", e),
    }
});
```

### Q: macOS 和 Windows 有什么特殊注意事项？

**A**:
**macOS**:
- 删除文件可能是移动到废纸篓
- 重命名事件可能以 `RenameMode::Any` 形式出现

**Windows**:
- 文件修改可能使用 `ModifyKind::Any` 而非 `ModifyKind::Data`
- 路径格式可能不同

## 相关文件清单

### 核心文件

- `src/lib.rs` - 模块入口，批量文件监控实现
- `src/home_watcher.rs` - 主目录监控器实现

## 变更记录

### 2026-05-01

- ✅ 创建模块文档
- ✅ 记录事件去重和合并逻辑
- ✅ 文档化后台线程架构
- ✅ 添加测试策略建议

---

*本文档由 AI 自动生成和维护。如有问题或建议，请在 issue 中提出。*
