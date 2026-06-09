# PHYSICS

这个文件是 `crates/physics` 的 AI 规则。

`crates/physics` 是物理引擎适配层。外部 crate 只使用这里暴露的统一 API，不直接感知内部使用 Avian 还是 Rapier。

## 后端选择

默认后端：

```text
avian2d
```

可选后端：

```text
rapier2d
```

默认情况下启用 Avian 2D；如果启用 `rapier2d`，运行时后端切换到 Rapier 2D。

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
- Avian 后端适配：写到 `crates/physics/src/backend/avian2d`。
- Rapier 后端适配：写到 `crates/physics/src/backend/rapier2d.rs`。

## 边界规则

- `crates/app`、`crates/external_runtime`、`crates/intent`、`crates/gameplay`、`crates/render_2d`、`crates/render_3d` 不直接依赖 Avian 或 Rapier。
- 如果其它 crate 需要物理能力，先在 `crates/physics` 暴露统一 API。
- 不要把 Avian 或 Rapier 类型泄漏到 `physics` 的公共 API，除非这是经过明确设计的后端扩展点。
- `crates/physics/src/lib.rs` 不允许 re-export Avian 或 Rapier 类型。
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
- 2D / 3D 可以作为同一语义文件里的数据形状变体，例如 `PhysicsVelocity2d`，不要因为一个类型就拆 crate。
- 如果某个概念是 gameplay 判定，例如 hitbox、hurtbox、攻击范围、技能范围，不放在 `physics`。

## 后端适配规则

- 项目自己的基础组件是 facade，例如 `PhysicsBody`、`PhysicsCollider`、`PhysicsMaterial`。
- 物理引擎自己的组件只允许在 backend 目录内部使用，例如 Avian 的 `RigidBody`、`Collider`、`Sensor`。
- `backend/avian2d/mod.rs` 只负责注册 Avian 插件和 adapter systems。
- `backend/avian2d/convert.rs` 只负责把项目 facade 类型转换成 Avian 类型。
- `backend/avian2d/systems.rs` 只负责监听项目 facade component 的 `Added` / `Changed`，并向同一个 Bevy entity 插入 Avian backend component。
- 不要在 prefab、gameplay、ecs 或 render crate 里直接插入 Avian / Rapier component。
- 第一版 Avian adapter 覆盖 body、collider、sensor、material、mass、velocity；力、冲量和碰撞事件以后按明确语义再接入。

## Cargo 规则

- 只有 `crates/physics/Cargo.toml` 可以依赖 `avian2d`、`avian3d`、`bevy_rapier2d`、`bevy_rapier3d`。
- 新增物理后端时，必须通过 feature 暴露。
- 如果默认后端和可选后端同时被 Cargo feature 启用，`crates/physics` 内部必须只选择一个运行时后端。
- 新增 3D 后端时，应添加 `avian3d` 或 `rapier3d` feature，并更新本文件。

## 验证要求

修改 `crates/physics` 后必须运行：

```sh
cargo check -p physics
cargo check -p physics --features rapier2d
cargo run -p xtask -- check
```
