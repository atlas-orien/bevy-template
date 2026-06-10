# audio

`audio` 是音频基础层。

它定义“声音怎么表达和播放”，不定义某个具体对象使用哪些声音。

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
