# Assets

`assets/` 只放 Bevy runtime 直接加载的成品资源。

Bevy 默认会从 `assets/` 目录加载资源，所以代码里使用的路径应该相对
`assets/`：

```rust
asset_server.load("2d/animated/characters/name/name.png")
```

不要在 runtime 代码里加载 `workbench/`。`workbench/` 是用户、AI 和离线工具使用的工作台，用来放原始素材、参考资料和待处理输入。

## 基本规则

- 文件和目录使用 lowercase kebab-case。
- 路径要尽量稳定；Bevy handle 和 AI 生成的代码经常会引用这些路径。
- 优先按游戏语义分类，不按文件扩展名分类。
- 这里只放 runtime-ready 文件。
- 原始文件、下载参考、工具输入放到 `workbench/`。
- 第三方资源的来源和许可证记录写到 `docs/assets.md`。

## 目录结构

```text
assets/
  2d/
    animated/
      backgrounds/
      characters/
      effects/
      items/
      particles/
      props/
      tilemaps/
    static/
      backgrounds/
      characters/
      effects/
      items/
      particles/
      props/
      tilemaps/

  3d/
  ui/
  audio/
  fonts/
  levels/
  scenes/
```

## 2D 动画资源

`assets/2d/animated` 放 sprite sheet 和对应的帧动画描述文件。

每个动画资源使用独立目录：

```text
assets/2d/animated/{category}/{name}/
  {name}.png
  {name}.frames.ron
```

`{name}.png` 是 runtime 使用的 sprite sheet。

`{name}.frames.ron` 描述帧尺寸、行列数、clip、帧顺序、fps 和是否循环。

不管输入来自散帧，还是来自已经打包好的整张图，最终都应该输出成同一种 runtime 结构。

## 2D 静态资源

`assets/2d/static` 放世界空间里的非动画 2D 图片。

```text
assets/2d/static/{category}/{name}/
  {name}.png
```

UI 图片不放在这里。屏幕空间 UI 资源放到 `assets/ui`。

## UI

`assets/ui` 放屏幕空间视觉资源：

```text
assets/ui/icons/
assets/ui/images/
assets/ui/themes/
```

字体可以继续放在 `assets/fonts`，这样 UI 和世界空间文字都能共享。

## 3D

`assets/3d` 放 runtime 使用的模型文件，以及和模型绑定的 runtime 材质、贴图和动画。

等项目开始使用 3D 内容后，再按具体语义增加子目录。

## 音频

`assets/audio` 放 runtime 使用的音频文件。

建议项目里按需增加：

```text
assets/audio/music/
assets/audio/sfx/
assets/audio/voice/
assets/audio/ambience/
```

## 关卡和场景

- `assets/levels`: gameplay 关卡数据。
- `assets/scenes`: runtime 加载的场景数据。

这里只放 runtime 数据。设计说明、草稿和参考资料放到 `workbench/`。
