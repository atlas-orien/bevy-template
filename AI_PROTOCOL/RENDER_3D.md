# RENDER_3D

这个文件是 `crates/render_3d` 的 AI 规则。

`crates/render_3d` 是项目 3D 表现内容层。

它读取 ECS 世界数据，把游戏显示成 3D 画面。它不是不可修改的底层库，也不是 `bevy_render` 的二次实现。用户可以在这里写具体游戏的 3D 表现代码。

## 核心职责

- 3D 相机。
- 3D model、mesh、材质、灯光、环境、场景表现和表现专用 marker。
- 3D 动画、特效、粒子、覆盖表现。
- 3D UI 或 3D 空间中的表现连接代码。
- 根据 ECS 数据更新 3D 表现。
- 提供 `prefab` 可以直接组合的高层表现 bundle、marker、component 或 plugin。

## 代码落点

- 3D 相机：写到 `crates/render_3d/src/camera`。
- 3D 模型、mesh、实例化表现：写到 `crates/render_3d/src/models`。
- 3D 材质、shader material、材质状态：写到 `crates/render_3d/src/materials`。
- 3D 动画、骨骼动画、animation graph、播放状态：写到 `crates/render_3d/src/animation`。
- 3D 光源、阴影、bloom、lightmap 表现：写到 `crates/render_3d/src/lighting`。
- skybox、environment map、雾、体积、环境氛围：写到 `crates/render_3d/src/environment`。
- 3D 场景装配和场景级表现：写到 `crates/render_3d/src/scenes`。
- 角色 3D 表现：写到 `crates/render_3d/src/characters`。
- 物品、装备、掉落物 3D 表现：写到 `crates/render_3d/src/items`。
- 静物、装饰物、场景摆件 3D 表现：写到 `crates/render_3d/src/props`。
- 命中特效、法术特效、拖尾、爆炸等纯视觉生命周期效果：写到 `crates/render_3d/src/effects`。
- 3D 粒子发射器和粒子配置：写到 `crates/render_3d/src/particles`。
- 世界空间血条、名字、选中框、交互提示：写到 `crates/render_3d/src/overlays`。
- 3D 渲染调试显示：写到 `crates/render_3d/src/debug`。
- 3D UI 表现：写到 `crates/render_3d/src/ui`。

当前目录是模板默认结构，可以按具体游戏调整，但必须保持表现层边界清楚。

## assets 配合规则

`assets/3d` 是 runtime 资源目录，按资源类型分类。

`render_3d` 是资源使用层，按表现功能分类。

不要把 `render_3d/src` 做成 `assets/3d` 的机械镜像。一个表现对象经常需要同时使用 model、texture、material、animation、rig 和 shader，代码应该放到对象语义最清楚的目录。

常见映射：

- `crates/render_3d/src/models` 使用 `assets/3d/models`。
- `crates/render_3d/src/materials` 使用 `assets/3d/materials`、`assets/3d/textures`、`assets/shaders/3d`。
- `crates/render_3d/src/animation` 使用 `assets/3d/animations`、`assets/3d/rigs`、`assets/3d/skeletons`。
- `crates/render_3d/src/lighting` 使用 `assets/3d/lightmaps`、`assets/3d/irradiance-volumes`、`assets/3d/environment-maps`。
- `crates/render_3d/src/environment` 使用 `assets/3d/environment-maps`、`assets/3d/volumes`。
- `crates/render_3d/src/scenes` 使用 `assets/3d/scenes`，必要时也可以使用 `assets/scenes`。

示例：

```text
assets/3d/models/humanoid/humanoid.glb
assets/3d/materials/humanoid/skin-a.ron
assets/3d/textures/humanoid/skin-a-base-color.png
assets/3d/animations/humanoid/walk.glb
```

这些资源如果共同组成一个角色表现，代码应该写到：

```text
crates/render_3d/src/characters/
```

prefab 应该使用 `render_3d` 暴露的高层表现结构，不应该自己散落引用所有资源路径。

## 边界规则

- 可以直接使用 Bevy 的 `Camera3d`、`SceneRoot`、`Mesh3d`、`MeshMaterial3d`、`StandardMaterial`、`DirectionalLight`、`PointLight`、`SpotLight`、`AnimationPlayer`、`Transform`、`Visibility` 等类型。
- 可以生成相机、model、mesh、材质、灯光、UI 节点和渲染专用子实体。
- 可以定义渲染专用 `Component`，例如 camera marker、scene marker、animation state、material marker。
- 可以读取 ECS 组件来决定显示方式。
- 可以把 Bevy 类型组合进项目自己的表现 bundle，例如 `Character3dRenderBundle`。
- 不新增只镜像 Bevy 字段的 facade。
- 不定义核心玩法组件、bundle、resource、event。
- 不读取键盘、鼠标、手柄、外设、AI、网络或脚本输入。
- 不写入 `intent`。
- 不执行移动、战斗、物品、碰撞等世界规则。
- 不依赖 `prefab`。
- 不依赖 `physics`。
- 不依赖 `external_runtime`。
- 不放 2D sprite、2D HUD 或 2D 相机。

## 渲染实体规则

如果表现需要缩放、偏移、动画状态或材质，优先创建玩法 Entity 的渲染子 Entity。

不要为了显示效果直接修改玩法 Entity 的核心 `Transform`，除非这个 Transform 本身就是渲染专用实体。

推荐结构：

```text
Gameplay Entity
└── Render Entity
```

`Gameplay Entity` 的位置由 ECS system 结算。

`Render Entity` 的 mesh、material、light、animation 由 `render_3d` 维护。

## 和 prefab 的 render 边界

- `render_3d` 可以提供挂在 Main World Entity 上的表现组件、marker 或 bundle，供 `prefab` 组合。
- `prefab` 只组合这些 Main World 表现数据，不直接操作 RenderApp、Render World、render graph、pipeline 或 GPU resource。
- Render SubApp 的 extract、prepare、queue、draw 等流程属于 Bevy/render 层。
- 如果表现逻辑需要 system 同步、动画推进、材质更新或渲染子实体维护，放在 `render_3d`，不要放在 `prefab`。

## 依赖规则

- `render_3d` 可以依赖 `ecs`。
- `render_3d` 必须依赖 `error`。
- `render_3d` 不依赖 `audio`。
- `render_3d` 不依赖 `external_runtime`。
- `render_3d` 不依赖 `intent`。
- `render_3d` 不依赖 `prefab`。
- `render_3d` 不依赖 `physics`。
- `render_3d` 不依赖 `render_2d`。

## 验证要求

修改 `crates/render_3d` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
