此文件是项目约束来源。AI 不得为通过检查而修改本文件；规则变更必须由人发起。

# RENDER_2D

这个文件是 `crates/render_2d` 的 AI 规则。

`crates/render_2d` 是项目 2D 表现内容层。

它不是 `bevy_render` 的二次封装，也不是像 `physics` 那样默认不让用户修改的基础 facade。用户可以在这里写已经配置好的具体 2D 表现内容，`prefab` 直接组合这些高层表现 bundle、component、marker 或 plugin。

## 核心职责

- 用户配置的 2D 相机、屏幕、HUD、菜单、界面。
- 用户配置的 sprite、texture atlas、tilemap、2D mesh、2D material、视觉动画。
- 角色、物品、静物、背景、环境、特效、粒子、覆盖层、世界文字等具体项目表现。
- 读取 `ecs` 数据，把游戏世界显示成 2D 画面。
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

- `animation/frame`: sprite sheet、texture atlas、逐帧动画。
- `animation/skeletal`: 2D bone、skeleton、骨骼动画边界。
- `atlases`: 共享 texture atlas、sprite sheet layout、tileset layout。
- `background`: 背景、远景、视差背景层。
- `camera`: 2D 相机基础能力和可直接实例化的 camera presets。
- `characters`: 角色 2D 表现。
- `debug`: 渲染调试显示，例如 gizmo、边界、坐标轴、可视化标记。
- `effects`: 命中特效、技能特效、纯视觉生命周期效果。
- `environment`: 天气、雾、环境氛围、非背景类环境装饰。
- `images`: 通用静态图片表现 primitive，例如单张图片或纯色图片块；不负责加载具体资源路径。
- `items`: 物品、掉落物、可拾取物的 2D 表现。
- `lighting`: 2D 光照感、发光层、假阴影、bloom 相关表现配置。
- `materials`: 自定义 2D material、shader、特殊 sprite material。
- `mesh`: 自定义 2D mesh、程序化形状、非 sprite 几何表现。
- `overlays`: 贴在世界对象上的覆盖表现，例如血条、选中框、交互提示。
- `particles`: 粒子发射器、粒子配置、纯视觉粒子生命周期。
- `pixel`: pixel art、pixel-perfect、pixel grid snap 相关表现策略。
- `props`: 静物、装饰物、可见但不负责玩法规则的场景物件。
- `screens`: 标题画面、过场屏、加载屏等屏幕级表现。
- `text`: 世界空间文字，例如伤害数字、漂浮提示、角色头顶名字。
- `tilemap`: tile map、tile layer、tile chunk、tileset 表现。
- `transitions`: 屏幕转场、淡入淡出、wipe 等过渡表现。
- `ui`: 2D UI 表现。
- `camera/base.rs`: 共享 Bevy 2D camera bundle/config，只放 `Camera2d`、`Camera`、`RenderLayers`、`Transform` 等基础组合。
- `camera/markers.rs`: camera 语义 marker，例如 `SceneCamera2d`、`FollowCameraTarget2d`。
- `camera/presets`: 业务可直接选择的 camera preset，例如 fixed、follow、ui。
- `camera/presets/fixed.rs`: 固定不动的 2D 场景相机。
- `camera/presets/follow.rs`: 跟随目标的 2D 场景相机。
- `camera/presets/ui.rs`: UI 专用 camera 配置。
- `ui`: UI root target、UI 层级 marker 和 UI node 基础 bundle。
- `ui/root.rs`: UI root、全屏 UI node、UI 层级 bundle。
- `ui/demo_menu.rs`: demo 菜单 UI 的具体视觉表现 bundle，例如颜色、字体、尺寸、边距、按钮样式。

## 文件组织规则

- 小目录可以直接把入口类型写在 `mod.rs`；复杂目录再拆成语义明确的文件。
- 具体 Component、Bundle、Resource、system 拆到语义明确的文件里。
- 模板阶段每个目录可以只保留可删除的占位文件。
- 用户开始真实项目后，可以直接删除或替换占位文件。
- 不新增 `common.rs`、`misc.rs`、`utils.rs` 这类含义模糊的文件。
- 帧动画和骨骼动画必须分目录；不要把骨骼、slot、skin、attachment 写进 `animation/frame`。

## Images 规则

- `images` 是通用图片表现 primitive，不是 Bevy `Sprite` 的二次 facade。
- `images` 可以提供已经命名的 bundle/product，例如 `StaticImage2d` 和 `StaticImage2dBundle`。
- `images` 不加载具体资源路径；图片资源由 `catalog` 或上层传入 `Handle<Image>`。
- `background`、`layers`、`ui` 等复合表现可以组合 `images`，不要重复散装 `Sprite`、`Transform`、`Visibility`。

## 边界规则

- 可以生成相机、sprite、UI 节点、渲染专用子实体和视觉效果实体。
- 可以定义渲染专用 `Component`，例如 sprite marker、animation state、camera marker。
- 可以读取 ECS 组件来决定显示方式。
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

- `animation` 只定义 2D 表现层动画。
- animation 可以修改视觉表现数据，例如 sprite atlas index、opacity、视觉 transform。
- animation 不表达攻击判定、技能阶段、硬直、combo window、移动规则或物理碰撞。
- `animation/frame` 是通用逐帧动画基础能力，不写具体角色、具体 demo 或具体内容目录。
- `animation/frame` 不创建 `base/`、`content/`、`demo/` 子目录；当前目录本身就是通用 frame animation 层。
- `animation/frame` 只暴露通用动画状态、manifest、handle、loader、plugin 和 system，不暴露 `Demo*` 类型或 `demo_*` API。
- 具体角色如何使用 frame animation，写到 `render_2d/src/characters` 等语义目录；例如角色视觉 bundle 组合 `FrameAnimation2d` 和 `FrameAnimationManifest2d` handle。
- 具体资源路径由 `catalog` 绑定；`animation/frame` 不直接加载具体图片资源。
- `animation/skeletal` 当前不是通用骨骼 runtime，而是具体自定义骨骼动画产品集合。
- `animation/skeletal` 下每个具体产品必须建目录，例如 `animation/skeletal/demo/`，不要写成 `demo_skeletal_animation.rs` 巨型单文件。
- skeletal 产品目录 root 只放 `mod.rs`、`entry.rs`、`systems.rs`、`tests.rs` 和 `rig/`。
- skeletal 产品入口写在 `entry.rs`，`mod.rs` 只声明模块和 re-export；这符合全项目 `mod.rs` 只做导出的规则。
- skeletal 的骨架结构写在 `rig/` 子目录，至少拆成 `structure.rs`、`parts.rs`、`bundles.rs`、`layout.rs`。
- `rig/structure.rs` 描述完整 rig 由哪些部件组成；`parts.rs` 描述 torso、arm、shoulder 等部件；`bundles.rs` 描述 root/bone/joint bundle；`layout.rs` 描述尺寸、颜色、位置和左右侧语义。
- 帧动画的 sprite sheet 布局、clip、帧顺序、fps 和循环信息必须来自
  `assets/2d/animated/**/*.frames.ron`。
- `render_2d` 不允许为具体 sprite sheet 写硬编码切片逻辑，例如
  `TextureAtlasLayout::from_grid(...)` 或 `ImageArrayLayout::{RowCount, ColumnCount}`。
- 第一版不实现复杂骨骼 runtime，只保留清楚的数据边界。

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

- `render_2d` 可以依赖 `ecs`。
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
