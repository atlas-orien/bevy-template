此文件是项目约束来源。AI 不得为通过检查而修改本文件；规则变更必须由人发起。

# RENDER_2D

这个文件是 `crates/render_2d` 的 AI 规则。

`crates/render_2d` 是项目 2D 表现内容层。

它不是 `bevy_render` 的二次封装，也不是像 `physics` 那样默认不让用户修改的基础 facade。用户可以在这里写已经配置好的具体 2D 表现内容，`prefab` 直接组合这些高层表现 bundle、component、marker 或 plugin。

## 核心职责

- 用户配置的 2D 相机、屏幕、HUD、菜单、界面。
- 用户配置的 sprite、texture atlas、tilemap、2D mesh、2D material、视觉动画。
- 角色、物品、静物、背景、环境、特效、粒子、覆盖层、世界文字等具体项目表现。
- 根据上层传入的视觉组件、资源句柄和表现状态，把游戏世界显示成 2D 画面。
- 创建渲染专用 Entity、Component、Bundle、Resource 和视觉 system。

## Bevy 边界

- 绝对不要重写一次 `bevy_render`。
- 能直接使用 Bevy 类型时，直接使用 Bevy 类型。
- `Sprite`、`TextureAtlas`、`SpriteImageMode` 直接用 Bevy。
- `Transform`、`Visibility`、`Anchor` 直接用 Bevy。
- `Text2d`、`Text`、`Node`、`ImageNode` 直接用 Bevy。
- UI 的 `ZIndex`、`GlobalZIndex` 直接用 Bevy。
- Bevy UI root 通过 `UiTargetCamera` 绑定到具体 UI camera；默认 UI camera 使用 `IsDefaultUiCamera`。
- `render_2d` 不定义运行时 camera target 句柄；`UiCamera` 是用于实例化 UI camera 的 bundle，UI camera 和 UI root 的绑定由 gameplay 在实例化阶段完成。
- UI 节点本身不靠 `RenderLayers` 分层；世界 sprite、mesh、Text2d 等世界表现才使用 `RenderLayers`。
- 多 camera 叠加时使用 `Camera.order`，UI camera 的 order 必须高于默认 world camera。
- 可以把 Bevy 类型组合进项目自己的高层表现 bundle，例如角色表现 bundle、道具表现 bundle、tile layer bundle。
- 不新增只镜像 Bevy 字段的 facade，例如纯粹复制颜色、透明度、缩放、旋转、z-index 的 wrapper。

## 和 prefab 的关系

- `render_2d` 提供已经配置好的具体表现内容。
- `prefab` 直接使用 `render_2d` 提供的 bundle、component、marker 或 helper。
- `prefab` 不应该为了 2D 表现再重复配置一遍 sprite、动画、材质、tilemap、覆盖层等细节。
- 如果某个表现内容只属于某个具体游戏对象，可以先写在 `render_2d` 的对应分类目录，再由 `prefab` 组合。

## 代码落点

- `primitives`: 最小通用表现单元，供 capabilities/products 组合。
- `capabilities`: 较复杂的通用表现能力，可以带 plugin、system、runtime state。
- `products`: 具体游戏对象、画面、场景或 UI 表现，通常给 `prefab` 直接组合。
- `products` 对 `prefab` 暴露的公共入口应优先是命名的 public bundle struct；不要把最终公共产品 API 做成 `pub fn into_bundle(self) -> impl Bundle`。

`primitives`:

- `primitives/frame_animation`: sprite sheet、texture atlas、逐帧动画。
- `primitives/atlases`: 通用 texture atlas 表现 primitive，例如显示 atlas 中某一格。
- `primitives/camera`: 2D 相机基础能力和可直接实例化的 camera presets。
- `primitives/images`: 通用静态图片表现 primitive，例如单张图片或纯色图片块；不负责加载具体资源路径。
- `primitives/layers`: 通用 layer stack 和 parallax layer 能力。
- `primitives/text`: 世界空间文字，例如伤害数字、漂浮提示、角色头顶名字。
- `primitives/tilemap`: 通用 tile map primitive，例如基于 Bevy `TilemapChunk` 的 tile chunk layer。
- `primitives/camera/base.rs`: 共享 Bevy 2D camera bundle/config，只放 `Camera2d`、`Camera`、`RenderLayers`、`Transform` 等基础组合。
- `primitives/camera/presets`: 业务可直接选择的 camera preset，例如 fixed、follow、ui。
- `primitives/tilemap/chunk.rs`: 通用 `TilemapChunkLayer2d`，只负责把 tileset handle、chunk size、tile size、tile index 数据和 transform 组合成 Bevy tilemap chunk bundle。
- marker 不单独集中到 `markers.rs`。marker 应和它标记的具体对象、bundle、system 放在同一个模块里，例如 camera marker 放在 camera preset，atlas marker 放在 atlas primitive，layer marker 放在 layer primitive。

`capabilities`:

- `capabilities/skeletal_animation`: 2D bone、skeleton、骨骼动画边界。
- `capabilities/effects`: 命中特效、技能特效、纯视觉生命周期效果。
- `capabilities/lighting`: 2D 光照感、发光层、假阴影、bloom 相关表现配置。
- `capabilities/materials`: 自定义 2D material、shader、特殊 sprite material。
- `capabilities/mesh`: 自定义 2D mesh、程序化形状、非 sprite 几何表现。
- `capabilities/particles`: 粒子发射器、粒子配置、纯视觉粒子生命周期。
- `capabilities/pixel`: pixel art、pixel-perfect、pixel grid snap 相关表现策略。

`products`:

- `products/background`: 背景、远景、视差背景层。
- `products/characters`: 角色 2D 表现。
- `products/debug`: 渲染调试显示，例如 gizmo、边界、坐标轴、可视化标记。
- `products/environment`: 天气、雾、环境氛围、非背景类环境装饰。
- `products/items`: 物品、掉落物、可拾取物的 2D 表现。
- `products/overlays`: 贴在世界对象上的覆盖表现，例如血条、选中框、交互提示。
- `products/props`: 静物、装饰物、可见但不负责玩法规则的场景物件。
- `products/screens`: 标题画面、过场屏、加载屏等屏幕级表现。
- `products/transitions`: 屏幕转场、淡入淡出、wipe 等过渡表现。
- `products/ui`: 2D UI 表现、UI root target、UI 层级 marker 和 UI node 基础 bundle。
- `products/ui/root.rs`: UI root、全屏 UI node、UI 层级 bundle。
- `products/ui/demo_menu.rs`: demo 菜单 UI 的具体视觉表现 bundle，例如颜色、字体、尺寸、边距、按钮样式。

## 文件组织规则

- 小目录可以直接把入口类型写在 `mod.rs`；复杂目录再拆成语义明确的文件。
- 具体 Component、Bundle、Resource、system 拆到语义明确的文件里。
- `demo` 目录和 `Demo*` 类型是可删除示例，用来给 AI/人类开发提供参考组合方式。
- 空 `Component` 才是 marker；marker struct 名称必须以 `Marker` 结尾。
- 以 `Marker` 结尾的 `Component` struct 必须是空 struct，不允许带字段。
- 空产品目录可以保留很薄的 `mod.rs` / `plugin.rs` 占位；已经成型的 primitives、capabilities、products 结构不应删除或打散。
- 不新增 `common.rs`、`misc.rs`、`utils.rs` 这类含义模糊的文件。
- 帧动画和骨骼动画必须分目录；不要把骨骼、slot、skin、attachment 写进 `frame_animation`。

## Images 规则

- `images` 是通用图片表现 primitive，不是 Bevy `Sprite` 的二次 facade。
- `images` 可以提供已经命名的 bundle/product，例如 `StaticImage2d` 和 `StaticImage2dBundle`。
- `images` 不加载具体资源路径；图片资源由 `catalog` 或上层传入 `Handle<Image>`。
- `background`、`layers`、`ui` 等复合表现可以组合 `images`，不要重复散装 `Sprite`、`Transform`、`Visibility`。

## Atlases 规则

- `atlases` 是通用 texture atlas 表现 primitive，不写具体角色、物品或 demo 产品。
- `atlases` 可以提供 `AtlasSprite2d` 这类 named bundle，用于显示 atlas layout 的某一格。
- `atlases` 不加载具体资源路径，不创建 `TextureAtlasLayout` 资源；image handle 和 layout handle 由 `catalog`、`frame_animation` 或上层传入。
- `atlases` 不负责按时间切换帧；逐帧播放属于 `primitives/frame_animation`。

## Text 规则

- `text` 是通用世界空间文字表现 primitive，第一版只处理 `Text2d`。
- `text` 不处理 UI 布局文字；UI text 属于 `ui` 目录中的具体 UI 表现。
- `text` 不加载具体字体路径；字体资源由 `catalog` 或上层传入 `Handle<Font>`。
- `text` 可以提供已经命名的 bundle/product，例如 `WorldText2d` 和 `WorldText2dBundle`。

## Tilemap 规则

- `tilemap` 是通用 tilemap primitive，不写具体 demo 地图产品。
- `tilemap/chunk.rs` 暴露 `TilemapChunkLayer2d`，基于 Bevy 内置 `TilemapChunk` 和 `TilemapChunkTileData`。
- `tilemap` 不硬编码具体地图原点、尺寸布局、demo 常量或资源路径。
- tile index 布局、tileset 资源和 transform 由 `prefab`、`catalog` 或更高层具体产品传入。
- 不新增 `DemoTilemap*` 类型；demo 地面属于 `prefab/src/world_2d/demo_level` 的具体关卡组合。

## 边界规则

- 可以生成相机、sprite、UI 节点、渲染专用子实体和视觉效果实体。
- 可以定义渲染专用 `Component`，例如 sprite marker、animation state、camera marker。
- 可以读取 `render_2d` 自己定义的视觉组件来决定显示方式。
- 不读取 `ecs` crate 的玩法组件；玩法数据到视觉状态的同步属于 `prefab`、`gameplay` 或更高层桥接代码。
- 不定义核心玩法组件、核心玩法 bundle、玩法 resource、玩法 event。
- 不读取键盘、鼠标、手柄、外设、AI、网络或脚本输入。
- 不写入 `intent`。
- 不执行移动、战斗、物品、碰撞等世界规则。
- 不依赖 `prefab`。
- 不依赖 `physics`。
- 不依赖 `external_runtime`。
- 不依赖 `intent`。
- 不放 3D mesh、3D light、3D camera；这些属于 `render_3d`。

## Animation 规则

- `frame_animation` 和 `skeletal_animation` 只定义 2D 表现层动画。
- animation 模块可以修改视觉表现数据，例如 sprite atlas index、opacity、视觉 transform。
- animation 模块不表达攻击判定、技能阶段、硬直、combo window、移动规则或物理碰撞。
- `primitives/frame_animation` 是通用逐帧动画基础能力，不写具体角色、具体 demo 或具体内容目录。
- `primitives/frame_animation` 不创建 `base/`、`content/`、`demo/` 子目录；当前目录本身就是通用 frame animation 层。
- `primitives/frame_animation` 只暴露通用动画状态、manifest、handle、loader、plugin 和 system，不暴露 `Demo*` 类型或 `demo_*` API。
- 具体角色如何使用 frame animation，写到 `render_2d/src/products/characters` 等语义目录；例如角色视觉 bundle 组合 `FrameAnimation2d` 和 `FrameAnimationManifest2d` handle。
- 具体资源路径由 `catalog` 绑定；`frame_animation` 不直接加载具体图片资源。
- `.frames.ron` 的 Bevy `AssetLoader` 属于 `frame_animation`；它可以使用 `helper::assets::ron` 做薄反序列化，但 manifest 类型和转换逻辑不移出 `render_2d`。
- `capabilities/skeletal_animation` 当前不是通用骨骼 runtime，而是具体自定义骨骼动画产品集合。
- `capabilities/skeletal_animation` 下每个具体产品必须建目录，例如 `capabilities/skeletal_animation/demo/`，不要写成 `demo_skeletal_animation.rs` 巨型单文件。
- skeletal 产品目录 root 只放 `mod.rs`、`entry.rs`、`systems.rs`、`tests.rs` 和 `rig/`。
- skeletal 产品入口写在 `entry.rs`，`mod.rs` 只声明模块和 re-export；这符合全项目 `mod.rs` 只做导出的规则。
- skeletal 的骨架结构写在 `rig/` 子目录，至少拆成 `structure.rs`、`parts.rs`、`bundles.rs`、`layout.rs`。
- `rig/structure.rs` 描述完整 rig 由哪些部件组成；`parts.rs` 描述 torso、arm、shoulder 等部件；`bundles.rs` 描述 root/bone/joint bundle；`layout.rs` 描述尺寸、颜色、位置和左右侧语义。
- 帧动画的 sprite sheet 布局、clip、帧顺序、fps 和循环信息必须来自
  `assets/2d/manifests/frames/**/*.frames.ron`。
- `render_2d` 不允许为具体 sprite sheet 写硬编码切片逻辑，例如
  `TextureAtlasLayout::from_grid(...)` 或 `ImageArrayLayout::{RowCount, ColumnCount}`。
- 第一版不实现复杂骨骼 runtime，只保留清楚的数据边界。

## Capabilities Plugin 规则

- `Render2dPlugin` 默认只加载没有外部 message 前置条件的能力插件。
- 依赖具体 gameplay message、demo event 或外部状态初始化的能力插件必须保持可选，由调用方显式安装。
- `ParticlesPlugin` 只处理 `render_2d` 自己的 emitter 状态、粒子生成和粒子生命周期；`MovementIntent`、`DemoSensorTriggeredEvent` 等玩法数据由 `prefab` 或 `gameplay` 转换成粒子 emitter 状态或 burst 调用。
- capability 可以暴露 `Demo*` 示例 bundle/component，但示例 runtime system 不应让普通 `Render2dPlugin` 在缺少 ECS/gameplay 初始化时 panic。

## 渲染实体规则

如果表现需要缩放、偏移、动画状态或材质，优先创建玩法 Entity 的渲染子 Entity。

不要为了显示效果直接修改玩法 Entity 的核心 `Transform`，除非这个 Transform 本身就是渲染专用实体。

推荐结构：

```text
Gameplay Entity
└── Render Entity
```

`Gameplay Entity` 的位置由 ECS system 结算。

`Render Entity` 的 sprite、scale、atlas、动画由 `render_2d` 维护。

## 不应该放这里

- 物理碰撞体、传感器、joint、raycast、shape query。
- 寻路网格、寻路查询、路径规划。
- hitbox、hurtbox、攻击范围、技能范围。
- 输入源读取和 intent 写入。
- AI、网络、脚本外部运行时。
- 3D 表现内容。

## 依赖规则

- `render_2d` 不依赖 `ecs`。
- `render_2d` 必须依赖 `error`。
- `render_2d` 不依赖 `audio`。
- `render_2d` 不依赖 `external_runtime`。
- `render_2d` 不依赖 `intent`。
- `render_2d` 不依赖 `prefab`。
- `render_2d` 不依赖 `physics`。
- `render_2d` 不依赖 `render_3d`。

## 验证要求

修改 `crates/render_2d` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
