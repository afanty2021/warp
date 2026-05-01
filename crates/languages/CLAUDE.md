# Languages - 语言支持模块

[根目录](../../CLAUDE.md) > [crates](../) > **languages**

> 最后更新：2026-05-01

## 模块职责

Languages 模块为 Warp 的编辑器功能提供了多语言语法高亮、自动缩进和符号解析支持。该模块集成了 [arborium](https://github.com/arborium/arborium) 库，为 32 种编程语言提供 tree-sitter 语法解析器和查询。

**核心功能**：
- 32 种编程语言的语法高亮
- 自动缩进计算
- 符号解析（用于代码导航）
- 括号配对
- 注释前缀检测
- 语言检测（基于文件名和扩展名）

## 入口与启动

### 主要函数

**语言查询**：
- `language_by_name(name: &str)` - 按名称获取语言
- `language_by_filename(path: &Path)` - 按文件名获取语言

### 使用示例

```rust
use languages::{language_by_name, language_by_filename};
use std::path::Path;

// 按名称获取
let rust = language_by_name("rust").unwrap();
println!("Rust indent unit: {:?}", rust.indent_unit);

// 按文件名获取
let path = Path::new("/src/main.rs");
let lang = language_by_filename(path).unwrap();
println!("Language: {}", lang.display_name());

// 使用语法高亮查询
let source = "fn main() { println!(\"Hello\"); }";
let mut cursor = QueryCursor::new();
let highlights = lang.highlight_query;
// 使用 cursor 和 highlights 进行语法高亮
```

## 对外接口

### 核心数据结构

**`Language`**：
```rust
pub struct Language {
    pub grammar: ParserGrammar,           // Tree-sitter 解析器
    pub highlight_query: Query,           // 语法高亮查询
    pub indents_query: Option<Query>,     // 自动缩进查询
    pub indent_unit: IndentUnit,          // 缩进单位（空格/Tab）
    pub comment_prefix: Option<String>,   // 注释前缀
    pub bracket_pairs: Vec<(char, char)>, // 括号配对
    pub symbols_query: Option<Query>,     // 符号解析查询
    pub display_name: String,             // 显示名称
}
```

### 语言查询接口

**`language_by_name`**：
```rust
pub fn language_by_name(name: &str) -> Option<Arc<Language>>
```
支持的语言别名：
- `go` → `golang`
- `bash`/`sh`/`zsh` → `shell`
- `js` → `javascript`
- `ts` → `typescript`
- `py` → `python`
- `rs` → `rust`
- `cs`/`c#` → `csharp`
- `c++` → `cpp`

**`language_by_filename`**：
```rust
pub fn language_by_filename(path: &Path) -> Option<Arc<Language>>
```
基于文件扩展名和特殊文件名检测语言。

## 关键依赖与配置

### 依赖项

**核心依赖**：
- `arborium` - Tree-sitter 语法解析器和查询
- `rust-embed` - 嵌入语法文件和查询
- `serde` / `serde_yaml` - 配置反序列化
- `lazy_static` - 语言注册表
- `warp_editor` - 编辑器类型集成

### 支持的语言

**32 种语言**：
```
rust, golang, yaml, python, javascript, jsx, typescript, tsx,
java, cpp, shell, csharp, html, css, c, json, hcl, lua, ruby,
php, toml, swift, kotlin, scala, powershell, elixir, sql,
starlark, objective-c, xml, vue, dockerfile
```

### 配置文件

**语法目录结构**：
```
grammars/
├── {language}/
│   ├── config.yaml       # 语言配置（缩进、注释、括号）
│   ├── indents.scm       # 缩进查询（可选）
│   └── identifiers.scm   # 符号查询（可选）
```

**config.yaml 示例**：
```yaml
display_name: "Rust"
indent_unit:
  Space: 4
comment_prefix: "//"
brackets:
  - { start: "(", end: ")" }
  - { start: "{", end: "}" }
  - { start: "[", end: "]" }
```

## 数据模型

### 语言注册表

**`LanguageRegistry`**：
- 单例模式（`lazy_static`）
- 缓存已加载的语言（`Arc<Language>`）
- 线程安全（`Mutex` 保护）

### 语言加载流程

1. 检查缓存 → 返回已加载的语言
2. 规范化语言名称 → arborium 名称
3. 从 arborium 获取语法
4. 加载 `config.yaml`
5. 从 arborium 获取高亮查询
6. 加载自定义缩进和符号查询
7. 创建 `Language` 实例并缓存

### 文件名检测

**特殊文件名**：
- `.bashrc`/`.bash_profile` → Shell
- `.zshrc`/`.zprofile` → Shell
- `Dockerfile`/`Containerfile` → Dockerfile
- `BUILD`/`WORKSPACE` → Starlark（Bazel）

**文件扩展名**：
支持 100+ 种文件扩展名映射到 32 种语言。

## 测试与质量

### 测试覆盖

✅ **单元测试**：3 个测试（在 `lib_tests.rs`）
- 语言名称规范化
- 文件扩展名映射
- 特殊文件名检测

### 测试策略

**运行测试**：
```bash
cargo test -p languages
```

**测试重点**：
1. 语言检测正确性
2. 文件名映射完整性
3. 语言加载无错误
4. 查询有效性

### 集成测试

建议在编辑器集成中验证：
- 语法高亮正确性
- 缩进计算准确性
- 符号解析功能
- 多语言文件混合编辑

## 常见问题 (FAQ)

### Q: 如何添加新语言支持？

**A**:
1. 确认 arborium 支持该语言
2. 在 `SUPPORTED_LANGUAGES` 数组中添加语言名
3. 在 `to_arborium_name()` 中添加名称映射
4. 在 `get_arborium_highlight_query()` 中添加高亮查询
5. 在 `language_by_filename()` 中添加文件扩展名映射
6. （可选）在 `grammars/` 目录添加自定义配置

### Q: 为什么有些语言没有缩进查询？

**A**: 缩进查询是可选的。如果没有 `indents.scm`，`Language.indents_query` 将为 `None`，编辑器应该使用默认缩进行为。

### Q: 如何调试语法高亮问题？

**A**:
1. 确认语言正确加载：`language_by_name("your_lang")`
2. 检查高亮查询是否有效：`lang.highlight_query`
3. 使用 tree-sitter CLI 测试查询：
   ```bash
   tree-sitter query test.scm test_file.rs
   ```
4. 查看 arborium 的查询定义

### Q: 文件名检测的优先级是什么？

**A**:
1. **特殊文件名**（如 `Dockerfile`）- 最高优先级
2. **文件扩展名**（如 `.rs`）- 次优先级
3. **未匹配** - 返回 `None`

### Q: 如何处理未知文件类型？

**A**:
```rust
match language_by_filename(path) {
    Some(lang) => {
        // 使用特定语言的语法高亮
    }
    None => {
        // 使用纯文本模式或尝试其他检测方法
    }
}
```

### Q: arborium 和自定义查询的关系？

**A**:
- **高亮查询**：优先使用 arborium 的内置查询
- **缩进查询**：从 `grammars/{lang}/indents.scm` 加载
- **符号查询**：从 `grammars/{lang}/identifiers.scm` 加载
- arborium 提供了 32 种语言的高质量查询，无需手动维护

### Q: 如何获取语言的括号配对规则？

**A**:
```rust
let lang = language_by_name("rust").unwrap();
for (open, close) in &lang.bracket_pairs {
    println!("{} {}", open, close);
}
// 输出：( ) { } [ ]
```

### Q: 支持多语言文件吗？

**A**: 当前实现每个文件返回单一语言。对于多语言文件（如 HTML + CSS + JS），建议：
1. 基于主扩展名选择语言
2. 使用注入语法（tree-sitter 的 `language-injection` 功能）
3. 在编辑器层处理语言切换

## 相关文件清单

### 核心文件

- `src/lib.rs` - 模块入口，语言注册表和查询函数

### 测试文件

- `src/lib_tests.rs` - 单元测试

### 资源文件

- `grammars/` - 语法文件和查询目录
  - `{language}/config.yaml` - 语言配置
  - `{language}/indents.scm` - 缩进查询
  - `{language}/identifiers.scm` - 符号查询

## 变更记录

### 2026-05-01

- ✅ 创建模块文档
- ✅ 记录 32 种支持的语言
- ✅ 文档化语言检测逻辑
- ✅ 添加新语言支持指南

---

*本文档由 AI 自动生成和维护。如有问题或建议，请在 issue 中提出。*
