# 2D 渲染子包

`render_2d` 是项目 2D 表现内容层。

它的任务是“这个游戏里的东西看起来是什么样”，不是“重新封装 Bevy 2D 渲染”。

这里可以直接使用 Bevy 的 `Sprite`、`TextureAtlas`、`Text2d`、`Node`、`ImageNode`、`Transform`、`Visibility` 等类型。

模板阶段默认没有真实游戏内容。每个表现目录只保留可删除的 `example.rs`，用于示范代码应该写在哪里。

## 职责

- 用户配置的 2D 相机。
- 用户配置的 2D 屏幕、抬头显示、菜单、界面。
- 用户配置的精灵、纹理图集、2D 动画。
- 读取 `ecs` 数据，把游戏世界显示出来。
- 创建渲染专用 Entity、Component、bundle 和动画状态。
- 提供 `prefab` 可以直接使用的高层表现 bundle。

## 当前结构

- `camera`: 2D 相机表现。
- `animation`: 2D 表现层动画。
- `characters`: 角色 2D 表现。
- `props`: 静物、装饰物、可见但不负责玩法规则的场景物件。
- `tilemap`: tile map、tile layer、tile chunk 和 tileset 表现。
- `environment`: 背景、远景、视差层、环境装饰。
- `effects`: 命中特效、粒子替代 sprite、纯视觉生命周期效果。
- `screens`: 屏幕级表现，例如标题画面、过场屏、加载屏。
- `ui`: 2D UI 表现。

每个目录都有可删除的 `example.rs`。`animation` 继续拆成 `frame/example.rs` 和 `skeletal/example.rs`，分别示范帧动画和骨骼动画的边界。用户开始真实项目后，可以直接删除或替换这些 example 文件。

## 文件规则

- 每个目录的 `mod.rs` 只做模块导出、re-export 和 Plugin 组装。
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

`render_2d` 可以把这些 Bevy 类型放进项目自己的表现 bundle，例如 `Character2dRenderBundle`。但不要新增只镜像 Bevy 字段的 facade。

碰撞、攻击范围、寻路区域、触发区域不要写在这里。

## animation

`animation` 只处理视觉动画，不处理玩法时序。

- `animation/frame`: sprite sheet、texture atlas、逐帧播放。
- `animation/skeletal`: 2D bone、skeleton、骨骼播放状态。
- 攻击前摇、技能窗口、硬直、combo、碰撞判定不放在 render animation。

## 和 ecs/intent/gameplay 的区别

- `crates/ecs/src/components` 定义玩家是什么。
- `crates/ecs/src/systems` 定义玩家位置等 ECS 数据如何被规则改变。
- `external_runtime` 读取 input/local、input/device、input/ai 等控制来源；网络是双向通信层，v2 单独设计。
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
