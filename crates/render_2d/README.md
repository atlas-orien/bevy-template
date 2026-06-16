# 2D 渲染子包

`render_2d` 是项目 2D 表现内容层。

它的任务是“这个游戏里的东西看起来是什么样”，不是“重新封装 Bevy 2D 渲染”。

这里可以直接使用 Bevy 的 `Sprite`、`TextureAtlas`、`Text2d`、`Node`、`ImageNode`、`Transform`、`Visibility` 等类型。

模板阶段默认没有真实游戏内容。每个表现目录只保留可替换的占位文件，用于固定代码落点。

## 职责

- 用户配置的 2D 相机。
- 用户配置的 2D 屏幕、抬头显示、菜单、界面。
- 用户配置的精灵、纹理图集、2D 动画。
- 读取 `ecs` 数据，把游戏世界显示出来。
- 创建渲染专用 Entity、Component、bundle 和动画状态。
- 提供 `prefab` 可以直接使用的高层表现 bundle。

## 当前结构

- `primitives`: 最小通用表现单元，例如 camera、images、layers、text、tilemap、frame animation。
- `capabilities`: 较复杂的通用表现能力，例如 skeletal、effects、particles、materials、mesh、lighting、pixel。
- `products`: 具体游戏对象、画面、场景或 UI 表现，例如 characters、props、background、ui、screens。

关键路径：

- `primitives/camera`: 2D 相机基础能力和可直接实例化的 camera presets。
- `primitives/atlases`: 通用 texture atlas sprite primitive。
- `primitives/frame_animation`: sprite sheet、texture atlas、逐帧播放。
- `primitives/markers.rs`: 跨 primitive 使用的无数据语义 marker。
- `primitives/tilemap/chunk.rs`: `TilemapChunkLayer2d`，基于 Bevy `TilemapChunk` 组合 tileset、chunk size、tile size、tile index 数据和 transform。
- `capabilities/skeletal_animation`: 自定义骨骼动画能力。
- `products/ui`: 2D UI 表现、UI root target、UI 层级 marker 和 UI node 基础 bundle。
- `products/characters`: 角色 2D 表现。
- `products/background`: 背景、远景、视差背景层。

每个目录都可以保留一个可删除的占位模块。用户开始真实项目后，可以直接删除或替换这些占位文件。

## 文件规则

- 小目录可以直接把入口类型写在 `mod.rs`；复杂目录再拆成语义明确的文件。
- 具体 Component、Bundle、system 拆到语义明确的文件里。
- 不新增 `common.rs`、`misc.rs` 这类含义模糊的文件。
- 默认模板不强制生成主相机；如果项目需要默认相机，写在 `camera`。
- prefab 和 gameplay 不生成主相机。

## Bevy 边界

不要在这里重建 Bevy 的基础组件。

- `Sprite`、`TextureAtlas`、`SpriteImageMode` 直接用 Bevy。
- `Transform`、`Visibility`、`Anchor` 直接用 Bevy。
- `Text2d`、`Text`、`Node`、`ImageNode` 直接用 Bevy。
- UI 的 `ZIndex`、`GlobalZIndex` 直接用 Bevy。
- UI root 用 Bevy `UiTargetCamera` 显式绑定到 UI camera。`IsDefaultUiCamera` 只作为默认 fallback。
- UI 节点本身不使用 `RenderLayers`；世界 sprite、mesh、`Text2d` 等世界表现才用 `RenderLayers`。
- 多 camera 叠加时用 `Camera.order`。UI camera order 应高于 world camera，确保屏幕 UI 在世界画面之上。

`render_2d` 可以把这些 Bevy 类型放进项目自己的表现 bundle，例如 `Character2dRenderBundle`。但不要新增只镜像 Bevy 字段的 facade。

碰撞、攻击范围、寻路区域、触发区域不要写在这里。

## animation

`frame_animation` 和 `skeletal_animation` 只处理视觉动画，不处理玩法时序。

- `primitives/frame_animation`: sprite sheet、texture atlas、逐帧播放。
- `capabilities/skeletal_animation`: 2D bone、skeleton、骨骼播放状态。
- 攻击前摇、技能窗口、硬直、combo、碰撞判定不放在 render animation。

## tilemap

`tilemap` 只放通用 tilemap 表现 primitive。

- `TilemapChunkLayer2d` 负责把 `Handle<Image>`、chunk size、tile display size、tile index 数据和 translation 组合成 Bevy tilemap chunk bundle。
- 具体地图布局、demo 地面原点、tileset 路径和 loader settings 不放在 `render_2d::primitives::tilemap`。
- demo 关卡地面属于 `prefab/src/world_2d/demo_level`；资源加载属于 `catalog`。

## 和 ecs/intent/gameplay 的区别

- `crates/ecs/src/components` 定义玩家是什么。
- `crates/ecs/src/systems` 定义玩家位置等 ECS 数据如何被规则改变。
- `peripherals` 读取本机键盘、鼠标和手柄；`interaction` 读取 UI 和世界对象 hover/click 等 Bevy interaction；`external_runtime` 读取 AI、脚本、回放等 Bevy App 外部来源。网络是双向通信层，v2 单独设计。
- `intent` 表达 Entity 想做什么。
- `prefab` 把 `render_2d` 的高层表现 bundle 组合进完整对象模板。
- `gameplay` 负责状态流、gameplay session 生命周期和 system 调度。
- `render_2d` 只负责对象看起来是什么样。

## 渲染实体

表现层可以创建渲染专用子实体。

例如：

```text
Gameplay Entity
└── Render Entity
```

玩法 Entity 的 `Transform` 表示世界位置，由 ECS system 修改。

渲染子 Entity 的 `Sprite`、贴图图集、scale、动画 timer 由 `render_2d` 修改。

这样可以避免为了显示效果直接改玩法 Entity 的核心数据。

## 不应该放这里

- 不定义核心游戏数据。
- 不读取键盘来决定 Entity 意图。
- 不写入 intent。
- 不执行战斗、碰撞、物品结算等世界规则。
- 不依赖 external_runtime、intent、prefab、physics、gameplay。
- 不放 3D 网格、3D 灯光、3D 相机。
