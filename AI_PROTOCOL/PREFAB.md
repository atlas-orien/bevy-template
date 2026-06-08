# PREFAB

这个文件是 `crates/prefab` 的 AI 规则。

`crates/prefab` 是可生成对象模板层，用来组合 `crates/ecs` 和 `crates/physics`。

## 代码落点

- 角色 prefab：写到 `crates/prefab/src/characters`。
- 物品 prefab：未来写到 `crates/prefab/src/items`。
- 世界对象 prefab：未来写到 `crates/prefab/src/world`。

## 边界规则

- `prefab` 可以依赖 `ecs` 和 `physics`。
- `prefab` 不依赖 `render_2d` 或 `render_3d`。
- `prefab` 不读取键盘、鼠标、手柄、网络输入。
- `prefab` 不写 ECS system 函数。
- `prefab` 不决定生成时机。
- `simulation` 决定什么时候生成，`prefab` 决定生成什么组合。

## Bundle 规则

- 生成实体时优先使用 prefab bundle，不要在生成系统里散装组件。
- `Player` 只是 marker component。
- `PlayerBundle` 是 ECS 语义组合。
- `PlayerPrefabBundle` 是可生成玩家实体的默认模板组合。

## 验证要求

修改 `crates/prefab` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
