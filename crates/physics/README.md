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
- `PhysicsMaterial`: 项目自己的物理材质语义。
- `PhysicsMass`: 项目自己的质量语义。
- `PhysicsVelocity2d`、`PhysicsAngularVelocity2d`: 项目自己的物理运动状态。
- `PhysicsForce2d`、`PhysicsImpulse2d`: 项目自己的力和冲量语义。
- `PhysicsCollisionStarted`、`PhysicsCollisionEnded`、`PhysicsSensorTriggered`: 项目自己的物理事件语义。

后端类型应该尽量收敛在本 crate 内部。

## 边界

- `app`、`external_runtime`、`intent`、`gameplay`、`render_2d`、`render_3d` 不直接依赖 Avian 或 Rapier。
- 如果需要新的物理能力，优先在 `physics` 暴露统一 API。
- `physics` 的公共 API 不 re-export Avian 或 Rapier 类型。
- 游戏语义数据仍然放在 `crates/ecs`。
- 物理引擎插件、刚体、碰撞体、传感器、调试显示放在 `physics`。

## 文件规则

- 每个基础物理语义必须先有子目录，再在子目录里写具体文件。
- 子目录的 `mod.rs` 只做模块导出和 re-export。
- 具体文件名不能和所在目录同名，避免 Rust module inception。
- `config/settings.rs`: 物理配置。
- `body/kind.rs`: 刚体语义。
- `collider/shape.rs`: 碰撞体形状。
- `layer/collision_layer.rs`: 碰撞层。
- `sensor/marker.rs`: 传感器标记。
- `material/surface.rs`: 物理材质。
- `mass/properties.rs`: 质量。
- `motion/velocity.rs`: 物理速度和角速度。
- `force/linear.rs`: 力和冲量。
- `events/collision.rs`: 物理事件语义。

hitbox、hurtbox、攻击范围、技能范围属于 gameplay 判定，不属于 physics 基础层。

## 后续扩展

当前模板先支持 2D 物理。

未来需要 3D 时，可以继续添加：

```text
avian3d
rapier3d
```
