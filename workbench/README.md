# Workbench

`workbench/` 是用户给 AI 和离线工具使用的工作台目录。

Bevy runtime 不直接加载这里的文件。最终给 Bevy 使用的资源必须输出到 `assets/`，最终游戏代码必须写到 `crates/`。

## 用法

- 把想法、需求、参考资料和待处理素材放在这里。
- AI 优先读取这里的内容来理解用户想做什么。
- 离线 CLI 可以读取这里的素材，生成 `assets/` 里的运行时资源。
- 不要在游戏代码里通过 `AssetServer` 加载 `workbench/` 下的文件。

## 当前目录

```text
workbench/
  source_frames/
```

`source_frames/` 是帧动画散图输入目录，对应输出目标：

```text
assets/2d/animated/
```

## source_frames 规则

源目录和目标目录必须使用相同的分类和名称：

```text
workbench/source_frames/{category}/{name}/
assets/2d/animated/{category}/{name}/
```

例如：

```text
workbench/source_frames/characters/player/
assets/2d/animated/characters/player/
```

散帧文件使用这个命名格式：

```text
{clip_name}_{frame_number}.png
```

例如：

```text
idle_down_000.png
idle_down_001.png
idle_down_002.png
run_down_000.png
run_down_001.png
run_down_002.png
```

打包后目标目录只放运行时需要的文件：

```text
assets/2d/animated/characters/player/player.png
assets/2d/animated/characters/player/player.frames.ron
```

`player.png` 是打包后的规则网格 sprite sheet。

`player.frames.ron` 是帧动画描述文件，记录图片路径、帧尺寸、行列数、clip 名称、帧 index、fps 和是否循环。

当前打包命令：

```sh
cargo run -p xtask -- pack-frame characters/player
```

其中 `characters/player` 会自动映射：

```text
workbench/source_frames/characters/player/
assets/2d/animated/characters/player/
```

## 命名

- 文件和目录使用 lowercase kebab-case。
- 帧编号使用补零数字，例如 `000`、`001`、`002`。
- 同一个 `{name}` 目录里的散帧应该使用同一帧尺寸。
- 第一版只支持 PNG 散帧。
