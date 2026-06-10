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
- 项目统一碰撞体语义：写到 `crates/physics/src/collider`。
- 项目统一碰撞层语义：写到 `crates/physics/src/layer`。
- 项目统一物理材质语义：写到 `crates/physics/src/material`。
- 项目统一质量语义：写到 `crates/physics/src/mass`。
- 项目统一物理运动语义：写到 `crates/physics/src/motion`。
- 项目统一力和冲量语义：写到 `crates/physics/src/force`。
- 项目统一传感器标记：写到 `crates/physics/src/sensor`。
- 项目统一物理事件语义：写到 `crates/physics/src/events`。
- Rapier 总入口：写到 `crates/physics/src/backend/rapier`。
- Rapier 2D 适配：写到 `crates/physics/src/backend/rapier/dim2`。
- Rapier 3D 适配：写到 `crates/physics/src/backend/rapier/dim3`。

## 边界规则

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
- `body/kind.rs` 只定义刚体语义，例如 Dynamic、Static、Kinematic。
- `collider/shape.rs` 只定义碰撞体形状，不定义 sensor、material 或 hitbox。
- `layer/collision_layer.rs` 只定义物理碰撞层。
- `sensor/marker.rs` 只定义传感器标记。
- `material/surface.rs` 只定义物理材质，例如 friction、restitution。
- `mass/properties.rs` 只定义质量。
- `motion/velocity.rs` 只定义物理速度、角速度等运动状态。
- `force/linear.rs` 只定义力和冲量。
- `events/collision.rs` 只定义物理碰撞/传感器事件语义；当前 Bevy 版本使用 `Message` / `add_message`。
- 不要在 `crates/physics/src` 根目录直接新增物理语义文件；根目录只保留 `lib.rs`、`plugin.rs` 和 backend 入口。
- 2D / 3D 可以作为同一语义文件里的数据形状变体，例如 `PhysicsVelocity2d` 和 `PhysicsVelocity3d`。
- 如果某个概念是 gameplay 判定，例如 hitbox、hurtbox、攻击范围、技能范围，不放在 `physics`。

## Rapier 适配规则

- 项目自己的基础组件是 facade，例如 `PhysicsBody`、`PhysicsCollider`、`PhysicsMaterial`。
- Rapier 自己的组件只允许在 backend 目录内部使用，例如 Rapier 的 `RigidBody`、`Collider`、`Sensor`。
- `backend/rapier/mod.rs` 只负责注册 2D / 3D Rapier 子适配。
- `backend/rapier/dim2/mod.rs` 只负责注册 `bevy_rapier2d` 插件和 2D adapter systems。
- `backend/rapier/dim3/mod.rs` 只负责注册 `bevy_rapier3d` 插件和 3D adapter systems。
- `backend/rapier/dim*/convert.rs` 只负责把项目 facade 类型转换成 Rapier 类型。
- `backend/rapier/dim*/systems.rs` 只负责监听项目 facade component 的 `Added` / `Changed`，并向同一个 Bevy entity 插入 Rapier component。
- 不要在 prefab、gameplay、ecs 或 render crate 里直接插入 Rapier component。
- 第一版 Rapier adapter 覆盖 body、collider、sensor、material、mass、velocity；力、冲量和碰撞事件以后按明确语义再接入。
- `PhysicsCollider::Circle` 和 `PhysicsCollider::Rectangle` 属于 2D Rapier。
- `PhysicsCollider::Sphere` 和 `PhysicsCollider::Cuboid` 属于 3D Rapier。
- 2D / 3D 归属由 collider 形状决定，不由 `PhysicsBody` 决定。
- Rapier 的 rectangle / cuboid collider 使用半尺寸，转换逻辑必须留在 `convert.rs`。
- Rapier 的线速度和角速度共享同一个 `Velocity` component，更新其中一个时必须保留另一个。

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
