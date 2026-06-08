# RENDER_2D

这个文件是 `crates/render_2d` 的 AI 规则。

`crates/render_2d` 是 2D 表现层。

它读取 ECS 世界数据和 simulation 状态，把游戏显示成 2D 画面。

## 核心职责

- 2D 相机。
- 2D sprite、纹理图集、动画状态和表现专用 marker。
- 2D 场景背景、屏幕表现、HUD、菜单、界面。
- 根据 ECS 数据更新 2D 表现。

## 代码落点

- 2D 相机：写到 `crates/render_2d/src/camera`。
- 角色表现：写到 `crates/render_2d/src/characters`。
- 屏幕和背景表现：写到 `crates/render_2d/src/screens`。
- 2D UI 表现：写到 `crates/render_2d/src/ui`。

当前目录是模板默认结构，可以按具体游戏调整，但必须保持表现层边界清楚。

## 边界规则

- 可以生成相机、sprite、UI 节点和渲染专用子实体。
- 可以定义渲染专用 `Component`，例如 `PlayerSprite`、`PlayerAnimation`、`MainCamera`。
- 可以读取 ECS 组件来决定显示方式。
- 不定义核心玩法组件、bundle、resource、event。
- 不读取键盘、鼠标、手柄、网络输入。
- 不写入 `intent`。
- 不执行移动、战斗、物品、碰撞等世界规则。
- 不依赖 `prefab`。
- 不依赖 `physics`。
- 不依赖 `input`。
- 不放 3D 网格、3D 灯光、3D 相机。

## 渲染实体规则

如果表现需要缩放、偏移、动画状态或材质，优先创建玩法 Entity 的渲染子 Entity。

不要为了显示效果直接修改玩法 Entity 的核心 `Transform`，除非这个 Transform 本身就是渲染专用实体。

推荐：

```text
Player gameplay Entity
└── PlayerSprite render Entity
```

`Player gameplay Entity` 的位置由 ECS system 结算。

`PlayerSprite render Entity` 的 sprite、scale、atlas、动画由 `render_2d` 维护。

## 依赖规则

- `render_2d` 可以依赖 `ecs`。
- `render_2d` 可以依赖 `simulation` 的状态定义。
- `render_2d` 必须依赖 `error`。
- `render_2d` 不依赖 `input`。
- `render_2d` 不依赖 `intent`。
- `render_2d` 不依赖 `prefab`。
- `render_2d` 不依赖 `physics`。

## 验证要求

修改 `crates/render_2d` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
