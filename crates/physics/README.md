# physics

`physics` 是物理引擎适配层。

外部 crate 不需要知道内部使用 Avian 还是 Rapier，只使用这里暴露的统一 API。

## 后端选择

`physics` 默认使用 Avian 2D。

使用 Avian 2D：

```sh
cargo run
```

使用 Rapier 2D：

```sh
cargo run --features physics/rapier2d
```

默认情况下启用 Avian 2D；如果启用 `rapier2d`，运行时后端会切换到 Rapier 2D。

## 对外 API

当前对外暴露：

- `PhysicsPlugin`: 物理插件入口。
- `PhysicsDebugPlugin`: 物理调试显示插件入口。
- `PhysicsConfig`: 项目自己的物理配置数据。
- `PhysicsBody`: 项目自己的刚体语义。
- `PhysicsCollider`: 项目自己的碰撞体语义。
- `PhysicsSensor`: 项目自己的传感器标记。
- `PhysicsLayer`: 项目自己的碰撞层语义。

后端类型应该尽量收敛在本 crate 内部。

## 边界

- `app`、`input`、`intent`、`gameplay`、`render_2d`、`render_3d` 不直接依赖 Avian 或 Rapier。
- 如果需要新的物理能力，优先在 `physics` 暴露统一 API。
- `physics` 的公共 API 不 re-export Avian 或 Rapier 类型。
- 游戏语义数据仍然放在 `crates/ecs`。
- 物理引擎插件、刚体、碰撞体、传感器、调试显示放在 `physics`。

## 后续扩展

当前模板先支持 2D 物理。

未来需要 3D 时，可以继续添加：

```text
avian3d
rapier3d
```
