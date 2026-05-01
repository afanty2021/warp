# Voice Input 模块

[根目录](../../CLAUDE.md) > [crates](../) > **voice_input**

## 模块职责

提供语音输入功能，支持麦克风录音、音频重采样、WAV 转换和会话管理。用于语音到文本转换和 AI 语音交互。

## 入口与启动

- **主要入口**：`crates/voice_input/src/lib.rs`
- **核心实体**：`VoiceInput`（SingletonEntity）
- **会话类型**：`VoiceSession`

## 对外接口

### 核心类型

```rust
pub struct VoiceInput {
    state: VoiceInputState,
    pub should_suppress_new_feature_popup: bool,
    voice_session_start: Option<instant::Instant>,
}

pub struct VoiceSession {
    result_rx: oneshot::Receiver<VoiceSessionResult>,
}
```

### 状态机

```rust
pub enum VoiceInputState {
    Idle,
    Listening {
        stream: cpal::Stream,
        chunk_size: usize,
        enabled_from: VoiceInputToggledFrom,
        resampler: Arc<Mutex<SincFixedIn<f32>>>,
        resampled: Arc<Mutex<Vec<f32>>>,
        result_tx: Option<oneshot::Sender<VoiceSessionResult>>,
    },
    Transcribing,
}
```

### 主要方法

```rust
impl VoiceInput {
    // 开始录音
    pub fn start_listening(
        &mut self,
        ctx: &mut ModelContext<Self>,
        source: VoiceInputToggledFrom,
    ) -> Result<VoiceSession, StartListeningError>;

    // 停止录音并转换
    pub fn stop_listening(&mut self, ctx: &mut ModelContext<Self>) -> Result<(), anyhow::Error>;

    // 中止录音
    pub fn abort_listening(&mut self);

    // 状态查询
    pub fn is_listening(&self) -> bool;
    pub fn is_transcribing(&self) -> bool;
    pub fn is_active(&self) -> bool;
}
```

### 功能特性

- **麦克风录音**：使用 cpal 库跨平台音频捕获
- **音频重采样**：自动重采样到 16kHz（语音输入标准）
- **WAV 转换**：自动转换为 Base64 编码的 WAV 格式
- **会话管理**：异步返回录音结果
- **权限检查**：麦克风访问权限验证
- **超时处理**：6 分钟超时限制
- **多声道混音**：自动混音为单声道

## 关键依赖与配置

### 依赖项

- `cpal`：跨平台音频库
- `rubato`：音频重采样
- `hound`：WAV 文件写入
- `base64`：Base64 编码

### 配置要求

- **目标采样率**：16000Hz（语音输入标准）
- **声道数**：单声道（mono）
- **块大小**：512 samples（可调整）
- **超时**：6 分钟
- **麦克风权限**：需要用户授权

## 数据模型

### 录音结果

```rust
pub enum VoiceSessionResult {
    Audio {
        wav_base64: String,        // Base64 编码的 WAV 数据
        session_duration_ms: u64,  // 会话时长（毫秒）
    },
    Aborted {
        session_duration_ms: Option<u64>,
    },
}
```

### 触发源

```rust
pub enum VoiceInputToggledFrom {
    Button,           // 按钮触发
    Key { state: KeyState },  // 键盘快捷键触发
}
```

### 错误类型

```rust
pub enum StartListeningError {
    AlreadyRunning,    // 已在录音
    AccessDenied,      // 麦克风权限被拒绝
    Other(anyhow::Error),  // 其他错误
}
```

## 测试与质量

### 测试覆盖

- ⚠️ 无公开单元测试
- 依赖手动测试和集成测试

### 代码质量

- 异步音频处理
- 资源管理（流、内存）
- 错误处理和日志记录

## 常见问题 (FAQ)

### Q: 如何获取录音结果？

A: 使用 `VoiceSession::await_result()` 等待异步结果。

### Q: 支持哪些平台？

A: 支持 cpal 的所有平台（macOS、Windows、Linux）。

### Q: 为什么是 16kHz？

A: 这是语音识别的标准采样率，如 Wispr。

### Q: 如何处理权限问题？

A: 检查 `MicrophoneAccessState`，提示用户授权。

### Q: 可以调整录音质量吗？

A: 可以修改 `TARGET_SAMPLE_RATE` 和 `DEFAULT_CHUNK_SIZE`。

### Q: 如何中止录音？

A: 调用 `abort_listening()` 而非 `stop_listening()`。

## 相关文件清单

- `src/lib.rs`：主要实现

## 变更记录

### 2026-05-01 (23:18)

- ✅ 初始化模块文档
- ✅ 添加状态机和方法说明
- ✅ 添加音频处理和配置文档
- ✅ 添加错误处理和会话管理说明
