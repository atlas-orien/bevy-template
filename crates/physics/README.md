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
- `PhysicsCollider2d/PhysicsCollider3d`: 项目自己的碰撞体语义。
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
- `PhysicsCollisionStarted`、`PhysicsCollisionEnded`、`PhysicsSensorTriggered`: 项目自己的碰撞和传感器事件语义。
- `PhysicsContactForce2d`、`PhysicsContactForce3d`: 项目自己的 2D / 3D 接触力事件语义。
- `PhysicsQuery2d`、`PhysicsQuery3d`: 项目自己的 2D / 3D 物理查询入口。
- `PhysicsQueryFilter`: 项目自己的物理查询过滤条件。
- `PhysicsRayHit2d`、`PhysicsRayHit3d`: 项目自己的 raycast 命中结果。
- `PhysicsPointProjection2d`、`PhysicsPointProjection3d`: 项目自己的点投影命中结果。

后端类型收敛在本 crate 内部。

## 使用规则

`physics` 是基础能力 crate，不是普通游戏内容目录。普通用户通常不需要修改这里。

用户使用物理时，应该在 prefab 或 gameplay 代码里引用这里的类型：

```rust
use physics::{PhysicsCollider2d, PhysicsMaterial, PhysicsRigidBody};
```

然后把它们组合到具体对象上。比如角色、道具、地形、触发器这些具体内容，应该写在 `prefab` 或更上层的游戏逻辑里。

只有在扩展项目通用物理能力时才修改 `physics`，例如新增通用 collider 形状、joint、raycast、shapecast、collision event 映射、物理配置或 Rapier adapter。

## 连接方式

`PhysicsRigidBody` 这类类型不是 Rapier 的类型，而是项目自己的 facade component。

当 entity 添加或修改这些 facade component 时，`physics` 的 backend system 会把它们转换成 Rapier 真正使用的 component，并插入到同一个 entity 上。

Rapier 2D 第一版映射：

- `PhysicsRigidBody` -> Rapier2D `RigidBody`
- `PhysicsCollider2d::Circle` / `Rectangle` -> Rapier2D `Collider`
- `PhysicsCollider2d::Polyline` -> Rapier2D line-segment `Collider`
- `PhysicsCollider2d::ConvexPolygon` -> Rapier2D convex-hull `Collider`
- `PhysicsSensor` -> Rapier2D `Sensor`
- `PhysicsMaterial` -> Rapier2D `Friction` + `Restitution`
- `PhysicsMass` -> Rapier2D `AdditionalMassProperties`
- `PhysicsVelocity2d` + `PhysicsAngularVelocity2d` -> Rapier2D `Velocity`
- `PhysicsForce2d` -> Rapier2D `ExternalForce`
- `PhysicsImpulse2d` -> Rapier2D `ExternalImpulse`
- Rapier2D `CollisionEvent` -> `PhysicsCollisionStarted` / `PhysicsCollisionEnded` / `PhysicsSensorTriggered`
- Rapier2D `ContactForceEvent` -> `PhysicsContactForce2d`
- Rapier2D raycast query -> `PhysicsQuery2d::cast_ray` / `intersect_ray`
- Rapier2D point query -> `PhysicsQuery2d::intersect_point` / `project_point`

Rapier 3D 第一版映射：

- `PhysicsRigidBody` -> Rapier3D `RigidBody`
- `PhysicsCollider3d::Sphere` / `Cuboid` -> Rapier3D `Collider`
- `PhysicsSensor` -> Rapier3D `Sensor`
- `PhysicsMaterial` -> Rapier3D `Friction` + `Restitution`
- `PhysicsMass` -> Rapier3D `AdditionalMassProperties`
- `PhysicsVelocity3d` + `PhysicsAngularVelocity3d` -> Rapier3D `Velocity`
- `PhysicsForce3d` -> Rapier3D `ExternalForce`
- `PhysicsImpulse3d` -> Rapier3D `ExternalImpulse`
- Rapier3D `CollisionEvent` -> `PhysicsCollisionStarted` / `PhysicsCollisionEnded` / `PhysicsSensorTriggered`
- Rapier3D `ContactForceEvent` -> `PhysicsContactForce3d`
- Rapier3D raycast query -> `PhysicsQuery3d::cast_ray` / `intersect_ray`
- Rapier3D point query -> `PhysicsQuery3d::intersect_point` / `project_point`

2D / 3D 归属由用户选择的 collider component 类型决定：`PhysicsCollider2d` 进入 Rapier 2D，`PhysicsCollider3d` 进入 Rapier 3D。

`PhysicsCollider2d::Polyline` 是线段碰撞体，适合地形边缘、平台边缘、墙体轮廓。`PhysicsCollider2d::ConvexPolygon` 是凸多边形实体碰撞体，不表示任意凹多边形。

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
- `body/kind.rs`: 当前只放刚体语义，例如 `PhysicsRigidBody`。
- `collider/shape.rs`: 碰撞体形状。
- `layer/collision_layer.rs`: 碰撞层。
- `sensor/marker.rs`: 传感器标记。
- `material/surface.rs`: 物理材质。
- `mass/properties.rs`: 质量。
- `motion/velocity.rs`: 物理速度和角速度。
- `force/linear.rs`: 力和冲量。
- `events/collision.rs`: 碰撞和传感器事件语义。
- `events/contact_force.rs`: 接触力事件语义。
- `query/filter.rs`: 查询过滤条件。
- `query/raycast.rs`: raycast 命中结果。
- `query/point.rs`: point query 命中结果。
- `backend/rapier/mod.rs`: Rapier 总入口。
- `backend/rapier/dim2/mod.rs`: Rapier2D 插件和 adapter system 注册。
- `backend/rapier/dim2/convert.rs`: 项目 facade 类型到 Rapier2D 类型的转换。
- `backend/rapier/dim2/systems.rs`: 同步 facade component 到 Rapier2D backend component。
- `backend/rapier/dim2/events.rs`: Rapier2D message 到项目 physics message 的转发。
- `backend/rapier/dim2/query.rs`: 项目 2D physics query facade 到 Rapier2D query API 的桥接。
- `backend/rapier/dim3/mod.rs`: Rapier3D 插件和 adapter system 注册。
- `backend/rapier/dim3/convert.rs`: 项目 facade 类型到 Rapier3D 类型的转换。
- `backend/rapier/dim3/systems.rs`: 同步 facade component 到 Rapier3D backend component。
- `backend/rapier/dim3/events.rs`: Rapier3D message 到项目 physics message 的转发。
- `backend/rapier/dim3/query.rs`: 项目 3D physics query facade 到 Rapier3D query API 的桥接。

hitbox、hurtbox、攻击范围、技能范围属于 gameplay 判定，不属于 physics 基础层。
