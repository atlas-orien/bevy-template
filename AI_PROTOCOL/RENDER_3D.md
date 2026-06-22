此文件是项目约束来源。AI 不得为通过检查而修改本文件；规则变更必须由人发起。

# RENDER_3D

这个文件是 `crates/render_3d` 的 AI 规则。

`crates/render_3d` 是项目 3D 表现内容层。

它不是 `bevy_render` 的二次封装，也不是像 `physics` 那样默认不让用户修改的基础 facade。用户可以在这里写已经配置好的具体 3D 表现内容，`prefab` 直接组合这些高层表现 bundle、component、marker 或 plugin。

## 核心职责

- 用户配置的 3D 相机、scene、model、mesh、材质、灯光、环境和场景表现。
- 用户配置的 3D 动画、特效、粒子、覆盖表现。
- 角色、物品、静物、场景、环境、特效、覆盖层等具体项目表现。
- 根据上层传入的视觉组件、资源句柄和表现状态，把游戏世界显示成 3D 画面。
- 创建渲染专用 Entity、Component、Bundle、Resource 和视觉 system。
- 提供 `prefab` 可以直接组合的高层表现 bundle、marker、component 或 plugin。

## Bevy 边界

- 绝对不要重写一次 `bevy_render`。
- 能直接使用 Bevy 类型时，直接使用 Bevy 类型。
- `Camera3d`、`Projection`、`Camera` 直接用 Bevy。
- `SceneRoot`、`Mesh3d`、`MeshMaterial3d` 直接用 Bevy。
- `StandardMaterial`、custom material、shader handle 直接用 Bevy。
- `DirectionalLight`、`PointLight`、`SpotLight` 直接用 Bevy。
- `AnimationPlayer`、`AnimationGraph`、`AnimationGraphHandle` 直接用 Bevy。
- `Transform`、`Visibility` 直接用 Bevy。
- 可以把 Bevy 类型组合进项目自己的高层表现 bundle，例如角色 3D 表现 bundle、道具 3D 表现 bundle、scene 表现 bundle。
- 不新增只镜像 Bevy 字段的 facade。

## 和 prefab 的关系

- `render_3d` 提供已经配置好的具体表现内容。
- `prefab` 直接使用 `render_3d` 提供的 bundle、component、marker 或 helper。
- `prefab` 不应该为了 3D 表现再重复配置一遍 scene、mesh、材质、灯光、动画、覆盖层等细节。
- 如果某个表现内容只属于某个具体游戏对象，可以先写在 `render_3d` 的对应分类目录，再由 `prefab` 组合。

## 代码落点

- `primitives`: 最小通用表现单元，供 capabilities/products 组合。
- `capabilities`: 较复杂的通用表现能力，可以带 plugin、system、runtime state。
- `products`: 具体游戏对象、画面、场景或 3D 表现，通常给 `prefab` 直接组合。
- `products` 对 `prefab` 暴露的公共入口应优先是命名的 public bundle struct 或具体 product struct；不要把最终公共产品 API 做成散装 Bevy 字段。

`primitives`:

- `primitives/camera`: 3D camera 基础结构和可直接实例化的 camera presets。
- `primitives/lights`: 3D light 基础 bundle 和可直接实例化的 light presets。
- `primitives/materials`: 通用材质描述和把资源句柄转换成 Bevy material 的能力；不绑定具体资源路径。
- `primitives/meshes`: 单个 mesh 资源或程序化 mesh 表现 primitive，不混入具体对象语义。
- `primitives/models`: model、mesh、material、transform、visibility 的通用组合；完整 glb/gltf scene 属于 `products/scenes`。
- `primitives/transforms`: 3D camera、light、model 常用的 transform helper，例如 look-at。

`capabilities`:

- `capabilities/animation`: 3D glTF animation、AnimationGraph、播放状态和状态同步 system。
- `capabilities/effects`: 命中特效、技能特效、纯视觉生命周期效果。
- `capabilities/particles`: 3D 粒子发射器、粒子配置、纯视觉粒子生命周期。

`products`:

- `products/characters`: 角色 3D 表现。
- `products/debug`: 3D 渲染调试显示，例如包围盒、坐标轴、骨骼可视化。
- `products/environment`: skybox、environment map、雾、体积、环境氛围。
- `products/items`: 物品、装备、掉落物的 3D 表现。
- `products/overlays`: 贴在世界对象上的覆盖表现，例如血条、名字、选中框、交互提示。
- `products/props`: 静物、装饰物、可见但不负责玩法规则的场景物件。
- `products/scenes`: 3D 场景装配和场景级表现。

## 文件组织规则

- 小目录可以直接把入口类型写在 `mod.rs`；复杂目录再拆成语义明确的文件。
- 具体 Component、Bundle、Resource、system 拆到语义明确的文件里。
- `demo` 目录和 `Demo*` 类型是可删除示例，用来给 AI/人类开发提供参考组合方式。
- 空 `Component` 才是 marker；marker struct 名称必须以 `Marker` 结尾。
- 以 `Marker` 结尾的 `Component` struct 必须是空 struct，不允许带字段。
- marker 和它标记的具体对象、bundle、system 放在同一个模块里，不单独集中到 `markers.rs`。
- 空产品目录可以保留很薄的 `mod.rs` / `plugin.rs` 占位；已经成型的 primitives、capabilities、products 结构不应删除或打散。
- 不新增 `common.rs`、`misc.rs`、`utils.rs` 这类含义模糊的文件。

## assets 配合规则

`assets/3d` 是 runtime 资源目录，按资源类型分类。

`render_3d` 是资源使用层，按表现功能分类。

不要把 `render_3d/src` 做成 `assets/3d` 的机械镜像。一个表现对象经常需要同时使用 model、texture、material、animation、rig 和 shader，代码应该放到对象语义最清楚的目录。

常见映射：

- `primitives/meshes` 使用上层传入的 mesh handle，mesh 可能来自 `assets/3d/models` 或程序生成。
- `primitives/models` 使用上层传入的 mesh/material handle。
- `primitives/materials` 使用上层传入的 texture / shader handle 和材质参数。
- `capabilities/animation` 使用 `assets/3d/animations`、`assets/3d/rigs`、`assets/3d/skeletons` 或 glTF 内嵌 animation label。
- `products/environment` 使用 `assets/3d/environment-maps`、`assets/3d/volumes`。
- `products/scenes` 使用 `assets/3d/scenes`，必要时也可以引用 `assets/scenes`。

示例：

```text
assets/3d/models/humanoid/humanoid.glb
assets/3d/materials/humanoid-skin-a/skin-a.ron
assets/3d/materials/humanoid-skin-a/base-color.png
assets/3d/animations/humanoid/walk.glb
```

这些资源如果共同组成一个角色表现，代码应该写到：

```text
crates/render_3d/src/characters/
```

prefab 应该使用 `render_3d` 暴露的高层表现结构，不应该自己散落引用所有资源路径。

材质基础结构和材质预设可以分层：

- `helper::assets` 定义通用 image、texture、shader 资源路径和加载设置；材质贴图使用 `TextureAsset`，普通图片才使用 `ImageAsset`。
- `primitives/materials` 定义通用材质描述和把资源句柄转换成 Bevy material 的能力。
- `primitives/materials/presets` 可以定义模板自带 preview/demo 材质预设，并绑定这些预设自己的资源路径。
- `render_3d` 不直接调用 `asset_server.load(...)` 或 `asset_server.load_with_settings(...)`；加载具体 image、texture、shader 必须通过 `helper::assets` 的 `ImageAsset`、`TextureAsset`、`ShaderAsset` 等类型。
- 其它 render_3d 目录不要散落具体贴图路径；需要资源绑定时优先通过 `helper::assets`、材质 preset 或更高层具体 product 表达。
- prefab 不直接加载材质资源路径，只组合 render_3d 暴露的材质/presentation 结果。

## 边界规则

- 可以直接使用 Bevy 的 `Camera3d`、`SceneRoot`、`Mesh3d`、`MeshMaterial3d`、`StandardMaterial`、`DirectionalLight`、`PointLight`、`SpotLight`、`AnimationPlayer`、`Transform`、`Visibility` 等类型。
- 可以生成相机、model、mesh、材质、灯光和渲染专用子实体。
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
- 不放普通 UI、菜单、按钮、背包、HUD 等界面逻辑。

普通 UI 不属于 `render_3d`。贴在 3D 世界对象上的名字、血条、选中框、交互提示放到 `overlays`。真正存在于 3D 世界里的屏幕、广告牌、全息面板，按语义放到 `props`、`effects` 或具体对象目录。

## Animation 规则

- `capabilities/animation` 定义 3D 表现层动画播放能力，包括 `AnimationClip3d`、`AnimationPlayback3d`、AnimationGraph 构造和 scene-ready 播放 system。
- animation 模块可以修改视觉表现数据，例如 active animation node、播放模式、视觉 transform。
- animation 模块不表达攻击判定、技能阶段、硬直、combo window、移动规则或物理碰撞。
- 具体角色的 animation state、clip set 和状态同步 system 可以写在 `capabilities/animation/<product>/` 或具体 `products/characters` 子目录；第一版 demo fox 放在 `capabilities/animation/demo/`。
- `catalog` 只绑定具体 glTF/scene/model 资源路径；catalog 不定义 `Demo*Animations`、animation set、animation state 或播放 system。
- glTF animation label 到项目 animation state 的映射属于 `render_3d`，不属于 `catalog`。
- 具体角色如何使用 animation，写到 `render_3d/src/products/characters` 等语义目录；例如角色视觉 bundle 组合 `DemoFox3dAnimationSet` 和 `DemoFox3dAnimationStateSet`。
- 第一版不实现复杂骨骼 runtime，只使用 Bevy glTF loader 自动生成的 `AnimationPlayer`，后续骨骼/slot/skin/attachment 必须单独分目录。

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
