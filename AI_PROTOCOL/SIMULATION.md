# SIMULATION

这个文件是 `crates/simulation` 的 AI 规则。

`crates/simulation` 是游戏流程和世界调度层。

## 核心职责

- 定义游戏状态流。
- 定义状态进入、运行、退出时的调度。
- 决定什么时候生成 prefab。
- 决定什么时候清理实体。
- 决定哪些 ECS system 在哪些状态或阶段运行。

## 代码落点

- 游戏状态：写到 `crates/simulation/src/state`。
- 系统调度、运行条件、系统集合：写到 `crates/simulation/src/schedule`。
- 生成时机：写到 `crates/simulation/src/spawning`。
- 清理策略：写到 `crates/simulation/src/cleanup`。
- 关卡、回合、场景生命周期：写到 `crates/simulation/src/lifecycle`。

当前旧目录可以逐步迁移，不需要保留旧名字。

## 边界规则

- 不定义 `Component`、`Bundle`、`Resource`、`Event`。
- 不写底层 ECS 规则函数；这些放到 `crates/ecs/src/systems`。
- 不封装物理后端；这些放到 `crates/physics`。
- 不读取输入；输入来源未来需要先转换成 `crates/intent` 表达的意图。
- 不写渲染、动画、UI、相机；这些放到渲染层。
- 不直接散装实体组件；生成实体时优先使用 `crates/prefab` 提供的 prefab bundle。

## 依赖规则

- `simulation` 可以依赖 `ecs`。
- `simulation` 可以依赖 `prefab`。
- `simulation` 必须依赖 `error`。
- `simulation` 不依赖 `render_2d` 或 `render_3d`。
- `simulation` 默认不直接依赖 `physics`；物理能力通过 `prefab` 组合，通过 `app` 注册 `PhysicsPlugin`。

## 验证要求

修改 `crates/simulation` 后必须运行：

```sh
cargo run -p xtask -- check
cargo check --workspace
```
