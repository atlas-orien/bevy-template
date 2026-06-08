# prefab

`prefab` 放可生成的游戏对象模板。

它负责把 `crates/ecs` 的游戏语义数据、`crates/physics` 的物理能力和 `crates/render_2d` 的表现数据组合成可以直接生成的完整对象模板。

## 职责

- 定义可生成对象的组合 Bundle。
- 组合 ECS、physics、render bundle。
- 给 scenes 层提供稳定的对象生成入口。

例如：

```rust
commands.spawn(Player2dPrefabBundle::default());
```

而不是在生成系统里散装很多组件。

## 当前结构

- `world_2d`: 2D 世界对象 prefab，例如玩家、敌人、地图物件。
- `world_3d`: 3D 世界对象 prefab，当前只保留命名空间。
- `ui`: 屏幕 UI prefab，2D 和 3D 游戏都可以复用。

## 边界

- 可以依赖 `ecs`、`physics`、`render_2d`。
- 未来 3D prefab 可以依赖 `render_3d`。
- 不读取输入。
- 不写 ECS system 函数。
- 不负责状态流、关卡流程或生成时机。

`scenes` 决定具体场景使用哪些 prefab。

`simulation` 决定什么时候进入或退出场景。
