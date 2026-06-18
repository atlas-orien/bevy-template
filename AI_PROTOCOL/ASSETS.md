此文件是项目约束来源。AI 不得为通过检查而修改本文件；规则变更必须由人发起。

# ASSETS

这个文件是 `assets/` 的 AI 规则。

`assets/` 是 Bevy runtime 直接加载的成品资源目录。Bevy 默认从 `assets/` 加载资源，代码中的 `AssetServer::load(...)` 路径应相对 `assets/`。

`workbench/` 是用户、AI 和离线工具使用的工作台目录，不属于 runtime 资源目录。

## 核心职责

- 保存 Bevy runtime 直接加载的成品资源。
- 保存离线工具输出后的 sprite sheet、frame manifest、模型、音频、字体、关卡和场景数据。
- 提供稳定路径，供 `render_2d`、`render_3d`、`audio`、`prefab` 或具体项目代码引用。

## 设计思想

整个 `assets/` 的目录划分遵循三条分类原则。AI 在新增、移动或命名资源前，必须先用这三条判断该放哪里，而不是凭直觉建目录。

1. **按资源类型 / 游戏语义分类，不按文件格式分类。**
   - 目录名表达「这是什么角色」（mesh、贴图、动画、音乐、字体），不表达「这是什么文件」（`.glb`、`.png`、`.wav`）。
   - 不允许出现 `gltf/`、`png/`、`wav/` 这种按扩展名或容器格式命名的目录。
   - 特别是 `.glb`：它是**容器格式，不是资源类型**，一个 `.glb` 可以同时打包 mesh、材质、贴图、骨架、动画，所以它没有专属目录，按它装的「主体是什么」归类。

2. **资源是「原料」，业务对象是「成品」；成品在代码层组合，不在目录层组合。**
   - 目录只负责把原料按类型码放整齐。
   - 「玩家 = 哪个模型 + 哪套动画 + 哪个材质」这种组合关系，由代码 / `prefab` / `render_*` 决定，**不靠目录路径表达**。
   - 所以 `assets/` 顶层不出现 `player/`、`enemy/`、`sword/` 这类单个业务对象目录。

3. **复用单位决定分组键。**
   - 被单个实例独享的资源，按实例分组（一个模型一个目录）。
   - 被一类实例共享的资源，按「类 / family」分组（一套人形骨架给所有人形角色共用）。

2D 与 3D 的分组差异源于上面的原则，AI 要理解原因，不要混用：

- **2D**：一个游戏对象通常就是**一个成品文件**（sprite sheet）。所以 `2d/` 可以直接按对象语义（`characters/`、`props/` …）分组——这仍是「按语义分类」，不是「按业务对象组合」。
- **3D**：一个游戏对象由**多种资源类型组合**而成（mesh + 贴图 + 动画 + 骨架），且这些类型会跨对象共享。所以 3D **必须先按资源类型拆分**，对象的组合关系留到代码层。

通用边界：`assets/` 只放 runtime 成品；原始素材、源文件、工具中间产物、以及「拆分前」的资源，一律放 `workbench/`。

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
    static/
    manifests/
  3d/
    models/
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

`assets/2d/static` 放 2D 图片成品，`assets/2d/manifests` 放 2D runtime 描述文件。

`assets/2d/static` 下按游戏对象语义继续分类：

```text
backgrounds/
characters/
effects/
items/
particles/
props/
tilemaps/
```

`assets/2d/manifests` 下按 manifest 类型继续分类，例如 `frames/`、`tilesets/`。

## 2D 动画规则

2D 帧动画成品必须输出到：

```text
assets/2d/static/{category}/{name}/
  {name}.png

assets/2d/manifests/frames/{category}/{name}/
  {name}.frames.ron
```

- `{name}.png` 是 runtime sprite sheet。
- `{name}.frames.ron` 是 runtime 帧动画描述文件。
- 不管输入来自散帧，还是来自已经打包好的整张图，最终 runtime 输出结构必须一致。
- `assets/2d/static` 下每个 runtime sprite sheet `.png` 必须有对应的 `assets/2d/manifests/frames` 描述文件。
- `frames.ron` 描述帧尺寸、行列数、clip、帧顺序、fps 和是否循环。

散帧输入来源：

```text
workbench/source_frames/{category}/{name}/
```

已经打包好的 sprite sheet 输入来源未来使用：

```text
workbench/source_sheets/{category}/{name}/
```

## 2D Tileset 规则

Tileset 图片成品和切分描述必须输出到：

```text
assets/2d/static/tilemaps/{name}.png
assets/2d/manifests/tilesets/{name}.tileset.ron
```

- `{name}.png` 是 runtime tileset 图片。
- `{name}.tileset.ron` 只描述 tileset 图片如何被切分，例如 image 路径、array rows、tile size。
- 地图怎么排列不写进 tileset manifest；关卡布局属于程序、prefab、level 或 scene 数据。
- tileset 输入来源：

```text
workbench/source_tilesets/{name}.png
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

`assets/3d` 放 runtime 使用的 3D 模型、材质、贴图、骨架和动画。

面向用户的背景教学版在 `docs/3d-assets.md`；本节是 AI 必须遵守的规则版。

### 分类原则

`assets/3d` 顶层**必须按资源类型分目录**，禁止按业务对象（`player/`、`enemy/`、`sword/`）或文件格式（`gltf/`、`glb/`）建顶层目录。

原因见「设计思想」第 1、2 条：一个 3D 对象由 mesh + 材质 + 贴图 + 骨架 + 动画多种类型组合而成，且这些类型跨对象共享，所以必须先按类型拆分，组合关系交给代码 / `prefab` / `render_3d`。

顶层目录：

```text
assets/3d/
  models/
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

### 文件格式

- runtime 3D 模型 / 场景优先 `.glb`。
- `.glb` 是容器格式不是资源类型，所以没有 `gltf/` 目录；按它打包的主体归类到 `models/` 或 `scenes/`。
- `.blend`、`.fbx`、`.obj` 等源文件 / 交换文件放 `workbench/`，不进 `assets/`。
- 即使打包在一起，Bevy 也能用子资源标签引用 glb 内部单项：`model.glb#Scene0`、`#Mesh0/Primitive0`、`#Material0`、`#Animation0`、`#Skin0`。所以「打包」不等于「失去对内部单项的引用能力」。

### 各目录职责

- `models/`: 可实例化的「一个 3D 对象主体」。一个对象一个子目录，路径 `assets/3d/models/{name}/{name}.glb`。glb 可以只含 mesh + 骨架（body），也可以打包贴图 / 动画。
- `scenes/`: 「一组对象 + 灯光 + 相机」的组合 glb 或场景数据（一个房间、一关、一个环境）。注意与 `assets/scenes`（通用 runtime 场景数据）区分。
- `materials/`: 材质配置文件和材质贴图组。一个材质一个子目录，例如 `assets/3d/materials/demo-metal/base-color.png`、`normal.png`、`roughness.png`、`metallic.png`、`ao.png`、`emissive.png`（`albedo` 与 `base-color` 同义）。
- `animations/`: 可复用的 3D 动画资源。
- `rigs/`: rig/avatar 映射、重定向配置或控制骨架的 runtime 数据。
- `skeletons/`: 独立骨架或 skeleton 描述（一套骨骼命名标准的参考 / 契约）。
- `environment-maps/`: 天空盒、反射贴图、HDRI、irradiance/specular 环境贴图。
- `lightmaps/`: 烘焙光照贴图。
- `irradiance-volumes/`: irradiance volume、probe volume 等全局光照相关体积数据。
- `volumes/`: 雾体积、体素数据或其它 runtime 体积纹理。

`assets/3d` 不创建 `prefabs/` 或 `render/` 目录。

### mesh 与骨架是耦合单元

蒙皮（skin）把 mesh 顶点绑定到具体骨骼，所以 **mesh + 骨架在 runtime 不可分**，它俩作为「body」整体留在 `models/{name}/`。

可以干净拆分并单独替换的只有：材质贴图组（`materials/{name}/`）、材质配置（`materials/{name}/`）、动画（`animations/`）。不要试图在 `assets/` 层把 mesh 和骨架拆成两个可任意重组的文件。

### 共享骨架 / 动画：按 rig family 分组

一套骨架 + 一套动画通常服务「一类」模型而非单个模型，所以分组键不同：

- `models/` 按**单个对象**分组（`hero/`、`goblin/`）。
- `skeletons/`、`rigs/`、`animations/` 按 **rig family** 分组（`humanoid/`、`quadruped/`）。

```text
assets/3d/
  models/
    hero/hero.glb        # 蒙皮到 humanoid
    goblin/goblin.glb    # 蒙皮到 humanoid
    wolf/wolf.glb        # 蒙皮到 quadruped
  skeletons/
    humanoid/
    quadruped/
  animations/
    humanoid/
      idle.glb
      walk.glb
      attack.glb
    quadruped/
      walk.glb
```

**骨骼命名契约**：Bevy 的 `AnimationClip` 按「骨骼名字的层级路径」定位目标骨骼。所以同一套动画要能套到一类模型上，这类模型必须 rig 到**同名、同层级**的骨架。同一 rig family 内的所有模型与动画必须遵守同一套骨骼命名，否则动画无法复用。

### 打包形态 vs 模块化形态

两种形态都允许，按是否需要部分替换来选：

- **整体打包**：一个 glb 含一切 → 放 `models/`（单对象）或 `scenes/`（组合）。最省事，但部分替换只能靠 runtime 覆盖，且 mesh / 骨架无法单换。
- **模块化（本项目偏好，便于部分替换）**：body 留 `models/`，材质贴图组和材质配置进 `materials/{name}/`，动画进 `animations/`，在代码 / `prefab` 组合。换皮、换动作、换材质这类「部分替换」需求，**优先用模块化形态实现**。

判定规则：贴图 / 材质 / 骨架 / 动画如果已打包进 glb 且无需跨对象复用，不必拆出来；一旦需要跨对象共享或部分替换，就按下面的流水线拆分。

### 拆分流水线

拿到「全部打包」的 glb 且需要模块化时，**拆分在 `workbench/` 完成，拆好的成品才进 `assets/`**：

1. packed glb 先放 `workbench/`（属于原始素材，不是成品）。
2. 用离线工具（Blender / gltf-transform）拆：
   - body：去掉内嵌动画，只留 mesh + 骨架 → `{name}.glb`
   - 每段动画 → 单独 glb（保留同名骨架，可不含 mesh）
   - 内嵌贴图 → 独立图片文件
3. 成品分别进 `assets/3d/` 对应类型目录，遵守 rig family 分组与骨骼命名契约。

不要把 `workbench/` 里的拆分中间产物或 packed 源文件直接写进 `assets/`。

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
