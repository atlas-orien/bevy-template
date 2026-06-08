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
- 物理配置：写到 `crates/physics/src/config.rs`。
- 项目统一刚体语义：写到 `crates/physics/src/body.rs`。
- 项目统一碰撞体语义：写到 `crates/physics/src/collider.rs`。
- 项目统一碰撞层语义：写到 `crates/physics/src/layer.rs`。
- Avian 后端适配：写到 `crates/physics/src/backend/avian2d.rs`。
- Rapier 后端适配：写到 `crates/physics/src/backend/rapier2d.rs`。

## 边界规则

- `crates/app`、`crates/controller`、`crates/simulation`、`crates/render_2d`、`crates/render_3d` 不直接依赖 Avian 或 Rapier。
- 如果其它 crate 需要物理能力，先在 `crates/physics` 暴露统一 API。
- 不要把 Avian 或 Rapier 类型泄漏到 `physics` 的公共 API，除非这是经过明确设计的后端扩展点。
- 游戏语义数据仍然放在 `crates/ecs`。
- 物理引擎插件、物理后端配置、物理调试显示放在 `crates/physics`。

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
