# RENDER_2D

这个文件是 `crates/render_2d` 的 AI 规则。

`crates/render_2d` 是 2D 表现层。

它读取 ECS 世界数据，把游戏显示成 2D 画面。

## 核心职责

- 2D 相机。
- 2D sprite、纹理图集、动画状态和表现专用 marker。
- 2D 场景背景、屏幕表现、HUD、菜单、界面。
- 根据 ECS 数据更新 2D 表现。

## 代码落点

- 2D 相机：写到 `crates/render_2d/src/camera`。
- 2D 表现层动画：写到 `crates/render_2d/src/animation`。
- 2D 外观属性：写到 `crates/render_2d/src/appearance`。
- 2D 表现层几何：写到 `crates/render_2d/src/geometry`。
- 2D 视觉 transform：写到 `crates/render_2d/src/transform`。
- 2D 视觉排序：写到 `crates/render_2d/src/ordering`。
- 2D sprite 专用属性：写到 `crates/render_2d/src/sprite`。
- 角色表现：写到 `crates/render_2d/src/characters`。
- 屏幕和背景表现：写到 `crates/render_2d/src/screens`。
- 2D UI 表现：写到 `crates/render_2d/src/ui`。

当前目录是模板默认结构，可以按具体游戏调整，但必须保持表现层边界清楚。

## 文件组织规则

- 每个目录的 `mod.rs` 只做模块导出、re-export 和 Plugin 组装。
- 不要把具体 Component、Bundle 或 system 全部写进 `mod.rs`。
- `camera/main_camera.rs` 定义主 2D 相机 marker 和 bundle。
- `camera/systems.rs` 定义相机生成和同步 system。
- `animation/frame` 定义帧动画、sprite sheet、atlas animation 数据，例如 `sprite_frame.rs`、`clip.rs`、`playback.rs`。
- `animation/skeletal` 定义 2D 骨骼动画数据。
- `appearance/color.rs` 定义表现层颜色 component，例如 `RenderColor2d`。
- `appearance/opacity.rs` 定义表现层透明度，例如 `RenderOpacity2d`。
- `appearance/visibility.rs` 定义表现层可见性，例如 `RenderVisibility2d`。
- `geometry/shape.rs` 定义视觉形状，例如 `RenderShape2d`。
- `geometry/size.rs` 定义视觉尺寸，例如 `RenderSize2d`。
- `geometry/anchor.rs` 定义视觉锚点，例如 `RenderAnchor2d`。
- `transform/offset.rs` 定义视觉偏移，例如 `RenderOffset2d`。
- `transform/scale.rs` 定义视觉缩放，例如 `RenderScale2d`。
- `transform/rotation.rs` 定义视觉旋转，例如 `RenderRotation2d`。
- `ordering/z_index.rs` 定义视觉排序，例如 `RenderZIndex2d`。
- `sprite/flip.rs` 定义 sprite 翻转，例如 `RenderFlip2d`。
- `characters/character.rs` 定义角色 2D 表现 marker、表现配置和 bundle。
- `screens/clear_color.rs` 定义屏幕背景色等屏幕级表现 system。
- `ui/theme.rs` 定义 2D UI/表现层颜色常量。
- `ui/markers.rs` 定义 2D UI marker。
- 新增表现类型时，先判断它属于 camera、characters、screens 还是 ui；不要新增含义模糊的 `common.rs`、`misc.rs`。

## 边界规则

- 可以生成相机、sprite、UI 节点和渲染专用子实体。
- 可以定义渲染专用 `Component`，例如 sprite marker、animation state、camera marker。
- 可以读取 ECS 组件来决定显示方式。
- 不定义核心玩法组件、bundle、resource、event。
- 不读取键盘、鼠标、手柄、外设、AI、网络或脚本输入。
- 不写入 `intent`。
- 不执行移动、战斗、物品、碰撞等世界规则。
- 不依赖 `prefab`。
- 不依赖 `physics`。
- 不依赖 `external_runtime`。
- 不放 3D 网格、3D 灯光、3D 相机。

## 表现属性规则

- `geometry` 只定义 2D 表现层几何，例如形状、尺寸、锚点。
- `appearance` 只定义 2D 外观属性，例如颜色、透明度、可见性。
- `transform` 只定义 2D 视觉 transform，例如表现偏移、缩放、旋转。
- `ordering` 只定义 2D 视觉排序。
- `sprite` 只定义 sprite 专用表现属性。
- 这些目录都不定义物理碰撞、攻击范围或 gameplay 区域。
- `RenderShape2d::Circle` 不等于 `PhysicsCollider2d::Circle`。
- `RenderSize2d` 不等于 hitbox 或 hurtbox。
- `RenderOffset2d`、`RenderScale2d`、`RenderRotation2d` 只影响视觉表现，不改变 gameplay Transform 或物理状态。
- `RenderZIndex2d` 只表达视觉排序，不表达 ECS parent/child 关系或 gameplay 优先级。
- `RenderVisibility2d` 和 `RenderOpacity2d` 只控制显示，不表示实体是否存在、死亡或可交互。
- 如果几何数据会影响碰撞、寻路、攻击判定或世界规则，放到 `physics`、`ecs` 或 gameplay，不放到 `render_2d/geometry`。

## Animation 规则

- `animation` 只定义 2D 表现层动画。
- `animation/frame` 放帧动画、sprite sheet、texture atlas animation。
- `animation/skeletal` 放 2D 骨骼动画、bone、skeleton、骨骼播放状态。
- 帧动画和骨骼动画必须分目录；不要把骨骼、slot、skin、attachment 写进 `animation/frame`。
- 第一版不实现复杂骨骼 runtime，只保留清楚的数据边界。
- animation 可以修改视觉表现数据，例如 sprite atlas index、opacity、视觉 transform。
- animation 不表达攻击判定、技能阶段、硬直、combo window、移动规则或物理碰撞。

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

## 相机规则

- 默认 2D 主相机由 `render_2d::camera` 在 startup 注册生成。
- prefab 不生成主相机。
- gameplay 不生成主相机。
- 如果某个游戏需要多相机、跟随相机或相机切换，类型和 system 仍然写在 `render_2d/src/camera`，调度入口由 `Camera2dPlugin` 组装。

## 和 prefab 的 render 边界

- `render_2d` 可以提供挂在 Main World Entity 上的表现组件、marker 或 bundle，供 `prefab` 组合。
- `prefab` 只组合这些 Main World 表现数据，不直接操作 RenderApp、Render World、render graph、pipeline 或 GPU resource。
- Render SubApp 的 extract、prepare、queue、draw 等流程属于 Bevy/render 层。
- 如果表现逻辑需要 system 同步、动画推进、材质更新或渲染子实体维护，放在 `render_2d`，不要放在 `prefab`。

## 依赖规则

- `render_2d` 可以依赖 `ecs`。
- `render_2d` 必须依赖 `error`。
- `render_2d` 不依赖 `audio`。
- `render_2d` 不依赖 `external_runtime`。
- `render_2d` 不依赖 `intent`。
- `render_2d` 不依赖 `prefab`。
- `render_2d` 不依赖 `physics`。

## 验证要求

修改 `crates/render_2d` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
