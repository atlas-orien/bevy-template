# RENDER_2D

这个文件是 `crates/render_2d` 的 AI 规则。

`crates/render_2d` 是项目 2D 表现内容层。

它不是 Bevy 2D/render 的二次封装层。它直接使用 Bevy 的 `Sprite`、`TextureAtlas`、`Text2d`、`Node`、`ImageNode`、`Transform`、`Visibility` 等组件。

这个模板阶段默认没有真实游戏内容。每个表现目录只保留一个可删除的 `example.rs`，用于告诉用户代码应该写在哪里。和 `physics` 不同，`render_2d` 是用户会经常修改的内容层。用户新增或修改角色外观、动画、UI、背景、相机表现时，应该修改这里。

## 核心职责

- 2D 相机。
- 用户配置的 2D sprite、纹理图集、动画状态和表现专用 marker。
- 用户配置的 2D 场景背景、屏幕表现、HUD、菜单、界面。
- 根据 ECS 世界数据更新 2D 表现，但不改变玩法规则。
- 为 `prefab` 提供高层表现 bundle、marker 和配置好的表现系统。

## 代码落点

- 2D 相机：写到 `crates/render_2d/src/camera`。
- 2D 表现层动画：写到 `crates/render_2d/src/animation`。
- 角色表现：写到 `crates/render_2d/src/characters`。
- 屏幕和背景表现：写到 `crates/render_2d/src/screens`。
- 2D UI 表现：写到 `crates/render_2d/src/ui`。

当前目录是模板默认结构，可以按具体游戏调整，但必须保持表现层边界清楚。

## 文件组织规则

- 每个目录的 `mod.rs` 只做模块导出、re-export 和 Plugin 组装。
- 不要把具体 Component、Bundle 或 system 全部写进 `mod.rs`。
- 每个默认目录保留一个 `example.rs`，示范该目录里应该写什么。
- 用户开始具体项目后，可以删除或替换 `example.rs`。
- 新增表现类型时，先判断它属于 animation、camera、characters、screens 还是 ui；不要新增含义模糊的 `common.rs`、`misc.rs`。

## 边界规则

- 可以生成相机、sprite、UI 节点和渲染专用子实体。
- 可以定义渲染专用 `Component`，例如 sprite marker、animation state、camera marker。
- 可以读取 ECS 组件来决定显示方式。
- 可以定义项目具体表现 bundle，例如 `Player2dRenderBundle`、`Slime2dRenderBundle`、`HealthBar2dBundle`。
- 可以直接使用 Bevy 的 2D/UI 组件；不要为了字段命名再包一层项目 facade。
- 不定义核心玩法组件、bundle、resource、event。
- 不读取键盘、鼠标、手柄、外设、AI、网络或脚本输入。
- 不写入 `intent`。
- 不执行移动、战斗、物品、碰撞等世界规则。
- 不依赖 `prefab`。
- 不依赖 `physics`。
- 不依赖 `external_runtime`。
- 不放 3D 网格、3D 灯光、3D 相机。

## Bevy 使用规则

- 不要重建 Bevy render、Bevy sprite 或 Bevy UI 的基础 API。
- 不要新增 `RenderColor2d`、`RenderSize2d`、`RenderFlip2d`、`RenderVisibility2d`、`RenderZIndex2d` 这类只镜像 Bevy 字段的 facade。
- 颜色、尺寸、翻转、锚点、可见性、z 排序、纹理图集、sprite image mode 等，优先直接使用 Bevy 的 `Sprite`、`Anchor`、`Visibility`、`Transform`、`TextureAtlas`、`Node`、`ImageNode`、`ZIndex`、`GlobalZIndex`。
- `render_2d` 的价值在于把这些 Bevy 组件配置成项目具体表现内容，而不是隐藏 Bevy。
- 碰撞、攻击范围、寻路区域、触发区域不要写在这里。
- 如果几何数据会影响碰撞、寻路、攻击判定或世界规则，放到 `physics`、`ecs` 或 gameplay，不放到 `render_2d`。

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

- 默认模板不强制生成主相机。
- 如果项目需要默认 2D 主相机，由 `render_2d::camera` 注册生成。
- prefab 不生成主相机。
- gameplay 不生成主相机。
- 如果某个游戏需要多相机、跟随相机或相机切换，类型和 system 仍然写在 `render_2d/src/camera`，调度入口由 `Camera2dPlugin` 组装。

## 和 prefab 的 render 边界

- `render_2d` 可以提供挂在 Main World Entity 上的表现组件、marker 或 bundle，供 `prefab` 组合。
- `prefab` 只组合 `render_2d` 提供的高层表现 bundle、marker 或组件，不配置表现细节。
- `prefab` 不直接操作 RenderApp、Render World、render graph、pipeline 或 GPU resource。
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
