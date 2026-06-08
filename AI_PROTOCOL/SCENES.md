# SCENES

这个文件是 `crates/scenes` 的 AI 规则。

`crates/scenes` 是场景装配层。

它负责把 `crates/prefab` 中的对象模板组合成具体场景，例如主菜单、第一关、战斗场景。

## 核心职责

- 定义一个场景由哪些 prefab 组成。
- 提供场景进入时的 spawn system。
- 提供场景退出时的 cleanup system 或 marker。
- 让 `simulation` 只负责状态流，不直接散装 prefab。

## 代码落点

- 主菜单场景：写到 `crates/scenes/src/main_menu`。
- 第一关场景命名空间：写到 `crates/scenes/src/level_01`。
- 场景共享 marker 和清理函数：写到 `crates/scenes/src/shared`。

## 边界规则

- `scenes` 可以依赖 `prefab`。
- `scenes` 可以定义场景生命周期专用 marker，例如 `SceneEntity`。
- `scenes` 可以写场景进入和退出 system。
- 模板本身不携带 demo scene 内容；具体游戏再添加 prefab 组合。
- `scenes` 不写移动、战斗、物品、碰撞等 gameplay system。
- `scenes` 不读取键盘、鼠标、手柄、网络输入。
- `scenes` 不写入 intent。
- `scenes` 不封装物理后端。
- `scenes` 不直接依赖 `render_2d` 或 `render_3d`；显示对象通过 prefab 间接组合。
- `scenes` 不依赖 `simulation`。

## 依赖规则

- `scenes` 可以依赖 `prefab`。
- `scenes` 可以依赖 `ecs`。
- `scenes` 必须依赖 `error`。
- `scenes` 不依赖 `simulation`。
- `scenes` 不依赖 `input`。
- `scenes` 不依赖 `intent`。
- `scenes` 不依赖 `physics`。
- `scenes` 不依赖 `render_2d` 或 `render_3d`。

## 验证要求

修改 `crates/scenes` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
