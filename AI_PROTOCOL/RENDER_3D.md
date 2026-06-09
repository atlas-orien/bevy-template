# RENDER_3D

这个文件是 `crates/render_3d` 的 AI 规则。

`crates/render_3d` 是 3D 表现层。

它读取 ECS 世界数据，把游戏显示成 3D 画面。

## 核心职责

- 3D 相机。
- 3D mesh、材质、灯光、场景表现和表现专用 marker。
- 3D UI 或 3D 空间中的表现连接代码。
- 根据 ECS 数据更新 3D 表现。

## 代码落点

- 3D 相机：写到 `crates/render_3d/src/camera`。
- 3D 场景、mesh、灯光表现：写到 `crates/render_3d/src/scene`。
- 3D UI 表现：写到 `crates/render_3d/src/ui`。

当前目录是模板默认结构，可以按具体游戏调整，但必须保持表现层边界清楚。

## 边界规则

- 可以生成相机、mesh、材质、灯光、UI 节点和渲染专用子实体。
- 可以定义渲染专用 `Component`，例如 camera marker、scene marker、animation state、material marker。
- 可以读取 ECS 组件来决定显示方式。
- 不定义核心玩法组件、bundle、resource、event。
- 不读取键盘、鼠标、手柄、外设、AI、网络或脚本输入。
- 不写入 `intent`。
- 不执行移动、战斗、物品、碰撞等世界规则。
- 不依赖 `prefab`。
- 不依赖 `physics`。
- 不依赖 `input`。
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
- `render_3d` 不依赖 `input`。
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
