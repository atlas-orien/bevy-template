# ASSETS

这个文件是 `assets/` 的 AI 规则。

`assets/` 是 Bevy runtime 直接加载的成品资源目录。Bevy 默认从 `assets/` 加载资源，代码中的 `AssetServer::load(...)` 路径应相对 `assets/`。

`workbench/` 是用户、AI 和离线工具使用的工作台目录，不属于 runtime 资源目录。

## 核心职责

- 保存 Bevy runtime 直接加载的成品资源。
- 保存离线工具输出后的 sprite sheet、frame manifest、模型、音频、字体、关卡和场景数据。
- 提供稳定路径，供 `render_2d`、`render_3d`、`audio`、`prefab` 或具体项目代码引用。

## 边界规则

- `assets/` 只放 runtime-ready 文件。
- 原始素材、下载参考、用户说明、AI 输入和工具输入放到 `workbench/`。
- runtime 代码不要通过 `AssetServer` 加载 `workbench/` 下的文件。
- 不要把临时 demo 资源或验证资源写成模板默认内容。
- 第三方资源的来源、许可证和用途记录写到 `docs/assets.md`。
- 文件和目录使用 lowercase kebab-case。
- 路径要稳定；避免无意义重命名，因为 Bevy handle 和 AI 生成代码会引用这些路径。
- 优先按游戏语义分类，不按文件扩展名分类。

## 目录规则

标准目录：

```text
assets/
  2d/
    animated/
    static/
  3d/
  ui/
  audio/
  fonts/
  levels/
  scenes/
```

`assets/2d/animated` 和 `assets/2d/static` 下按游戏对象语义继续分类：

```text
backgrounds/
characters/
effects/
items/
particles/
props/
tilemaps/
```

## 2D 动画规则

2D 帧动画成品必须输出到：

```text
assets/2d/animated/{category}/{name}/
  {name}.png
  {name}.frames.ron
```

- `{name}.png` 是 runtime sprite sheet。
- `{name}.frames.ron` 是 runtime 帧动画描述文件。
- 不管输入来自散帧，还是来自已经打包好的整张图，最终 runtime 输出结构必须一致。
- `frames.ron` 描述帧尺寸、行列数、clip、帧顺序、fps 和是否循环。

散帧输入来源：

```text
workbench/source_frames/{category}/{name}/
```

已经打包好的 sprite sheet 输入来源未来使用：

```text
workbench/source_sheets/{category}/{name}/
```

如果打包好的 sprite sheet 没有描述文件，工具最多自动生成默认 clip 或草稿描述；动画语义分组仍需要用户或 AI 补充。

## 2D 静态规则

世界空间非动画 2D 图片放到：

```text
assets/2d/static/{category}/{name}/
  {name}.png
```

UI 图片不放到 `assets/2d/static`，而是放到 `assets/ui`。

## UI 规则

屏幕空间 UI 视觉资源放到 `assets/ui`。

推荐项目子目录：

```text
assets/ui/icons/
assets/ui/images/
assets/ui/themes/
```

字体默认放到 `assets/fonts`，方便 UI 和世界空间文字共享。

## 3D 规则

`assets/3d` 放 runtime 使用的 3D 模型、材质、贴图和动画。

项目开始使用 3D 后，再按具体语义增加子目录。

## Audio 规则

`assets/audio` 放 runtime 使用的音频文件。

推荐项目子目录：

```text
assets/audio/music/
assets/audio/sfx/
assets/audio/voice/
assets/audio/ambience/
```

## Levels / Scenes 规则

- `assets/levels`: gameplay 关卡数据。
- `assets/scenes`: runtime 加载的场景数据。

设计说明、草稿和参考资料不要放在这里，放到 `workbench/`。

## 验证要求

修改 `assets/`、`workbench/` 资源输入输出规则或本协议后必须运行：

```sh
cargo run -p xtask -- check
```
