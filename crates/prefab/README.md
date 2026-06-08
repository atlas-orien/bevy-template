# prefab

`prefab` 放可生成的游戏对象模板。

它负责把 `crates/ecs` 的游戏语义数据和 `crates/physics` 的物理能力组合成可以直接生成的 Bundle。

## 职责

- 定义可生成对象的组合 Bundle。
- 组合 ECS bundle 和 physics bundle。
- 给 simulation/spawning 层提供稳定的生成入口。

例如：

```rust
commands.spawn(PlayerPrefabBundle::default());
```

而不是在生成系统里散装很多组件。

## 当前结构

- `characters`: 角色类 prefab，例如玩家、敌人、NPC。

## 边界

- 可以依赖 `ecs` 和 `physics`。
- 不依赖 `render_2d` 或 `render_3d`。
- 不读取输入。
- 不写 ECS system 函数。
- 不负责状态流、关卡流程或生成时机。

`simulation` 决定什么时候生成，`prefab` 决定生成什么组合。
