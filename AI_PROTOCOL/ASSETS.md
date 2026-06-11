# ASSETS

这个文件是 `assets/` 的 AI 规则。

`assets/` 是 Bevy runtime 直接加载的成品资源目录。Bevy 默认从 `assets/` 加载资源，代码中的 `AssetServer::load(...)` 路径应相对 `assets/`。

`workbench/` 是用户、AI 和离线工具使用的工作台目录，不属于 runtime 资源目录。

## 核心职责

- 保存 Bevy runtime 直接加载的成品资源。
- 保存离线工具输出后的 sprite sheet、frame manifest、模型、音频、字体、关卡和场景数据。
- 提供稳定路径，供 `render_2d`、`render_3d`、`audio`、`prefab` 或具体项目代码引用。

## 边界规则

- `assets/README.md` 面向用户，只保留目录索引和最少规则；AI 必须以本协议的详细规则为准。
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
    models/
    textures/
    materials/
    animations/
    rigs/
    skeletons/
    scenes/
    environment-maps/
    lightmaps/
    irradiance-volumes/
    volumes/
  ui/
  audio/
  fonts/
  shaders/
    2d/
    3d/
    ui/
    post-process/
    includes/
  cursors/
  branding/
  data/
  levels/
  scenes/
  platform/
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

`assets/3d` 放 runtime 使用的 3D 模型、材质、贴图、骨骼和动画。

3D 资源不按角色、物品、场景这类业务对象做顶层分类。顶层目录必须按资源类型划分。

业务对象如何组合 model、material、texture、animation、rig，属于代码、`render_3d` 配置或 prefab 层，不属于 `assets/3d` 的目录职责。

优先使用 `.glb` 作为 runtime 3D 模型/场景文件格式。`.blend`、`.fbx`、`.obj` 等源文件或交换文件放到 `workbench/`，不要直接放进 `assets/`。

顶层目录：

```text
assets/3d/
  models/
  textures/
  materials/
  animations/
  rigs/
  skeletons/
  scenes/
  environment-maps/
  lightmaps/
  irradiance-volumes/
  volumes/
```

`models/` 放可实例化的 3D 模型主体。模型通常包含 mesh，也可能包含材质、贴图、骨骼和动画。

```text
assets/3d/models/{name}/
  {name}.glb
```

`textures/` 放 3D 材质使用的贴图，例如：

```text
albedo.png
base-color.png
normal.png
roughness.png
metallic.png
ao.png
emissive.png
```

- `materials/`: 材质配置文件或材质相关 runtime 数据。
- `animations/`: 可复用的 3D 动画资源。
- `rigs/`: rig/avatar 映射、重定向配置或控制骨架的 runtime 数据。
- `skeletons/`: 独立骨架或 skeleton 描述。
- `scenes/`: 完整 3D scene、场景模型或环境组合。
- `environment-maps/`: 天空盒、反射贴图、HDRI、irradiance/specular 环境贴图。
- `lightmaps/`: 烘焙光照贴图。
- `irradiance-volumes/`: irradiance volume、probe volume 等全局光照相关体积数据。
- `volumes/`: 雾体积、体素数据或其它 runtime 体积纹理。

贴图、材质、骨骼和动画如果已经打包进 `.glb`，不需要额外拆出来。只有需要复用、替换或单独管理时才拆到对应目录。

`assets/3d` 不创建 `prefabs/` 或 `render/` 目录。

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

## 其它 runtime 资源规则

- `assets/cursors`: 鼠标光标图片和光标相关 runtime 资源。
- `assets/branding`: 图标、启动图、logo 等品牌资源。
- `assets/data`: 表格、配置、规则数据等 runtime 数据文件。
- `assets/platform`: 平台特定 runtime 资源，例如 Android 图标资源。

不要照搬 Bevy 示例仓库里的 `docs/`、`external/` 这类维护目录。模板的 `assets/` 只保存 runtime 会加载的资源。

## Shader 规则

`assets/shaders` 放 WGSL、shader include 或 shader 配置。

shader 是 GPU 程序，也是 Bevy runtime 可以加载的资源文件。第一版不要求项目写自定义 shader；这里只预留高级表现目录。

标准目录：

```text
assets/shaders/
  2d/
  3d/
  ui/
  post-process/
  includes/
```

- `2d/`: sprite、2D material、tilemap、2D 特效用 shader。
- `3d/`: mesh、PBR 扩展、toon、water、foliage 等 3D shader。
- `ui/`: UI material、特殊按钮、面板效果等屏幕空间 UI shader。
- `post-process/`: 全屏后处理，例如描边、调色、bloom mask、CRT 效果。
- `includes/`: 公共 WGSL 片段，被其它 shader 引用，不直接作为效果使用。

## 验证要求

修改 `assets/`、`workbench/` 资源输入输出规则或本协议后必须运行：

```sh
cargo run -p xtask -- check
```
