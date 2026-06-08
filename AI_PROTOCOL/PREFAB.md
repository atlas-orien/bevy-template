# PREFAB

这个文件是 `crates/prefab` 的 AI 规则。

`crates/prefab` 是可生成对象模板基础库。

它组合 ECS、physics、render 等数据，提供可以被场景直接生成的完整对象模板。

## 代码落点

- 2D 世界对象 prefab：写到 `crates/prefab/src/world_2d`。
- 3D 世界对象 prefab：未来写到 `crates/prefab/src/world_3d`。
- 屏幕 UI prefab：写到 `crates/prefab/src/ui`。

## 边界规则

- `prefab` 可以依赖 `ecs`、`physics`、`render_2d`。
- 未来 3D prefab 可以依赖 `render_3d`。
- `prefab` 不读取键盘、鼠标、手柄、网络输入。
- `prefab` 不写 ECS system 函数。
- `prefab` 不决定生成时机。
- `scenes` 决定具体场景使用哪些 prefab。
- `simulation` 决定什么时候进入或退出场景。

## Bundle 规则

- 生成实体时优先使用 prefab bundle，不要在生成系统里散装组件。
- `Player` 只是 marker component。
- `PlayerBundle` 是 ECS 语义组合。
- `Player2dPrefabBundle` 是可生成 2D 玩家实体的默认模板组合。

## 验证要求

修改 `crates/prefab` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
