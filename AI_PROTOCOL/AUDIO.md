# AUDIO

这个文件是 `crates/audio` 的 AI 规则。

`crates/audio` 是音频基础层。

它定义声音来源、空间音频数据、播放设置、音频分组和播放请求。它不定义具体游戏对象使用哪些声音。

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

## Source 规则

- `source/sample.rs` 表示 wav、ogg、flac、mp3 等文件型声音。
- `source/procedural.rs` 表示程序生成声音，例如发动机、风、噪声、纯音。
- 具体资源名和具体对象声音配置不放在 `audio`。
- 复杂 DSP 或合成器 runtime 以后可以接第三方库；第一版只定义边界。

## Spatial 规则

- 音频可以有 2D 和 3D 空间数据。
- 2D / 3D 不拆成两个 crate。
- `spatial/audio_2d.rs` 放 2D 空间音频数据。
- `spatial/audio_3d.rs` 放 3D 空间音频数据。

## 边界规则

- `audio` 定义“声音怎么表达和播放”。
- `prefab` 或未来 `content` 定义“这个对象使用哪些声音”。
- `assets/audio` 放真实音频文件。
- 不写具体 `player_attack.wav`、`level_bgm`、`engine_loop` 配置。
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
