# physics

`physics` 是项目物理基础层。

外部 crate 不直接使用 Rapier，只使用这里暴露的统一 API。当前唯一后端是 `bevy_rapier`，并同时准备 2D / 3D。

## 对外 API

当前对外暴露：

- `PhysicsPlugin`: 物理插件入口。
- `PhysicsDebugPlugin`: 物理调试显示插件入口。
- `PhysicsConfig`: 项目自己的物理配置数据。
- `PhysicsRigidBody`: 项目自己的刚体语义。
- `PhysicsLockedAxes`、`PhysicsGravityScale`、`PhysicsDamping`、`PhysicsCcd`、`PhysicsSoftCcd`、`PhysicsSleeping`、`PhysicsRigidBodyDisabled`、`PhysicsAdditionalSolverIterations`: 项目自己的刚体控制语义。
- `PhysicsCollider2d`、`PhysicsCollider3d`: 项目自己的 2D / 3D 碰撞体语义。
- `PhysicsColliderDisabled`、`PhysicsContactSkin`、`PhysicsContactForceEventThreshold`: 项目自己的碰撞体控制语义。
- `PhysicsCollisionGroups`、`PhysicsSolverGroups`、`PhysicsActiveEvents`、`PhysicsActiveCollisionTypes`: 项目自己的碰撞体过滤和事件开关语义。
- `PhysicsSensor`: 项目自己的传感器标记。
- `PhysicsLayer`: 项目自己的碰撞层语义。
- `PhysicsMaterial`: 项目自己的物理材质语义。
- `PhysicsMass`: 项目自己的质量语义。
- `PhysicsVelocity2d`、`PhysicsAngularVelocity2d`: 项目自己的 2D 物理运动状态。
- `PhysicsVelocity3d`、`PhysicsAngularVelocity3d`: 项目自己的 3D 物理运动状态。
- `PhysicsForce2d`、`PhysicsImpulse2d`: 项目自己的 2D 力和冲量语义。
- `PhysicsForce3d`、`PhysicsImpulse3d`: 项目自己的 3D 力和冲量语义。
- `PhysicsCollisionStarted`、`PhysicsCollisionEnded`、`PhysicsSensorTriggered`: 项目自己的物理事件语义。

后端类型收敛在本 crate 内部。

## 连接方式

`PhysicsRigidBody`、`PhysicsCollider2d`、`PhysicsCollider3d` 这类类型不是 Rapier 的类型，而是项目自己的 facade component。

当 entity 添加或修改这些 facade component 时，`physics` 的 backend system 会把它们转换成 Rapier 真正使用的 component，并插入到同一个 entity 上。

Rapier 2D 第一版映射：

- `PhysicsRigidBody` -> Rapier2D `RigidBody`
- `PhysicsCollider2d::Circle` / `Rectangle` / `Polyline` / `ConvexPolygon` -> Rapier2D `Collider`
- `PhysicsSensor` -> Rapier2D `Sensor`
- `PhysicsMaterial` -> Rapier2D `Friction` + `Restitution`
- `PhysicsMass` -> Rapier2D `AdditionalMassProperties`
- `PhysicsVelocity2d` + `PhysicsAngularVelocity2d` -> Rapier2D `Velocity`
- `PhysicsForce2d` -> Rapier2D `ExternalForce`
- `PhysicsImpulse2d` -> Rapier2D `ExternalImpulse`

Rapier 3D 第一版映射：

- `PhysicsRigidBody` -> Rapier3D `RigidBody`
- `PhysicsCollider3d::Sphere` / `Cuboid` -> Rapier3D `Collider`
- `PhysicsSensor` -> Rapier3D `Sensor`
- `PhysicsMaterial` -> Rapier3D `Friction` + `Restitution`
- `PhysicsMass` -> Rapier3D `AdditionalMassProperties`
- `PhysicsVelocity3d` + `PhysicsAngularVelocity3d` -> Rapier3D `Velocity`
- `PhysicsForce3d` -> Rapier3D `ExternalForce`
- `PhysicsImpulse3d` -> Rapier3D `ExternalImpulse`

2D / 3D 归属由用户选择的 collider component 类型决定：`PhysicsCollider2d` 进入 Rapier 2D，`PhysicsCollider3d` 进入 Rapier 3D。

## 边界

- `app`、`external_runtime`、`intent`、`gameplay`、`render_2d`、`render_3d` 不直接依赖 `bevy_rapier2d` 或 `bevy_rapier3d`。
- 如果需要新的物理能力，优先在 `physics` 暴露统一 API。
- `physics` 的公共 API 不 re-export Rapier 类型。
- 游戏语义数据仍然放在 `crates/ecs`。
- 物理引擎插件、刚体、碰撞体、传感器、调试显示放在 `physics`。

## 文件规则

- 每个基础物理语义必须先有子目录，再在子目录里写具体文件。
- 子目录的 `mod.rs` 只做模块导出和 re-export。
- 具体文件名不能和所在目录同名，避免 Rust module inception。
- `config/settings.rs`: 物理配置。
- `body/kind.rs`: 刚体语义。
- `body/control.rs`: 刚体控制语义。
- `collider/shape.rs`: 碰撞体形状。
- `collider/control.rs`: 碰撞体控制语义。
- `collider/filter.rs`: 碰撞过滤和事件开关语义。
- `layer/collision_layer.rs`: 碰撞层。
- `sensor/marker.rs`: 传感器标记。
- `material/surface.rs`: 物理材质。
- `mass/properties.rs`: 质量。
- `motion/velocity.rs`: 物理速度和角速度。
- `force/linear.rs`: 力和冲量。
- `events/collision.rs`: 碰撞和传感器事件语义。
- `backend/rapier/mod.rs`: Rapier 总入口。
- `backend/rapier/dim2/*`: Rapier2D adapter。
- `backend/rapier/dim3/*`: Rapier3D adapter。

hitbox、hurtbox、攻击范围、技能范围属于 gameplay 判定，不属于 physics 基础层。
