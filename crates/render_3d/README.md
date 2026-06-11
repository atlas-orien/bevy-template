# 3D 渲染子包

`render_3d` 是项目 3D 表现内容层。

它的任务是“这个游戏里的东西以 3D 看起来是什么样”，不是“重新封装 Bevy 3D 渲染”，也不是“决定世界怎么变化”。

这里可以直接使用 Bevy 的 `Camera3d`、`SceneRoot`、`Mesh3d`、`MeshMaterial3d`、`StandardMaterial`、`DirectionalLight`、`PointLight`、`SpotLight`、`AnimationPlayer`、`Transform`、`Visibility` 等类型。

模板阶段默认没有真实游戏内容。每个表现目录只保留可替换的占位模块，用于固定代码落点。

## 职责

- 用户配置的 3D 相机。
- 用户配置的 3D 模型、材质、灯光、环境和场景表现。
- 用户配置的 3D 动画、特效、粒子和覆盖表现。
- 读取 `ecs` 数据，把游戏世界显示出来。
- 创建渲染专用 Entity、Component、bundle 和动画状态。
- 提供 `prefab` 可以直接使用的高层表现 bundle。

## 当前结构

- `camera`: 3D 相机表现，例如固定相机、跟随相机、第一/第三人称相机。
- `models`: 3D 模型加载、实例化、mesh 表现 bundle。
- `materials`: 标准材质、自定义材质、shader material 的项目落点。
- `animation`: 3D 动画、骨骼动画、animation graph 和播放状态。
- `lighting`: 3D 光源、阴影、bloom、lightmap 相关表现。
- `environment`: skybox、environment map、雾、体积、环境氛围。
- `scenes`: 3D 场景装配、场景级表现。
- `characters`: 角色 3D 表现。
- `items`: 物品、装备、掉落物的 3D 表现。
- `props`: 静物、装饰物、可见但不负责玩法规则的场景物件。
- `effects`: 命中特效、法术特效、拖尾、爆炸等纯视觉生命周期效果。
- `particles`: 3D 粒子发射器、粒子配置、纯视觉粒子生命周期。
- `overlays`: 世界空间血条、名字、选中框、交互提示。
- `debug`: 3D 渲染调试显示，例如包围盒、坐标轴、骨骼可视化。
- `ui`: 3D 专用 UI 或和 3D 视图绑定的 UI。

## assets 配合

`assets/3d` 是 runtime 资源目录，按资源类型分类。

`render_3d` 是资源使用层，按表现功能分类。

常见映射：

- `models`: 使用 `assets/3d/models`。
- `materials`: 使用 `assets/3d/materials`、`assets/3d/textures` 和 `assets/shaders/3d`。
- `animation`: 使用 `assets/3d/animations`、`assets/3d/rigs`、`assets/3d/skeletons`。
- `lighting`: 使用 `assets/3d/lightmaps`、`assets/3d/irradiance-volumes`、`assets/3d/environment-maps`。
- `environment`: 使用 `assets/3d/environment-maps`、`assets/3d/volumes`。
- `scenes`: 使用 `assets/3d/scenes`，必要时也可以引用 `assets/scenes`。

`render_3d` 不需要镜像 `assets/3d` 的目录结构。一个角色表现可以同时使用 model、texture、material、animation 和 rig，代码仍然应该放在 `characters`。

## 文件规则

- 每个目录的 `mod.rs` 只做模块导出、re-export 和 Plugin 组装。
- 具体 Component、Bundle、system 拆到语义明确的文件里。
- 不新增 `common.rs`、`misc.rs` 这类含义模糊的文件。
- 默认模板不强制生成主相机；如果项目需要默认相机，写在 `camera`。
- prefab 和 gameplay 不生成主相机。

## Bevy 边界

不要在这里重建 Bevy 的基础组件。

- `Camera3d`、`Projection`、`Camera` 直接用 Bevy。
- `SceneRoot`、`Mesh3d`、`MeshMaterial3d` 直接用 Bevy。
- `StandardMaterial`、custom material、shader handle 直接用 Bevy。
- `DirectionalLight`、`PointLight`、`SpotLight` 直接用 Bevy。
- `Transform`、`Visibility` 直接用 Bevy。

`render_3d` 可以把这些 Bevy 类型放进项目自己的表现 bundle，例如 `Character3dRenderBundle`。但不要新增只镜像 Bevy 字段的 facade。

碰撞、攻击范围、寻路区域、触发区域不要写在这里。

## 当前状态

这个子包已经存在，但默认没有接入 `app`。

当前默认模板仍然运行 2D：

```rust
Render2dPlugin
```

需要做 3D 模板时，在顶层组装 `Render3dPlugin`。

## 不应该放这里

- 不定义核心游戏数据。
- 不读取输入。
- 不写入 intent。
- 不写世界模拟、移动、战斗、碰撞或物品结算。
- 不依赖 external_runtime、intent、prefab、physics、gameplay、render_2d。
- 不放 2D 精灵、2D HUD、2D tilemap 或 2D 相机。

`render_2d` 和 `render_3d` 应该保持独立。
