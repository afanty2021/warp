# Asset Cache 模块

[根目录](../../CLAUDE.md) > [crates](../) > **asset_cache**

## 模块职责

提供异步资源加载和缓存功能，支持从 URL 获取资源并可选地持久化到本地文件系统。该模块优化了网络资源加载性能，支持内存缓存和磁盘缓存。

## 入口与启动

- **主要入口**：`crates/asset_cache/src/lib.rs`
- **核心扩展**：`AssetCacheExt` trait
- **资源类型**：`UrlAssetWithoutPersistence`、`UrlAssetWithPersistence`

## 对外接口

### 核心函数

```rust
pub fn url_source(url: impl Into<String>) -> AssetSource;
pub fn url_source_with_persistence(url: impl Into<String>, cache_dir: &Path) -> AssetSource;
```

### Trait 扩展

```rust
pub trait AssetCacheExt {
    fn load_asset_from_url<T: Asset>(
        &self,
        url: &str,
        cache_dir: Option<&Path>
    ) -> AssetState<T>;
}
```

### 功能特性

- **URL 资源加载**：从 HTTP/HTTPS URL 加载资源
- **持久化缓存**：可选地将资源保存到本地磁盘
- **缓存键管理**：基于 URL 哈希生成唯一缓存文件名
- **异步支持**：完全异步的资源获取
- **跨平台**：支持 Native 和 WASM 平台

## 关键依赖与配置

### 依赖项

- `warpui_core`：核心 UI 框架（AssetCache 基础类型）
- `reqwest`：HTTP 客户端
- `async-fs`：异步文件系统操作
- `bytes`：高效字节处理

### 配置要求

- 缓存目录需要有写权限
- URL 必须是有效的 HTTP/HTTPS 地址
- WASM 平台不支持持久化缓存

## 数据模型

### 资源类型

- **UrlAssetWithoutPersistence**：不持久化的 URL 资源命名空间
- **UrlAssetWithPersistence**：持久化的 URL 资源命名空间

### 缓存策略

- **内存优先**：首次加载后缓存在内存中
- **磁盘缓存**：可选地写入磁盘供下次使用
- **缓存键**：使用 URL 的哈希值作为文件名
- **文件名安全**：使用十六进制哈希避免文件系统问题

## 测试与质量

### 测试覆盖

- ⚠️ 无公开单元测试
- 依赖集成测试验证功能

### 代码质量

- 使用 `anyhow` 进行错误处理
- 异步操作使用 `async-compat` 适配不同运行时
- 日志记录用于调试和监控

## 常见问题 (FAQ)

### Q: 为什么缓存文件名是哈希值？

A: 为了确保文件名安全且唯一，避免特殊字符和长度限制问题。

### Q: 如何清除缓存？

A: 删除 `cache_dir` 目录中的所有文件。

### Q: WASM 平台支持吗？

A: 支持 URL 加载，但不支持持久化缓存。

### Q: 如何处理网络错误？

A: 使用 `anyhow::Result` 返回错误，调用方应处理网络失败情况。

## 相关文件清单

- `src/lib.rs`：主要实现和导出

## 变更记录

### 2026-05-01 (23:18)

- ✅ 初始化模块文档
- ✅ 添加资源加载和缓存策略说明
- ✅ 添加跨平台支持说明
