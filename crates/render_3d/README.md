# 3D 渲染子包

`render_3d` 是项目 3D 表现内容层。

它的任务是“这个游戏里的东西以 3D 看起来是什么样”，不是“重新封装 Bevy 3D 渲染”，也不是“决定世界怎么变化”。

这里可以直接使用 Bevy 的 `Camera3d`、`SceneRoot`、`Mesh3d`、`MeshMaterial3d`、`StandardMaterial`、`DirectionalLight`、`PointLight`、`SpotLight`、`AnimationPlayer`、`Transform`、`Visibility` 等类型。

## 职责

- 用户配置的 3D 相机。
- 用户配置的 3D mesh、材质、灯光、环境和场景表现。
- 用户配置的 3D 动画、特效、粒子和覆盖表现。
- 读取 `ecs` 数据，把游戏世界显示出来。
- 创建渲染专用 Entity、Component、bundle 和动画状态。
- 提供 `prefab` 可以直接使用的高层表现 bundle。

## 当前结构

`render_3d` 使用和 `render_2d` 一致的三层结构。

### primitives

通用、低层、可组合的 3D 表现基础件：

- `primitives/camera`: 3D camera 基础结构；当前提供 fixed、follow、orbit、isometric、top-down 等 camera preset。输入控制不放在这里，上层只修改对应 camera component 的数据。
- `primitives/meshes`: 当前提供 `StaticMesh3d` / `StaticMesh3dBundle`，组合 `Mesh3d`、`MeshMaterial3d<StandardMaterial>`、`Transform`、`Visibility`。
- `primitives/materials`: 当前提供 `StandardSurface3d`，用于创建项目默认的 `StandardMaterial` 配置；`presets` 可以封装模板自带 preview/demo 材质资源。
- `primitives/lights`: 当前提供 direction / point / spot 基础 light bundle，以及 `presets/sun` 里的太阳光预设。
- `primitives/transforms`: 当前提供 `LookAtTransform3d`，统一表达 3D 相机和灯光常用的 look-at transform。
- `primitives/textures`: 当前提供 `TextureAsset3d`，统一表达单张 texture 的路径和 sRGB/linear 加载设置。
- `primitives/shaders`: 当前提供 `ShaderAsset3d`，为后续自定义 3D shader preset 预留统一入口。
- 空 `Component` 才是 marker；marker struct 名称必须以 `Marker` 结尾。
- 以 `Marker` 结尾的 `Component` struct 必须是空 struct，不允许带字段。
- marker 和它标记的具体对象放在同一个模块里，不单独集中到 `markers.rs`。

### capabilities

可复用但不是最低层的 3D 表现能力：

- `capabilities/animation`: glTF、骨骼动画、animation graph 和播放状态。
- `capabilities/effects`: 命中特效、法术特效、拖尾、爆炸等纯视觉生命周期效果。
- `capabilities/particles`: 3D 粒子发射器、粒子配置、纯视觉粒子生命周期。

### products

具体游戏对象或语义对象的 3D 表现产品：

- `products/scenes`: 3D 场景装配和场景级表现。
- `products/characters`: 角色 3D 表现。
- `products/items`: 物品、装备、掉落物的 3D 表现。
- `products/props`: 静物、装饰物、可见但不负责玩法规则的场景物件。
- `products/environment`: skybox、environment map、雾、体积、环境氛围。
- `products/overlays`: 世界空间血条、名字、选中框、交互提示。
- `products/debug`: 3D 渲染调试显示，例如包围盒、坐标轴、骨骼可视化。

## assets 配合

`assets/3d` 是 runtime 资源目录，按资源类型分类。

`render_3d` 是资源使用层，按表现功能分类。

常见映射：

- `primitives/meshes` 使用 `assets/3d/models`。
- `primitives/materials` 使用 `assets/3d/materials` 和 `assets/shaders/3d`。
- `capabilities/animation` 使用 `assets/3d/animations`、`assets/3d/rigs`、`assets/3d/skeletons`。
- `products/environment` 使用 `assets/3d/environment-maps`、`assets/3d/volumes`。
- `products/scenes` 使用 `assets/3d/scenes`，必要时也可以引用 `assets/scenes`。

`render_3d` 不需要镜像 `assets/3d` 的目录结构。一个角色表现可以同时使用 model、texture、material、animation 和 rig，代码仍然应该放在 `products/characters`。

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
- 不放 2D 精灵、2D HUD、普通菜单、按钮、背包、tilemap 或 2D 相机。

普通 UI 不属于 `render_3d`。贴在 3D 世界对象上的名字、血条、选中框、交互提示放到 `products/overlays`。真正存在于 3D 世界里的屏幕、广告牌、全息面板，按语义放到 `products/props`、`capabilities/effects` 或具体对象目录。

`render_2d` 和 `render_3d` 应该保持独立。
