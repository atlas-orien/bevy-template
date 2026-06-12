此文件是项目约束来源。AI 不得为通过检查而修改本文件；规则变更必须由人发起。

# PHYSICS

这个文件是 `crates/physics` 的 AI 规则。

`crates/physics` 是项目物理基础层。外部 crate 只使用这里暴露的统一 API，不直接感知 `bevy_rapier2d` 或 `bevy_rapier3d`。

## 后端选择

当前唯一物理后端：

```text
bevy_rapier
```

本项目不再提供 Avian 后端，也不通过 Cargo feature 切换物理后端。

2D 和 3D 都需要支持：

```text
bevy_rapier2d
bevy_rapier3d
```

## 代码落点

- 物理插件入口：写到 `crates/physics/src/plugin.rs`。
- 物理配置：写到 `crates/physics/src/config`。
- 项目统一刚体语义：写到 `crates/physics/src/body`。
- 项目统一角色控制器语义：写到 `crates/physics/src/controller`。
- 项目统一碰撞体语义：写到 `crates/physics/src/collider`。
- 项目统一碰撞层语义：写到 `crates/physics/src/layer`。
- 项目统一物理材质语义：写到 `crates/physics/src/material`。
- 项目统一质量语义：写到 `crates/physics/src/mass`。
- 项目统一物理运动语义：写到 `crates/physics/src/motion`。
- 项目统一力和冲量语义：写到 `crates/physics/src/force`。
- 项目统一关节语义：写到 `crates/physics/src/joint`。
- 项目统一传感器标记：写到 `crates/physics/src/sensor`。
- 项目统一物理事件语义：写到 `crates/physics/src/events`。
- 项目统一物理查询语义：写到 `crates/physics/src/query`。
- Rapier 总入口：写到 `crates/physics/src/backend/rapier`。
- Rapier 2D 适配：写到 `crates/physics/src/backend/rapier/dim2`。
- Rapier 3D 适配：写到 `crates/physics/src/backend/rapier/dim3`。

## 边界规则

- `crates/physics` 是基础能力层，不是普通游戏内容层。
- 普通用户默认不修改 `crates/physics`；用户在 prefab、gameplay 或 ecs system 里引用 `physics` 提供的类型来组合对象和规则。
- 只有在明确扩展项目通用物理能力时才修改 `crates/physics`，例如新增通用 collider 形状、joint、raycast、shapecast、collision event 映射、物理配置或 Rapier adapter。
- 如果只是某个具体角色、道具、场景物体需要物理属性，写到 `crates/prefab`，不要修改 `crates/physics`。
- 如果只是某个游戏规则影响速度、移动、受力或状态，优先写到 `crates/ecs`、`crates/gameplay` 或后续明确的规则 crate，不要修改 `crates/physics`。
- `crates/app`、`crates/external_runtime`、`crates/intent`、`crates/gameplay`、`crates/render_2d`、`crates/render_3d` 不直接依赖 `bevy_rapier2d` 或 `bevy_rapier3d`。
- 如果其它 crate 需要物理能力，先在 `crates/physics` 暴露统一 API。
- 不要把 Rapier 类型泄漏到 `physics` 的公共 API，除非这是经过明确设计的后端扩展点。
- `crates/physics/src/lib.rs` 不允许 re-export Rapier 类型。
- 游戏语义数据仍然放在 `crates/ecs`。
- 物理引擎插件、物理后端配置、物理调试显示放在 `crates/physics`。

## 基础物理语义规则

- 每个基础物理语义必须先有子目录，再在子目录里写具体文件。
- 子目录的 `mod.rs` 只做模块导出和 re-export，不堆具体类型。
- 具体文件名不能和所在目录同名，避免 Rust module inception。
- `config/settings.rs` 只定义物理配置。
- `body/kind.rs` 当前只定义刚体语义，例如 `PhysicsRigidBody::Dynamic`、`Static`、`Kinematic`。
- `body/control.rs` 只定义刚体控制语义，例如 locked axes、gravity scale、damping、CCD、sleeping、disabled、solver iterations。
- `body` 目录可以作为物理主体分类目录；但第一版 Rapier 后端只承诺 rigid body。
- `controller/character.rs` 只定义项目自己的 kinematic character controller facade 和 output，不定义具体游戏移动规则。
- 不要提前添加 soft body、fluid body、particle body 等 facade 类型，除非后端能力和项目用法已经明确。
- `collider/shape.rs` 只定义碰撞体形状，不定义 sensor、material 或 hitbox。
- `collider/control.rs` 只定义碰撞体控制语义，例如 disabled、contact skin、contact force threshold。
- `collider/filter.rs` 只定义碰撞体过滤和事件开关语义，例如 collision groups、solver groups、active events、active collision types。
- `PhysicsCollider2d::Polyline` 表示用户自定义线段碰撞体，不表示有面积的多边形。
- `PhysicsCollider2d::ConvexPolygon` 表示用户自定义凸多边形碰撞体；不要用 `Polygon` 命名承诺任意凹多边形。
- `layer/collision_layer.rs` 只定义物理碰撞层。
- `sensor/marker.rs` 只定义传感器标记。
- `material/surface.rs` 只定义物理材质，例如 friction、restitution。
- `mass/properties.rs` 只定义质量。
- `motion/velocity.rs` 只定义物理速度、角速度等运动状态。
- `force/linear.rs` 只定义力和冲量。
- `joint/impulse.rs` 只定义项目自己的 impulse joint facade，不暴露 Rapier `ImpulseJoint`、`TypedJoint` 或具体 Rapier joint 类型。
- `events/collision.rs` 只定义物理碰撞/传感器事件语义；当前 Bevy 版本使用 `Message` / `add_message`。
- `events/contact_force.rs` 只定义物理接触力事件语义。因为力向量维度不同，接触力事件必须显式区分 `PhysicsContactForce2d` 和 `PhysicsContactForce3d`。
- `query/filter.rs` 只定义项目自己的查询过滤条件，不暴露 Rapier `QueryFilter`。
- `query/raycast.rs` 只定义项目自己的 raycast 返回数据，不暴露 Rapier `RayIntersection`。
- `query/point.rs` 只定义项目自己的 point query 返回数据，不暴露 Rapier `PointProjection`。
- `query/shape.rs` 只定义项目自己的 shape query / shapecast 返回数据，不暴露 Rapier `ShapeCastHit`。
- 不要在 `crates/physics/src` 根目录直接新增物理语义文件；根目录只保留 `lib.rs`、`plugin.rs` 和 backend 入口。
- 2D / 3D 可以作为同一语义文件里的数据形状变体，例如 `PhysicsVelocity2d` 和 `PhysicsVelocity3d`。
- 如果某个概念是 gameplay 判定，例如 hitbox、hurtbox、攻击范围、技能范围，不放在 `physics`。

## Rapier 适配规则

- 项目自己的基础组件是 facade，例如 `PhysicsRigidBody`、`PhysicsCollider2d/PhysicsCollider3d`、`PhysicsMaterial`。
- Rapier 自己的组件只允许在 backend 目录内部使用，例如 Rapier 的 `RigidBody`、`Collider`、`Sensor`。
- `backend/rapier/mod.rs` 只负责注册 2D / 3D Rapier 子适配。
- `backend/rapier/dim2/mod.rs` 只负责注册 `bevy_rapier2d` 插件和 2D adapter systems。
- `backend/rapier/dim3/mod.rs` 只负责注册 `bevy_rapier3d` 插件和 3D adapter systems。
- `backend/rapier/dim*/convert.rs` 只负责把项目 facade 类型转换成 Rapier 类型。
- `backend/rapier/dim*/systems.rs` 只负责监听项目 facade component 的 `Added` / `Changed`，并向同一个 Bevy entity 插入 Rapier component。
- `backend/rapier/dim*/events.rs` 只负责把 Rapier message 转发成项目自己的 physics message。
- `backend/rapier/dim*/query.rs` 只负责实现项目 physics query facade 对 Rapier query API 的调用。
- 不要在 prefab、gameplay、ecs 或 render crate 里直接插入 Rapier component。
- 第一版 Rapier adapter 覆盖 rigid_body、rigid body control、collider、collider control、collider filtering、sensor、material、mass、velocity、force、impulse。
- 第二阶段 Rapier adapter 覆盖 collision started、collision ended、sensor triggered、2D / 3D contact force event 转发，以及 2D / 3D raycast、point query、shape query、shapecast。
- 第二阶段 Rapier adapter 覆盖第一版 impulse joint facade：fixed、revolute、prismatic、rope、spring。
- 第二阶段 Rapier adapter 覆盖第一版 kinematic character controller facade 和 output 映射。
- `PhysicsCollider2d::Circle`、`Rectangle`、`Polyline`、`ConvexPolygon` 属于 2D Rapier。
- `PhysicsCollider3d::Sphere` 和 `PhysicsCollider3d::Cuboid` 属于 3D Rapier。
- 2D / 3D 归属由用户选择的 collider component 类型决定：`PhysicsCollider2d` 进入 Rapier 2D，`PhysicsCollider3d` 进入 Rapier 3D。
- 维度相关 facade 类型必须显式带 `2d` 或 `3d` 后缀；维度无关类型才使用通用命名。
- Rapier 的 rectangle / cuboid collider 使用半尺寸，转换逻辑必须留在 `convert.rs`。
- Rapier 的 `PhysicsCollider2d::ConvexPolygon` 使用 `Collider::convex_hull`，转换可能失败；失败时不要插入 Rapier collider。
- Rapier 的 `PhysicsCollider2d::Polyline` 使用 `Collider::polyline(points, None)`，用于地形边缘、平台边缘、墙体轮廓等线段碰撞。
- Rapier 的线速度和角速度共享同一个 `Velocity` component，更新其中一个时必须保留另一个。
- `PhysicsForce2d/3d` 映射到 Rapier `ExternalForce`，第一版只表达作用在质心的线性力，torque 默认为 0。
- `PhysicsImpulse2d/3d` 映射到 Rapier `ExternalImpulse`，第一版只表达作用在质心的线性冲量，torque impulse 默认为 0。
- `PhysicsImpulseJoint2d/3d` 映射到 Rapier `ImpulseJoint`，第一版只表达 fixed、revolute、prismatic、rope、spring。
- `PhysicsImpulseJoint2d/3d::parent` 是 joint 的第一个刚体 entity，挂载该 component 的 entity 是第二个刚体。
- 第一版 joint 不做 multibody joint，不做 motor facade，不做 limit facade。未来需要时先更新本文件和 README。
- `PhysicsCharacterController2d/3d` 映射到 Rapier `KinematicCharacterController`。
- `PhysicsCharacterControllerOutput2d/3d` 来自 Rapier controller output，但不暴露 Rapier output 类型。
- 第一版 character controller 只表达 translation、up、offset、slide、slope、snap、dynamic impulse、filter groups、exclude sensors。
- 第一版 character controller 不做 autostep、custom shape、custom mass；具体玩家移动输入和状态机仍然放在 gameplay / ecs。
- `PhysicsCollisionStarted` / `PhysicsCollisionEnded` 来自 Rapier collision event，但不暴露 Rapier event 类型。
- `PhysicsSensorTriggered` 来自 Rapier sensor collision started event；第一版只表达进入/触发，不表达退出。
- `PhysicsContactForce2d/3d` 来自 Rapier contact force event；用户需要在 collider 上启用 `PhysicsActiveEvents` 和 `PhysicsContactForceEventThreshold` 才能收到。
- `PhysicsQuery2d/3d` 是供 Bevy system 使用的 physics query facade。它可以在内部持有 Rapier context，但公共方法和返回值必须使用项目自己的类型。
- `PhysicsQueryFilter` 是项目查询过滤条件，第一版覆盖 exclude sensors、exclude solids、collision groups、exclude collider、exclude rigid body。
- `PhysicsQuery2d/3d::cast_ray` 返回最近的 raycast 命中。
- `PhysicsQuery2d/3d::intersect_ray` 返回 raycast 路径上的所有命中。
- `PhysicsQuery2d/3d::intersect_point` 返回包含某个点的所有 collider entity。
- `PhysicsQuery2d/3d::project_point` 返回某个点投影到最近 collider 上的结果。
- `PhysicsQuery2d/3d::intersect_shape` 返回与查询形状相交的所有 collider entity。
- `PhysicsQuery2d/3d::cast_shape` 返回移动查询形状时最近的 shapecast 命中。
- shape query / shapecast 的查询形状复用 `PhysicsCollider2d/3d`，不要新增重复的 query shape enum。
- `PhysicsRayHit2d/3d` 是项目 raycast 命中结果，第一版包含命中 entity、time of impact、point、normal。
- `PhysicsPointProjection2d/3d` 是项目 point projection 结果，第一版包含命中 entity、投影点、是否在 collider 内部。
- `PhysicsShapeCastHit2d/3d` 是项目 shapecast 命中结果，第一版包含命中 entity、time of impact、可选 witness / normal 细节。
- 不要在公共 API 中暴露 Rapier `QueryFilter`、`RayIntersection`、`PointProjection`、`ShapeCastHit`、`ShapeCastStatus`、`RapierContext`、`ReadRapierContext`。

## Cargo 规则

- 只有 `crates/physics/Cargo.toml` 可以依赖 `bevy_rapier2d`、`bevy_rapier3d`。
- 不要新增 Avian 依赖。
- 不要为物理后端新增 Cargo feature。
- 如果未来确实需要替换物理后端，必须先更新本文件和 `xtask` 规则。

## 验证要求

修改 `crates/physics` 后必须运行：

```sh
cargo check -p physics
cargo run -p xtask -- check
```
