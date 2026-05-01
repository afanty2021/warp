# Virtual FS 模块

[根目录](../../CLAUDE.md) > [crates](../) > **virtual_fs**

## 模块职责

提供虚拟文件系统测试工具，用于集成测试中创建临时文件和目录。支持文件创建、目录结构、符号链接（Unix）等操作。

## 入口与启动

- **主要入口**：`crates/virtual_fs/src/lib.rs`
- **核心结构**：`VirtualFS`
- **辅助工具**：`Warp`（路径工具）

## 对外接口

### 核心类型

```rust
pub struct VirtualFS {
    root: TempDir,
    cwd: PathBuf,
    tests: String,
}

pub struct Dirs {
    pub root: PathBuf,
    pub tests: PathBuf,
}
```

### 文件存根

```rust
pub enum Stub<'a> {
    FileWithContent(&'a str, &'a str),
    FileWithContentToBeTrimmed(&'a str, &'a str),
    EmptyFile(&'a str),
    #[cfg(unix)]
    MockExecutable(&'a str),
}
```

### 主要方法

```rust
impl VirtualFS {
    // 创建测试环境
    pub fn test(tag: &str, test_callback: impl FnOnce(Dirs, VirtualFS));

    // 文件系统操作
    pub fn mkdir(&mut self, directory: &str) -> &mut Self;
    pub fn with_files(&mut self, files: Vec<Stub>) -> &mut Self;
    pub fn touch(&mut self, files: Vec<Stub>) -> &mut Self;

    #[cfg(unix)]
    pub fn ln<T, U>(&mut self, target: T, link: U) -> &mut Self;

    // 导航
    pub fn back_to_root(&mut self) -> &mut Self;
}
```

### 功能特性

- **临时目录**：自动创建和管理临时文件系统
- **文件创建**：支持带内容或不带内容的文件
- **目录结构**：创建嵌套目录结构
- **符号链接**：Unix 系统支持符号链接
- **可执行文件**：Unix 系统支持创建可执行文件（755 权限）
- **自动清理**：测试结束后自动清理临时文件
- **路径规范化**：使用 `dunce` 规范化路径

## 关键依赖与配置

### 依赖项

- `tempfile`：临时目录管理
- `getset`：Getter 生成器

### 配置要求

- 仅用于测试环境
- Unix 特定功能需要 cfg(unix)

## 数据模型

### 测试模式

```rust
VirtualFS::test("my_test", |dirs, fs| {
    // 在临时文件系统中操作
    fs.mkdir("src").with_files(vec![
        Stub::FileWithContent("main.rs", "fn main() {}"),
    ]);
});
```

### 文件内容处理

- **FileWithContent**：直接使用提供的内容
- **FileWithContentToBeTrimmed**：去除首行并 trim 每行
- **EmptyFile**：创建空文件（"fake data"）
- **MockExecutable**：创建可执行文件（Unix，755 权限）

### 工具结构

```rust
impl Warp {
    pub fn executable() -> PathBuf;  // Warp 可执行文件路径
    pub fn fixtures() -> PathBuf;    // 测试 fixture 路径
    pub fn root() -> PathBuf;        // 仓库根路径
}
```

## 测试与质量

### 测试覆盖

- ⚠️ 本身是测试工具，无单独测试
- 通过使用它的测试间接验证

### 代码质量

- 链式调用 API
- 自动资源管理
- 平台特定代码隔离

## 常见问题 (FAQ)

### Q: 为什么使用虚拟文件系统？

A: 隔离测试环境，避免污染实际文件系统。

### Q: 如何访问创建的文件？

A: 使用 `Dirs::tests` 获取测试目录路径。

### Q: 支持 Windows 吗？

A: 基本支持，但符号链接等 Unix 特性不可用。

### Q: 如何创建复杂目录结构？

A: 链式调用 `mkdir()` 和 `with_files()`。

### Q: 临时文件何时清理？

A: `VirtualFS` 析构时自动清理。

### Q: 如何获取 Warp 可执行文件？

A: 使用 `Warp::executable()` 获取编译后的路径。

## 相关文件清单

- `src/lib.rs`：主要实现

## 变更记录

### 2026-05-01 (23:18)

- ✅ 初始化模块文档
- ✅ 添加测试工具和 API 说明
- ✅ 添加文件存根和操作说明
- ✅ 添加使用示例
