[根目录](../../CLAUDE.md) > **crates/integration**

# 集成测试模块

> 最后更新：2026年 5月 1日

## 模块职责

集成测试模块提供 Warp 的端到端测试框架，用于：

- **UI 交互测试**：模拟用户交互
- **功能测试**：验证完整功能流程
- **性能测试**：测量应用性能
- **回归测试**：防止功能退化

## 入口与启动

### 主要入口点

- `src/lib.rs` - 测试框架入口
- `tests/` - 集成测试文件

### 运行测试

```bash
# 运行所有集成测试
cargo nextest run -p integration

# 运行特定测试
cargo nextest run -p integration -- test_name

# 运行特定类别的测试
cargo nextest run -p integration -- --test-threads=1
```

## 对外接口

### 测试框架 API

**测试上下文**：
```rust
pub struct TestContext {
    pub app: App,
    pub terminal: TerminalHandle,
}

impl TestContext {
    pub fn new() -> Self;
    pub fn run_test<F>(f: F) -> Result<()>
    where F: FnOnce(&mut Self) -> Result<()>;
}
```

**断言**：
```rust
pub fn assert_text_visible(text: &str);
pub fn assert_element_exists(selector: &str);
pub fn assert_terminal_contains(text: &str);
```

### 测试辅助工具

**终端模拟**：
```rust
pub struct TerminalHandle {
    // 终端句柄
}

impl TerminalHandle {
    pub fn write(&mut self, text: &str);
    pub fn press_key(&mut self, key: Key);
    pub fn get_output(&self) -> String;
}
```

**UI 交互**：
```rust
pub fn click_element(selector: &str);
pub fn type_text(text: &str);
pub fn wait_for_element(selector: &str);
```

## 关键依赖与配置

### 依赖

- `warp` - 主应用
- `warpui` - UI 框架
- `warp_terminal` - 终端模块
- `tokio` - 异步运行时

### 特性标志

- `integration_tests` - 启用集成测试

### 配置

测试配置文件：
- `.config/nextest.toml` - Nextest 配置
- `crates/integration/tests/config/` - 测试配置

## 测试组织

### 测试目录结构

```
tests/
├── ai/              # AI 功能测试
├── terminal/        # 终端功能测试
├── ui/              # UI 交互测试
├── performance/     # 性能测试
└── regression/      # 回归测试
```

### 测试命名约定

- `test_*` - 普通测试
- `integration_*` - 集成测试
- `e2e_*` - 端到端测试

## 测试与质量

### 编写测试

**基本测试结构**：
```rust
#[tokio::test]
async fn test_feature_name() {
    let mut ctx = TestContext::new();
    ctx.run_test(|ctx| {
        // 测试逻辑
        Ok(())
    }).await.unwrap();
}
```

### 测试最佳实践

1. **独立性**：每个测试应该独立运行
2. **可重复性**：测试应该可重复
3. **快速**：测试应该快速执行
4. **清晰**：测试名称和断言应该清晰

### 测试覆盖

当前测试覆盖：
- ✅ 基本功能测试
- ✅ AI 功能测试
- ✅ 终端渲染测试
- ⚠️ 复杂工作流
- ⚠️ 性能回归
- ⚠️ 边界情况

## 常见问题 (FAQ)

### Q: 如何调试失败的测试？

A:
1. 启用详细日志
2. 使用 `--nocapture` 查看输出
3. 减小测试规模
4. 使用调试器

### Q: 测试太慢怎么办？

A:
1. 并行运行测试
2. 使用 mock 替代慢操作
3. 优化测试设置
4. 考虑使用测试数据库

### Q: 如何模拟用户交互？

A: 使用提供的辅助工具：
```rust
click_element("#button-id");
type_text("hello world");
press_key(Key::Enter);
```

### Q: 如何测试异步操作？

A: 使用 `tokio::test` 和等待辅助工具：
```rust
#[tokio::test]
async fn test_async_feature() {
    wait_for_condition(|| condition_met()).await;
}
```

## 相关文件清单

### 核心文件

- `src/lib.rs` - 测试框架
- `tests/` - 测试文件
- `src/client.rs` - 测试客户端
- `src/assertions.rs` - 断言工具

### 测试工具

- `src/terminal.rs` - 终端测试工具
- `src/ui.rs` - UI 测试工具
- `src/ai.rs` - AI 测试工具

### 配置

- `.config/nextest.toml` - Nextest 配置
- `tests/config/` - 测试配置

## 性能测试

### 性能指标

- 帧率（FPS）
- 渲染时间
- 内存使用
- 启动时间

### 性能基准

```rust
#[bench]
fn bench_render(b: &mut Bencher) {
    b.iter(|| {
        // 测试代码
    });
}
```

## CI/CD 集成

### GitHub Actions

集成测试在 CI 中自动运行：
- 每次 PR
- 每次 merge 到主分支
- 定时运行

### 测试报告

测试结果发布到：
- GitHub Actions 界面
- 构建仪表板
- 覆盖率报告

## 变更记录

### 2026-05-01

- 初始化集成测试模块文档
- 记录测试框架和工具
- 添加最佳实践
