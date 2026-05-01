# asset_macro - 资源引用过程宏

[根目录](../../CLAUDE.md) > [crates](../) > **asset_macro**

> 最后更新：2026年 5月 1 日

## 模块职责

提供编译时过程宏，用于安全地引用 Warp 应用程序中的资源文件。支持三种资源类型：打包资源、远程资源和条件资源（原生打包/Web 远程）。

## 入口与启动

### 过程宏入口

- **文件**：`src/lib.rs`
- **类型**：Proc Macro crate（`proc-macro = true`）

### 可用宏

| 宏名 | 用途 | 资源位置 |
|------|------|----------|
| `bundled_asset!` | 始终打包的资源 | `app/assets/bundled/` |
| `remote_asset!` | 始终远程获取的资源 | `app/assets/remote/` |
| `bundled_or_fetched_asset!` | 条件资源 | `app/assets/async/` |

## 对外接口

### 1. bundled_asset! - 打包资源

```rust
bundled_asset!("jpg/jellyfish_bg.jpg")
bundled_asset!("fonts/FiraCode.ttf", "bundled")  // 自定义文件夹
```

**返回类型**：
```rust
::warpui::assets::asset_cache::AssetSource::Bundled {
    path: ::std::borrow::Cow::str,
}
```

**行为**：
- 编译时检查文件是否存在
- 文件不存在时产生编译错误
- 返回内嵌在应用二进制文件中的资源路径

### 2. remote_asset! - 远程资源

```rust
remote_asset!("images/screenshot.png")
remote_asset!("data/config.json", "remote")  // 自定义文件夹
```

**返回类型**：
```rust
::asset_cache::url_source(::warp_util::assets::make_absolute_url(url))
```

**行为**：
- 编译时读取文件内容
- 计算 SHA-256 哈希
- 生成基于哈希的 CDN URL
- 返回 URL 资源源

### 3. bundled_or_fetched_asset! - 条件资源

```rust
bundled_or_fetched_asset!("fonts/Inter.ttf")
```

**行为**：
- **原生平台**：返回 `bundled_asset!`（内嵌在二进制中）
- **WASM 平台**：返回 `remote_asset!`（从 CDN 获取）
- 自动选择最优策略以控制 Web 构建大小

## 关键依赖与配置

### Cargo.toml 依赖

```toml
[dependencies]
quote = "1"
syn = { version = "2", features = ["derive", "parsing", "proc-macro", "printing"] }
proc-macro2 = "1"
sha2 = { workspace = true }
warp_util = { path = "../warp_util" }

[lib]
proc-macro = true
```

### 资源目录结构

```
app/assets/
├── bundled/     # 始终打包（小文件、必需文件）
├── remote/      # 始终远程（大文件、可选内容）
└── async/       # 条件资源（原生打包、Web 远程）
```

## 数据模型

### MacroArgs

```rust
struct MacroArgs {
    asset_name: LitStr,      // 资源名称（如 "jpg/bg.jpg"）
    asset_folder: Option<LitStr>,  // 可选的子文件夹
}
```

### URL 生成

远程资源 URL 格式：
```
https://<cdn-domain>/assets/<hash>/<asset-name>
```

其中 `<hash>` 是文件内容的 SHA-256 哈希的前几位。

## 测试与质量

### 测试覆盖

- **单元测试**：❌ 无
- **编译时测试**：✅ 文件存在性检查
- **文档测试**：❌ 无

### 代码质量特性

- **编译时验证**：确保引用的文件在编译时存在
- **类型安全**：返回强类型的 `AssetSource`
- **零运行时开销**：所有工作在编译时完成
- **清晰的错误消息**：文件缺失时显示完整路径

## 常见问题 (FAQ)

### Q: 为什么需要过程宏而不是普通函数？

A: 过程宏可以在编译时验证文件存在并生成适当的代码，避免运行时错误和性能开销。

### Q: 如何选择资源类型？

A:
- **bundled**：小文件、必需的 UI 元素、字体
- **remote**：大型媒体文件、可选内容
- **async**：在 Web 上需要但原生可以打包的内容

### Q: 文件哈希是如何计算的？

A: 使用 SHA-256 算法计算文件内容的哈希，哈希值用于 CDN URL 以支持缓存失效。

### Q: 可以使用自定义资源文件夹吗？

A: 可以，但通常不需要。默认的 `bundled`、`remote` 和 `async` 文件夹覆盖了所有用例。

### Q: 为什么 WASM 平台使用远程资源？

A: 为了控制 Web 构建的大小，大资源应从 CDN 异步加载而非内嵌在 WASM 二进制中。

## 使用示例

### 基本用法

```rust
use asset_macro::{bundled_asset, remote_asset, bundled_or_fetched_asset};

// 打包资源（小图标）
let icon = bundled_asset!("icons/app_icon.png");

// 远程资源（大型背景图）
let background = remote_asset!("images/large_bg.jpg");

// 条件资源（字体 - 原生打包、Web 远程）
let font = bundled_or_fetched_asset!("fonts/Inter.ttf");
```

### 自定义文件夹

```rust
// 通常不需要，但如果需要可以指定文件夹
let asset = bundled_asset!("data/file.json", "custom_folder");
```

## 相关文件清单

```
crates/asset_macro/
├── Cargo.toml              # 包配置（proc-macro crate）
├── src/
│   └── lib.rs              # 过程宏实现（158 行）
└── CLAUDE.md               # 本文档

app/assets/                 # 资源目录
├── bundled/                # 打包资源
├── remote/                 # 远程资源
└── async/                  # 条件资源
```

## 变更记录

### 2026-05-01

- ✅ 创建初始模块文档
- ✅ 记录三个过程宏的用法和区别
- ✅ 添加资源目录结构和 URL 生成说明
- ✅ 添加使用示例和常见问题
- ✅ 添加导航面包屑

---

*此模块文档是 Warp AI 上下文的一部分*
