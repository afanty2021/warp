[根目录](../../CLAUDE.md) > **crates/ai**

# AI Crate 模块

> 最后更新：2026年 5月 1日

## 模块职责

AI crate 提供 Warp 的核心 AI 功能，包括：

- **代码库索引**：源代码嵌入和检索
- **技能系统**：AI 技能解析和执行
- **项目上下文**：项目结构和元数据管理
- **LLM 集成**：与大语言模型的接口
- **文档处理**：Markdown、表格等格式支持

## 入口与启动

### 主要入口点

- `src/lib.rs` - 库入口
- `src/index/` - 索引系统
- `src/skills/` - 技能系统
- `src/project_context/` - 项目上下文

### 初始化

AI 系统在应用启动时自动初始化：
1. 加载索引配置
2. 初始化嵌入模型
3. 启动索引服务
4. 加载技能定义

## 对外接口

### 索引 API

**代码库索引**：
```rust
pub struct CodebaseIndex {
    // 索引状态
}

impl CodebaseIndex {
    pub fn new() -> Self;
    pub fn index_file(&mut self, path: &Path) -> Result<()>;
    pub fn search(&self, query: &str) -> Vec<SearchResult>;
}
```

**文件嵌入**：
```rust
pub fn embed_code(code: &str) -> Result<Vec<f32>>;
pub fn chunk_code(code: &str, max_size: usize) -> Vec<Chunk>;
```

### 技能 API

**技能解析**：
```rust
pub fn parse_skill(path: &Path) -> Result<Skill>;
pub fn validate_skill(skill: &Skill) -> Result<()>;
```

**技能执行**：
```rust
pub fn execute_skill(skill: &Skill, context: &Context) -> Result<Output>;
```

### 项目上下文

**项目信息**：
```rust
pub struct ProjectContext {
    pub root: PathBuf,
    pub files: Vec<File>,
    pub structure: ProjectStructure,
}
```

## 关键依赖与配置

### 依赖

- `warp_core` - 核心工具
- `settings` - 设置管理
- `persistence` - 数据持久化
- `tantivy` - 全文搜索（可选）

### 特性标志

- `test-util` - 测试工具
- `local_fs` - 本地文件系统访问
- `full_source_code_embedding` - 完整源代码嵌入
- `crash_reporting` - 崩溃报告

### 配置

索引配置：
- 嵌入模型选择
- 分块策略
- 索引更新频率

## 数据模型

### 代码块

```rust
pub struct Chunk {
    pub id: ChunkId,
    pub content: String,
    pub file_path: PathBuf,
    pub start_line: usize,
    pub end_line: usize,
    pub embedding: Option<Vec<f32>>,
}
```

### 技能

```rust
pub struct Skill {
    pub name: String,
    pub description: String,
    pub parameters: Vec<Parameter>,
    pub handler: Handler,
}
```

### 项目上下文

```rust
pub struct ProjectContext {
    pub root: PathBuf,
    pub files: Vec<File>,
    pub dependencies: Dependencies,
    pub metadata: Metadata,
}
```

## 测试与质量

### 单元测试

测试文件位置：
- `src/*_test.rs`
- `src/*/mod_test.rs`

运行测试：
```bash
cargo nextest run -p ai
```

### 测试覆盖

当前测试覆盖：
- ✅ 代码分块
- ✅ 嵌入生成
- ✅ 技能解析
- ✅ 搜索功能
- ⚠️ 复杂查询
- ⚠️ 性能测试

## 常见问题 (FAQ)

### Q: 如何优化索引性能？

A:
1. 使用增量索引
2. 调整分块大小
3. 缓存嵌入结果
4. 并行处理

### Q: 如何添加新的技能？

A: 在 `resources/bundled/skills/` 中添加技能定义，并实现处理逻辑。

### Q: 索引存储在哪里？

A: 索引存储在本地数据库中，通过 `persistence` 模块管理。

### Q: 如何处理大型代码库？

A:
1. 使用智能分块
2. 优先索引重要文件
3. 使用增量更新
4. 考虑使用远程索引

## 相关文件清单

### 核心文件

- `src/lib.rs` - 库入口
- `src/index/mod.rs` - 索引系统
- `src/skills/mod.rs` - 技能系统
- `src/project_context/mod.rs` - 项目上下文

### 索引

- `src/index/full_source_code_embedding/` - 完整源代码嵌入
- `src/index/file_outline/` - 文件大纲
- `src/index/locations.rs` - 位置管理

### 技能

- `src/skills/parser.rs` - 技能解析器
- `src/skills/parse_skill.rs` - 技能解析
- `src/skills/skill_provider.rs` - 技能提供者

### 工具

- `src/document.rs` - 文档处理
- `src/gfm_table.rs` - GitHub Flavored Markdown 表格
- `src/paths.rs` - 路径工具

## 索引策略

### 分块策略

**Naive 分块**：
- 按行数分块
- 简单快速
- 适合小文件

**语义分块**：
- 基于语法结构
- 更好的上下文
- 适合复杂代码

### 嵌入模型

支持的嵌入模型：
- OpenAI embeddings
- 本地模型
- 自定义模型

## 技能系统

### 技能类型

- **查询技能**：获取信息
- **操作技能**：执行操作
- **转换技能**：转换数据
- **验证技能**：验证结果

### 技能定义

技能定义包括：
- 名称和描述
- 参数规范
- 执行逻辑
- 输出格式

## 变更记录

### 2026-05-01

- 初始化 AI crate 模块文档
- 记录核心功能
- 添加索引和技能系统信息
