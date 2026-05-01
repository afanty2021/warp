# Input Classifier 模块

[根目录](../../CLAUDE.md) > [crates](../) > **input_classifier**

## 模块职责

提供用户输入分类功能，区分 Shell 命令和 AI 自然语言查询。支持基于启发式规则、FastText 和 ONNX 模型的多种分类策略。

## 入口与启动

- **主要入口**：`crates/input_classifier/src/lib.rs`
- **分类器类型**：`HeuristicClassifier`、`FasttextClassifier`、`OnnxClassifier`

## 对外接口

### 核心 Trait

```rust
#[async_trait]
pub trait InputClassifier: 'static + Send + Sync {
    async fn detect_input_type(
        &self,
        input: ParsedTokensSnapshot,
        context: &Context,
    ) -> InputType;

    async fn classify_input(
        &self,
        input: ParsedTokensSnapshot,
        context: &Context,
    ) -> anyhow::Result<ClassificationResult>;
}
```

### 输入类型

```rust
pub enum InputType {
    Shell,
    AI,
}
```

### 分类结果

```rust
pub struct ClassificationResult {
    p_shell: f32,  // Shell 命令概率
    p_ai: f32,     // AI 查询概率
}
```

### 功能特性

- **多分类器支持**：启发式、FastText、ONNX
- **异步分类**：支持异步推理
- **上下文感知**：考虑当前输入类型和 Agent 后续查询
- **概率输出**：提供置信度分数
- **特征标志**：通过 feature gates 启用不同分类器

## 关键依赖与配置

### 依赖项

- `warp_completer`：输入令牌解析
- `async-trait`：异步 trait 支持

### 特性标志

- `fasttext`：启用 FastText 分类器
- `onnx`：启用 ONNX 分类器

### 配置要求

- FastText 需要模型文件
- ONNX 需要模型文件和运行时
- 启发式分类器无额外依赖

## 数据模型

### 分类上下文

```rust
pub struct Context {
    pub current_input_type: InputType,
    pub is_agent_follow_up: bool,
}
```

### 分类器实现

- **HeuristicClassifier**：基于规则的启发式分类
- **FasttextClassifier**：基于 FastText 模型
- **OnnxClassifier**：基于 ONNX 模型

## 测试与质量

### 测试覆盖

- ✅ 单元测试：`crates/input_classifier/src/parser_tests.rs`
- ✅ 测试工具：`src/test_utils.rs`
- 测试内容：
  - 令牌解析
  - 分类器推理
  - 边界情况处理

### 代码质量

- 使用 `anyhow` 进行错误处理
- 异步接口设计
- 清晰的类型定义

## 常见问题 (FAQ)

### Q: 如何选择合适的分类器？

A: 启发式最快但准确率较低，FastText 平衡，ONNX 最准确但最慢。

### Q: 如何训练自定义模型？

A: 使用 FastText 或 ONNX 训练模型，替换默认模型文件。

### Q: 分类器性能如何？

A: 启发式 < 1ms，FastText ~5ms，ONNX ~10ms（取决于硬件）。

### Q: 如何处理 Agent 后续查询？

A: 设置 `context.is_agent_follow_up = true`。

## 相关文件清单

- `src/lib.rs`：核心 trait 和类型定义
- `src/heuristic_classifier.rs`：启发式分类器
- `src/fasttext.rs`：FastText 分类器（feature-gated）
- `src/onnx.rs`：ONNX 分类器（feature-gated）
- `src/parser.rs`：输入解析器
- `src/input_type.rs`：输入类型定义
- `src/parser_tests.rs`：解析器测试
- `src/test_utils.rs`：测试工具

## 变更记录

### 2026-05-01 (23:18)

- ✅ 初始化模块文档
- ✅ 添加分类器类型和接口说明
- ✅ 添加特性标志和配置说明
- ✅ 添加性能和测试信息
