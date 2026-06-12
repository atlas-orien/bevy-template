此文件是项目约束来源。AI 不得为通过检查而修改本文件；规则变更必须由人发起。

# AUDIO

这个文件是 `crates/audio` 的 AI 规则。

`crates/audio` 是音频基础层。

它定义声音来源、空间音频数据、播放设置、音频分组和播放请求。它不定义具体游戏对象使用哪些声音。

第一阶段必须优先保证固定音频资源可以播放：外部发送 `PlayAudioRequest::sample("audio/sfx/hit.ogg")`，`AudioFoundationPlugin` 负责通过 Bevy 的 `AssetServer` 和 `AudioPlayer` 播放 `assets/` 下已有文件。

## 代码落点

- 声音来源：写到 `crates/audio/src/source`。
- 文件型声音来源：写到 `crates/audio/src/source/sample.rs`。
- 程序生成声音来源：写到 `crates/audio/src/source/procedural.rs`。
- 空间音频：写到 `crates/audio/src/spatial`。
- 2D 空间音频：写到 `crates/audio/src/spatial/audio_2d.rs`。
- 3D 空间音频：写到 `crates/audio/src/spatial/audio_3d.rs`。
- 播放设置：写到 `crates/audio/src/playback.rs`。
- 音频分组：写到 `crates/audio/src/bus.rs`。
- 音量和静音：写到 `crates/audio/src/volume.rs`。
- 播放/停止请求：写到 `crates/audio/src/request.rs`。
- request 到 Bevy 音频实体的最小播放 system：写到 `crates/audio/src/lib.rs`，以后复杂后再拆到专用模块。

## Source 规则

- `source/sample.rs` 表示 wav、ogg、flac、mp3 等文件型声音。
- `source/procedural.rs` 表示程序生成声音，例如发动机、风、噪声、纯音。
- 具体资源名和具体对象声音配置不放在 `audio`。
- 复杂 DSP 或合成器 runtime 以后可以接第三方库；第一版只定义边界。

## 固定资源播放规则

- 固定资源播放使用 `PlayAudioRequest::sample(path)`。
- `path` 必须是相对 `assets/` 的路径，例如 `audio/sfx/hit.ogg`。
- `AudioFoundationPlugin` 必须注册 `PlayAudioRequest` 和 `StopAudioRequest` message。
- `AudioFoundationPlugin` 必须消费 sample 请求，并生成 Bevy 音频播放实体。
- sample 播放使用 Bevy 的 `AudioPlayer`、`PlaybackSettings` 和 `AssetServer`，不要自己实现解码器。
- 一次性音效默认使用 Bevy 的 despawn 播放模式，避免播放结束后残留实体。
- 循环音频使用 `AudioPlaybackSettings::looping()`。
- 播放实体必须带 `AudioPlaybackId` 和 `AudioPlaybackBus`，方便以后停止、调音量、做 bus 管理。
- `ProceduralAudioSource` 可以保留类型边界，但第一阶段不需要真正播放。
- `AudioBus` 可以保留分组语义，但第一阶段不需要实现完整 mixer。

## Spatial 规则

- 音频可以有 2D 和 3D 空间数据。
- 2D / 3D 不拆成两个 crate。
- `spatial/audio_2d.rs` 放 2D 空间音频数据。
- `spatial/audio_3d.rs` 放 3D 空间音频数据。

## 边界规则

- `audio` 定义“声音怎么表达和播放”。
- `prefab` 或未来 `content` 定义“这个对象使用哪些声音”。
- `assets/audio` 放真实音频文件。
- 不写具体 `object_action.wav`、`level_bgm`、`engine_loop` 配置。
- 不写 gameplay 判定。
- 不读取输入。
- 不定义渲染、物理或核心 ECS 数据。
- 不依赖 `prefab`、`gameplay`、`external_runtime`、`render_2d`、`render_3d`、`physics`。

## 依赖规则

- `audio` 可以依赖 `bevy`。
- `audio` 必须依赖 `error`。

## 验证要求

修改 `crates/audio` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
