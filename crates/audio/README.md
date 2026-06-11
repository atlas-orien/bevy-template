# audio

`audio` 是音频基础层。

它定义“声音怎么表达和播放”，不定义某个具体对象使用哪些声音。

当前已经打通最小播放链路：外部发送 `PlayAudioRequest::sample("audio/sfx/hit.ogg")`，`AudioFoundationPlugin` 会用 Bevy 从 `assets/` 加载并播放这个文件。

## 当前结构

- `source`: 声音来源，分为 sample 和 procedural。
- `spatial`: 空间音频数据，分为 2D 和 3D。
- `playback.rs`: 播放设置。
- `bus.rs`: 音频分组。
- `volume.rs`: 音量和静音。
- `request.rs`: 播放/停止请求数据。

## source

- `AudioSampleSource`: 文件型声音，例如 wav、ogg、flac、mp3。
- `ProceduralAudioSource`: 程序生成声音，例如发动机、风、噪声、纯音。

具体的 `object_action.wav`、`engine_loop`、`level_bgm` 不放在这里，放到 prefab 或未来 content。

## 播放固定资源

播放 runtime 音频资源时，路径相对 `assets/`：

```rust
use audio::request::PlayAudioRequest;
use bevy::prelude::*;

fn play_sound(mut requests: MessageWriter<PlayAudioRequest>) {
    requests.write(PlayAudioRequest::sample("audio/sfx/hit.ogg"));
}
```

循环播放：

```rust
use audio::playback::AudioPlaybackSettings;
use audio::request::PlayAudioRequest;
use bevy::prelude::*;

fn play_music(mut requests: MessageWriter<PlayAudioRequest>) {
    requests.write(
        PlayAudioRequest::sample("audio/music/theme.ogg")
            .with_settings(AudioPlaybackSettings::looping().with_volume(0.6)),
    );
}
```

`PlayAudioRequest` 只表达“播放哪个已有资源”。哪个 prefab、关卡或对象在什么时机播放哪个资源，不写在 `audio`。

## spatial

- `AudioPosition2d`: 2D 空间音频位置。
- `AudioPosition3d`: 3D 空间音频位置。

音频可以区分 2D / 3D，但不拆成两个 crate。

## 不应该放这里

- 不写具体游戏音效配置。
- 不写某个 prefab 使用哪个音频资源。
- 不写 gameplay 规则。
- 不读取输入。
- 不定义渲染、物理或 ECS 核心组件。
